use anyhow::{anyhow, Result};
use std::ops::Add;
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

#[derive(PartialEq, Debug)]
pub enum CurrencyCode {
    EUR,
}
const DEFAULT_CURRENCY_CODE: CurrencyCode = CurrencyCode::EUR;

#[derive(PartialEq, Debug)]
pub struct Money {
    pub amount: i64,
    pub currency_code: CurrencyCode,
}

impl Money {
    pub fn new(amount: i64, currency: CurrencyCode) -> Self {
        Self {
            amount,
            currency_code: currency,
        }
    }
    pub fn from_decimal(amount: i64, currency: Option<CurrencyCode>) -> Self {
        Money::new(amount, currency.unwrap_or(DEFAULT_CURRENCY_CODE))
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
    pub fn new(amount: i64) -> Result<Self> {
        if amount < 0 {
            return Err(anyhow!("Price cannot be negative"));
        }
        Ok(Self(Money::from_decimal(amount, None)))
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

pub struct ClassifiedAd {
    _owner_id: UserId,
    _text: Option<String>,
    _title: Option<String>,
    _price: Option<i64>,

    pub uuid: ClassifiedAdId,
}
impl ClassifiedAd {
    pub fn new(id: ClassifiedAdId, owner_id: UserId) -> Self {
        Self {
            uuid: id,
            _owner_id: owner_id,
            _text: None,
            _title: None,
            _price: None,
        }
    }

    /// Set the classified ad's  price.
    pub fn update_price(&mut self, price: i64) {
        self._price = Some(price);
    }

    /// Set the classified ad's  text.
    pub fn update_text(&mut self, text: String) {
        self._text = Some(text);
    }

    /// Set the classified ad's  title.
    pub fn update_title(&mut self, title: String) {
        self._title = Some(title);
    }
}

#[cfg(test)]
mod tests {
    use crate::{CurrencyCode, Money};

    #[test]
    fn money_with_same_amount_should_be_equal() {
        let first_amount = Money::from_decimal(5, None);
        let second_amount = Money::from_decimal(5, None);
        assert_eq!(first_amount, second_amount);
    }

    #[test]
    fn sum_of_money_gives_full_amount() {
        let coin1 = Money::from_decimal(1, None);
        let coin2 = Money::from_decimal(2, None);
        let coin3 = Money::from_decimal(2, None);

        let banknote = Money::from_decimal(5, None);

        assert_eq!(banknote, (coin1 + coin2 + coin3).unwrap());
    }
}
