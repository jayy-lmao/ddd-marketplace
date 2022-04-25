use anyhow::{anyhow, Result};
use events::*;
use marketplace_framework::Entity;

pub mod events;
pub mod ports;
pub mod simple_types;

pub use ports::*;
pub use simple_types::*;

pub struct ClassifiedAd {
    _owner_id: UserId,
    _approved_by: Option<UserId>,
    _text: Option<String>,
    _title: Option<String>,
    _price: Option<Price>,
    _state: ClassifiedAdState,
    _entity: Entity<ClassifiedAdEvents>,

    pub uuid: ClassifiedAdId,
}
impl ClassifiedAd {
    pub fn new(id: ClassifiedAdId, owner_id: UserId) -> Self {
        let mut entity = Entity::new();
        let created_event = ClassifiedAdCreated {
            id: id.clone().value(),
            owner_id: owner_id.clone().value(),
        };
        entity.raise(created_event.into());

        Self {
            uuid: id,
            _owner_id: owner_id,
            _approved_by: None,
            _text: None,
            _title: None,
            _price: None,
            _state: ClassifiedAdState::InActive,
            _entity: entity,
        }
    }

    fn ensure_valid_state(&self) -> Result<()> {
        let valid = match self._state {
            ClassifiedAdState::PendingReview => {
                self._title.is_some()
                    && self._text.is_some()
                    && self._price.is_some()
                    && !self._price.clone().unwrap().is_zero()
            }
            ClassifiedAdState::Active => {
                self._title.is_some()
                    && self._text.is_some()
                    && self._price.is_some()
                    && !self._price.clone().unwrap().is_zero()
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
    pub fn update_price(&mut self, price: Price) -> Result<()> {
        self._price = Some(price.clone());
        self.ensure_valid_state()?;
        let event = ClassifiedAdPriceUpdated {
            id: self.uuid.value(),
            price: price.money.amount,
        };

        self._entity.raise(event.into());
        Ok(())
    }

    /// Set the classified ad's  text.
    pub fn set_text(&mut self, text: String) -> Result<()> {
        self._text = Some(text.clone());
        self.ensure_valid_state()?;

        let text_event = ClassifiedAdTextUpdated {
            id: self.uuid.value(),
            ad_text: text,
        };
        self._entity.raise(text_event.into());

        Ok(())
    }

    /// Set the classified ad's  title.
    pub fn set_title(&mut self, title: String) -> Result<()> {
        self._title = Some(title.clone());
        self.ensure_valid_state()?;
        let title_event = ClassifiedAdTitleChanged {
            id: self.uuid.value(),
            title: title,
        };
        self._entity.raise(title_event.into());

        Ok(())
    }

    pub fn request_to_publish(&mut self) -> Result<()> {
        if self._title == None {
            return Err(anyhow!("Title cannot be empty"));
        }
        if self._text == None {
            return Err(anyhow!("Text cannot be empty"));
        }
        let invalid_price = match &self._price {
            Some(p) => p.is_zero(),
            None => true,
        };
        if invalid_price {
            return Err(anyhow!("Price cannot be 0"));
        }
        self._state = ClassifiedAdState::PendingReview;
        self.ensure_valid_state()?;
        let event = ClassifiedAdSentForReview {
            id: self.uuid.value(),
        };
        self._entity.raise(event.into());
        Ok(())
    }

    /// Get a reference to the classified ad's  state.
    pub fn state(&self) -> ClassifiedAdState {
        self._state.clone()
    }
}

#[cfg(test)]
mod tests {

    use uuid::Uuid;

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

    #[test]
    fn can_publish_an_ad() -> Result<()> {
        let mut classified_ad = ClassifiedAd::new(
            ClassifiedAdId::new(Uuid::new_v4()),
            UserId::new(Uuid::new_v4()),
        );

        classified_ad.set_title("Test ad".into())?;
        classified_ad.set_text("Please buy my stuff".into())?;
        let price = Price::from_decimal(100., FakeCurrencyLookup)?;
        classified_ad.update_price(price)?;
        classified_ad.request_to_publish()?;

        // assert!(matches!(
        //     classified_ad.state(),
        //     ClassifiedAdState::PendingReview
        // ));
        Ok(())
    }
}
