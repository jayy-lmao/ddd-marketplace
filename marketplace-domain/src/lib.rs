use anyhow::{anyhow, Result};
use math::round;
use std::ops::{Add, Sub};
use uuid::Uuid;

pub struct UserId {
    _value: Uuid,
}

impl UserId {
    pub fn new(value: Uuid) -> Self {
        Self { _value: value }
    }
}
pub struct ClassifiedAdId {
    _value: Uuid,
}

impl ClassifiedAdId {
    pub fn new(value: Uuid) -> Self {
        Self { _value: value }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum CurrencyCode {
    EUR,
    AUD,
}
const DEFAULT_CURRENCY_CODE: CurrencyCode = CurrencyCode::EUR;

#[derive(PartialEq, Debug)]
pub struct Money {
    pub amount: f64,
    pub currency_code: CurrencyCode,
}

impl Money {
    pub fn new(amount: f64, currency: CurrencyCode) -> Self {
        Self {
            amount,
            currency_code: currency,
        }
    }
    pub fn from_decimal(
        amount: f64,
        currency_code: Option<CurrencyCode>,
        currency_lookup: impl ICurrencyLookup,
    ) -> Result<Self> {
        let currency_code = match currency_code {
            Some(code) => {
                let currency = currency_lookup.find_currency(code.clone())?;
                if !currency.in_use {
                    return Err(anyhow!("Currency code {:?} is not valid", code));
                }
                let rounded = round::half_towards_zero(amount, currency.decimal_places);
                if rounded != amount {
                    return Err(anyhow!(
                        "Amount in {:?} cannot have more than {} decimals",
                        currency.currency_code,
                        currency.decimal_places
                    ));
                }
                currency.currency_code
            }
            None => DEFAULT_CURRENCY_CODE,
        };

        Ok(Money::new(amount, currency_code))
    }
}

impl Add for Money {
    type Output = Result<Money>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.currency_code != rhs.currency_code {
            return Err(anyhow!("Not same currency code"));
        }
        Ok(Money::new(self.amount + rhs.amount, self.currency_code))
    }
}
impl Sub for Money {
    type Output = Result<Money>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.currency_code != rhs.currency_code {
            return Err(anyhow!("Not same currency code"));
        }
        Ok(Money::new(self.amount - rhs.amount, self.currency_code))
    }
}

impl Add<Result<Money>> for Money {
    type Output = Result<Money>;

    fn add(self, rhs: Result<Money>) -> Self::Output {
        Ok(Money::new(self.amount + (rhs?.amount), self.currency_code))
    }
}
impl Add<Money> for Result<Money> {
    type Output = Result<Money>;

    fn add(self, rhs: Money) -> Self::Output {
        Ok(Money::new(self?.amount + rhs.amount, rhs.currency_code))
    }
}

pub struct Price(pub Money);

impl Price {
    pub fn new(amount: f64, currency_lookup: impl ICurrencyLookup) -> Result<Self> {
        if amount < 0. {
            return Err(anyhow!("Price cannot be negative"));
        }
        Ok(Self(Money::from_decimal(amount, None, currency_lookup)?))
    }
}

pub struct ClassifiedAdTitle {
    _value: String,
}

impl ClassifiedAdTitle {
    pub fn new(title: String) -> Result<Self> {
        if title.len() > 100 {
            return Err(anyhow!("Title cannot be longer than 100 characters"));
        }
        Ok(Self { _value: title })
    }
}

#[derive(Debug)]
pub enum ClassifiedAdState {
    PendingReview,
    Active,
    InActive,
    MarkedAsSold,
}

pub struct ClassifiedAd {
    _owner_id: UserId,
    _approved_by: Option<UserId>,
    _text: Option<String>,
    _title: Option<String>,
    _price: Option<f64>,
    _state: ClassifiedAdState,

    pub uuid: ClassifiedAdId,
}
impl ClassifiedAd {
    pub fn new(id: ClassifiedAdId, owner_id: UserId) -> Self {
        Self {
            uuid: id,
            _owner_id: owner_id,
            _approved_by: None,
            _text: None,
            _title: None,
            _price: None,
            _state: ClassifiedAdState::InActive,
        }
    }

