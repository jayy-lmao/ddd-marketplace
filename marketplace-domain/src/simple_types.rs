use crate::{ports::*, FakeCurrencyLookup};
use anyhow::{anyhow, Result};
use math::round;
use std::ops::{Add, Sub};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserId {
    _value: Uuid,
}

impl UserId {
    pub fn new(value: Uuid) -> Self {
        Self { _value: value }
    }

    /// Get a reference to the user id's  value.
    pub fn value(&self) -> Uuid {
        self._value
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Copy)]
pub struct ClassifiedAdId {
    _value: Uuid,
}

impl ClassifiedAdId {
    pub fn new(value: Uuid) -> Self {
        Self { _value: value }
    }

    /// Get a reference to the classified ad id's  value.
    pub fn value(&self) -> Uuid {
        self._value
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum CurrencyCode {
    EUR,
    AUD,
}
const DEFAULT_CURRENCY_CODE: CurrencyCode = CurrencyCode::EUR;

#[derive(PartialEq, Debug, Clone)]
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
            _ => DEFAULT_CURRENCY_CODE,
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

#[derive(PartialEq, Clone)]
pub struct Price {
    pub money: Money,
}

impl Price {
    pub fn is_zero(&self) -> bool {
        self.money.amount == 0.
    }
    pub fn from_decimal(
        amount: f64,
        currency: Option<CurrencyCode>,
        lookup: impl ICurrencyLookup,
    ) -> Result<Self> {
        if amount < 0. {
            return Err(anyhow!("Price cannot be negative"));
        }
        Ok(Self {
            money: Money::from_decimal(amount, currency, lookup)?,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
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
#[derive(Debug, PartialEq, Clone)]
pub struct ClassifiedAdText {
    _value: String,
}

impl ClassifiedAdText {
    pub fn new(text: String) -> Self {
        Self { _value: text }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ClassifiedAdState {
    PendingReview,
    Active,
    InActive,
    MarkedAsSold,
}

#[derive(Clone)]
pub struct CurrencyDetails {
    pub currency_code: CurrencyCode,
    pub in_use: bool,
    pub decimal_places: i8,
}
