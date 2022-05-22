use anyhow::{anyhow, Result};
use classified_ad_events::*;
// use marketplace_framework::{Entity, IAggregateRoot};

pub mod classified_ad;
pub mod classified_ad_events;
pub mod ports;
pub mod simple_types;
pub mod user_profile;

pub use ports::*;
pub use simple_types::*;

// #[derive(Clone)]
// pub struct ClassifiedAd {
//     _owner_id: UserId,
//     _approved_by: Option<UserId>,
//     _text: Option<ClassifiedAdText>,
//     _title: Option<ClassifiedAdTitle>,
//     _price: Option<Price>,
//     _state: ClassifiedAdState,
//     _entity: Entity<ClassifiedAdEvents>,

//     pub uuid: ClassifiedAdId,
// }
// impl ClassifiedAd {
//     pub fn new(id: ClassifiedAdId, owner_id: UserId) -> Self {
//         let mut entity = Entity::new();
//         let created_event = ClassifiedAdCreated {
//             id: id.clone().value(),
//             owner_id: owner_id.value(),
//         };
//         entity.raise(created_event.into());

//         Self {
//             uuid: id,
//             _owner_id: owner_id,
//             _approved_by: None,
//             _text: None,
//             _title: None,
//             _price: None,
//             _state: ClassifiedAdState::InActive,
//             _entity: entity,
//         }
//     }

//     fn when(&mut self, event: events::ClassifiedAdEvents) -> Result<()> {
//         match event {
//             ClassifiedAdEvents::Created(e) => {
//                 self._owner_id = UserId::new(e.owner_id);
//                 self.uuid = ClassifiedAdId::new(e.id);
//                 self._state = ClassifiedAdState::InActive;
//             }
//             ClassifiedAdEvents::TextUpdated(e) => {
//                 self._text = Some(ClassifiedAdText::new(e.ad_text))
//             }
//             ClassifiedAdEvents::TitleChanged(e) => {
//                 self._title = Some(ClassifiedAdTitle::new(e.title)?)
//             }
//             ClassifiedAdEvents::PriceUpdated(e) => {
//                 self._price = Some(Price::from_decimal(e.price, None, FakeCurrencyLookup)?)
//             }
//             ClassifiedAdEvents::SentForReview(_e) => self._state = ClassifiedAdState::PendingReview,
//         }
//         self.ensure_valid_state()
//     }

//     fn ensure_valid_state(&self) -> Result<()> {
//         let valid = match self._state {
//             ClassifiedAdState::PendingReview => {
//                 self._title.is_some()
//                     && self._text.is_some()
//                     && self._price.is_some()
//                     && !self._price.ok_or(anyhow!("No price"))?.is_zero()
//             }
//             ClassifiedAdState::Active => {
//                 self._title.is_some()
//                     && self._text.is_some()
//                     && self._price.is_some()
//                     && !self._price.ok_or(anyhow!("No price"))?.is_zero()
//                     && self._approved_by.is_some()
//             }
//             _ => true,
//         };
//         if !valid {
//             return Err(anyhow!("Post-checks failed in state {:?}", self._state));
//         }
//         Ok(())
//     }

//     /// Set the classified ad's  price.
//     pub fn update_price(&mut self, price: Price) -> Result<()> {
//         let event = ClassifiedAdPriceUpdated {
//             id: self.uuid.value(),
//             price: price.money.amount,
//         };

//         self._entity.raise(event.clone().into());
//         self.when(event.into())?;

//         Ok(())
//     }

//     /// Set the classified ad's  text.
//     pub fn set_text(&mut self, text: ClassifiedAdText) -> Result<()> {
//         let event = ClassifiedAdTextUpdated {
//             id: self.uuid.value(),
//             ad_text: text.value(),
//         };
//         self._entity.raise(event.clone().into());
//         self.when(event.into())?;

//         Ok(())
//     }

//     /// Set the classified ad's  title.
//     pub fn set_title(&mut self, title: ClassifiedAdTitle) -> Result<()> {
//         let event = ClassifiedAdTitleChanged {
//             id: self.uuid.value(),
//             title: title.value(),
//         };
//         self._entity.raise(event.clone().into());
//         self.when(event.into())?;