    fn ensure_valid_state(&self) -> Result<()> {
        let valid = match self._state {
            ClassifiedAdState::PendingReview => {
                self._title.is_some() && self._text.is_some() && self._price != Some(0.)
            }
            ClassifiedAdState::Active => {
                self._title.is_some()
                    && self._text.is_some()
                    && self._price != Some(0.)
                    && self._approved_by.is_some()
            }
            _ => true,
        };
        if !valid {
            return Err(anyhow!("Post-checks failed in state {:?}", self._state));
        }
        Ok(())
    }

    /// Set the classified ad's  price.
    pub fn update_price(&mut self, price: f64) -> Result<()>{
        self._price = Some(price);
        self.ensure_valid_state()
    }

    /// Set the classified ad's  text.
    pub fn update_text(&mut self, text: String) -> Result<()> {
        self._text = Some(text);
        self.ensure_valid_state()
    }

    /// Set the classified ad's  title.
    pub fn update_title(&mut self, title: String) -> Result<()> {
        self._title = Some(title);
        self.ensure_valid_state()
    }

    pub fn request_to_publish(&mut self) -> Result<()> {
        if self._title == None {
            return Err(anyhow!("Title cannot be empty"));
        }
        if self._text == None {
            return Err(anyhow!("Text cannot be empty"));
        }
        if self._price == Some(0.) {
            return Err(anyhow!("Price cannot be 0"));
        }
        self._state = ClassifiedAdState::PendingReview;
        self.ensure_valid_state()
    }
}

#[derive(Clone)]
pub struct CurrencyDetails {
    pub currency_code: CurrencyCode,
    pub in_use: bool,
    pub decimal_places: i8,
}

pub trait ICurrencyLookup {
    fn find_currency(&self, currency_code: CurrencyCode) -> Result<CurrencyDetails>;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mocks
    pub struct FakeCurrencyLookup;

    const CURRENCIES: &'static [CurrencyDetails] = &[CurrencyDetails {
        currency_code: CurrencyCode::EUR,
        in_use: true,
        decimal_places: 2,
    }];

    impl ICurrencyLookup for FakeCurrencyLookup {
        fn find_currency(&self, currency_code: CurrencyCode) -> Result<CurrencyDetails> {
            let currency = CURRENCIES
                .iter()
                .find(|&c| c.currency_code == currency_code)
                .map(|c| c.clone());

            match currency {
                Some(currency_details) => Ok(currency_details.clone()),
                None => Err(anyhow!(
                    "Could not find currency with code {:?}",
                    currency_code
                )),
            }
        }
    }

    // Tests

    #[test]
    fn money_with_same_amount_should_be_equal() -> Result<()> {
        let first_amount = Money::from_decimal(5., None, FakeCurrencyLookup)?;
        let second_amount = Money::from_decimal(5., None, FakeCurrencyLookup)?;
        assert_eq!(first_amount, second_amount);
        Ok(())
    }

    #[test]
    fn sum_of_money_gives_full_amount() -> Result<()> {
        let coin1 = Money::from_decimal(1., None, FakeCurrencyLookup)?;
        let coin2 = Money::from_decimal(2., None, FakeCurrencyLookup)?;
        let coin3 = Money::from_decimal(2., None, FakeCurrencyLookup)?;

        let banknote = Money::from_decimal(5., None, FakeCurrencyLookup)?;

        assert_eq!(banknote, (coin1 + coin2 + coin3).unwrap());
        Ok(())
    }
    #[test]
    fn subtracting_money() -> Result<()> {
        let coin1 = Money::from_decimal(4., None, FakeCurrencyLookup)?;
        let coin2 = Money::from_decimal(3., None, FakeCurrencyLookup)?;
        let coin3 = Money::from_decimal(1., None, FakeCurrencyLookup)?;

        assert_eq!((coin1 - coin2).unwrap(), coin3);
        Ok(())
    }
}