//         Ok(())
//     }

//     pub fn request_to_publish(&mut self) -> Result<()> {
//         if self._title == None {
//             return Err(anyhow!("Title cannot be empty"));
//         }
//         if self._text == None {
//             return Err(anyhow!("Text cannot be empty"));
//         }
//         let invalid_price = match &self._price {
//             Some(p) => p.is_zero(),
//             None => true,
//         };
//         if invalid_price {
//             return Err(anyhow!("Price cannot be 0"));
//         }

//         let event = ClassifiedAdSentForReview {
//             id: self.uuid.value(),
//         };

//         self._entity.raise(event.clone().into());
//         self.when(event.into())?;
//         Ok(())
//     }

//     /// Get a reference to the classified ad's  state.
//     pub fn state(&self) -> ClassifiedAdState {
//         self._state.clone()
//     }
// }

// /// From Enforcing the rules section
// impl IAggregateRoot<ClassifiedAdEvents> for ClassifiedAd {
//     fn when(&mut self, event: ClassifiedAdEvents) -> Result<()> {
//         self.when(event)
//     }

//     fn ensure_valid_state(&self) -> Result<()> {
//         self.ensure_valid_state()
//     }
// }

// pub struct FakeCurrencyLookup;

// const CURRENCIES: &[CurrencyDetails] = &[CurrencyDetails {
//     currency_code: CurrencyCode::EUR,
//     in_use: true,
//     decimal_places: 2,
// }];

// impl ICurrencyLookup for FakeCurrencyLookup {
//     fn find_currency(&self, currency_code: CurrencyCode) -> Result<CurrencyDetails> {
//         let currency = CURRENCIES
//             .iter()
//             .find(|&c| c.currency_code == currency_code)
//             .cloned();

//         match currency {
//             Some(currency_details) => Ok(currency_details),
//             None => Err(anyhow!(
//                 "Could not find currency with code {:?}",
//                 currency_code
//             )),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {

//     use uuid::Uuid;

//     use super::*;

//     // Mocks
//     // Tests

//     #[test]
//     fn money_with_same_amount_should_be_equal() -> Result<()> {
//         let first_amount = Money::from_decimal(5., None, FakeCurrencyLookup)?;
//         let second_amount = Money::from_decimal(5., None, FakeCurrencyLookup)?;
//         assert_eq!(first_amount, second_amount);
//         Ok(())
//     }

//     #[test]
//     fn sum_of_money_gives_full_amount() -> Result<()> {
//         let coin1 = Money::from_decimal(1., None, FakeCurrencyLookup)?;
//         let coin2 = Money::from_decimal(2., None, FakeCurrencyLookup)?;
//         let coin3 = Money::from_decimal(2., None, FakeCurrencyLookup)?;

//         let banknote = Money::from_decimal(5., None, FakeCurrencyLookup)?;

//         assert_eq!(banknote, (coin1 + coin2 + coin3).unwrap());
//         Ok(())
//     }
//     #[test]
//     fn subtracting_money() -> Result<()> {
//         let coin1 = Money::from_decimal(4., None, FakeCurrencyLookup)?;
//         let coin2 = Money::from_decimal(3., None, FakeCurrencyLookup)?;
//         let coin3 = Money::from_decimal(1., None, FakeCurrencyLookup)?;

//         assert_eq!((coin1 - coin2).unwrap(), coin3);
//         Ok(())
//     }

//     #[test]
//     fn can_publish_an_ad() -> Result<()> {
//         let mut classified_ad = ClassifiedAd::new(
//             ClassifiedAdId::new(Uuid::new_v4()),
//             UserId::new(Uuid::new_v4()),
//         );

//         classified_ad.set_title("Test ad".into())?;
//         classified_ad.set_text("Please buy my stuff".into())?;
//         let price = Price::from_decimal(100., None, FakeCurrencyLookup)?;
//         classified_ad.update_price(price)?;
//         classified_ad.request_to_publish()?;

//         // assert!(matches!(
//         //     classified_ad.state(),
//         //     ClassifiedAdState::PendingReview
//         // ));
//         Ok(())
//     }
// }
