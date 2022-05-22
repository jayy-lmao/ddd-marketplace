use anyhow::{anyhow, Result};
use marketplace_framework::AggregateRoot;
use uuid::Uuid;

use crate::{
    classified_ad_events::*, CurrencyCode, CurrencyDetails, ICurrencyLookup, Price, UserId,
};
// ================================================================================
// Value Objects
// ================================================================================
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

    pub fn value(&self) -> String {
        self._value.to_owned()
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
    pub fn value(&self) -> String {
        self._value.to_owned()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ClassifiedAdState {
    PendingReview,
    Active,
    InActive,
    MarkedAsSold,
}

// ================================================================================
// Events
// ================================================================================

// ================================================================================
// Aggregate
// ================================================================================
pub trait ClassifiedAdAggregate:
    AggregateRoot<Id = ClassifiedAdId, Event = ClassifiedAdEvents>
{
    fn id(&self) -> Result<ClassifiedAdId>;

    /// Set the classified ad's  price.
    fn update_price(&mut self, price: Price) -> Result<()> {
        let event = ClassifiedAdPriceUpdated {
            id: self.id()?.value(),
            price: price.money.amount,
        };

        self.apply(event)?;

        Ok(())
    }

    /// Set the classified ad's  text.
    fn set_text(&mut self, text: ClassifiedAdText) -> Result<()> {
        let event = ClassifiedAdTextUpdated {
            id: self.id()?.value(),
            ad_text: text.value(),
        };
        self.apply(event)?;

        Ok(())
    }

    /// Set the classified ad's  title.
    fn set_title(&mut self, title: ClassifiedAdTitle) -> Result<()> {
        let event = ClassifiedAdTitleChanged {
            id: self.id()?.value(),
            title: title.value(),
        };
        self.apply(event)?;

        Ok(())
    }

    fn title(&self) -> Option<ClassifiedAdTitle>;
    fn text(&self) -> Option<ClassifiedAdText>;
    fn price(&self) -> Option<Price>;

    fn request_to_publish(&mut self) -> Result<()> {
        if self.title() == None {
            return Err(anyhow!("Title cannot be empty"));
        }
        if self.text() == None {
            return Err(anyhow!("Text cannot be empty"));
        }
        let invalid_price = match self.price() {
            Some(p) => p.is_zero(),
            None => true,
        };
        if invalid_price {
            return Err(anyhow!("Price cannot be 0"));
        }

        let event = ClassifiedAdSentForReview {
            id: self.id()?.value(),
        };

        self.apply(event)?;
        Ok(())
    }

    // /// Get a reference to the classified ad's  state.
    // pub fn state(&self) -> ClassifiedAdState {
    //     self._state.clone()
    // }
}

#[derive(Clone)]
pub struct ClassifiedAd {
    _owner_id: Option<UserId>,
    _approved_by: Option<UserId>,
    _text: Option<ClassifiedAdText>,
    _title: Option<ClassifiedAdTitle>,
    _price: Option<Price>,
    _state: ClassifiedAdState,
    _changes: Vec<ClassifiedAdEvents>,

    pub uuid: Option<ClassifiedAdId>,
}
impl ClassifiedAd {
    pub fn new(classified_ad_id: ClassifiedAdId, owner_id: UserId) -> Self {
        Self {
            uuid: Some(classified_ad_id),
            _owner_id: Some(owner_id),
            _approved_by: None,
            _text: None,
            _title: None,
            _price: None,
            _state: ClassifiedAdState::InActive,
            _changes: vec![],
        }
    }
}

impl AggregateRoot for ClassifiedAd {
    type Id = ClassifiedAdId;
    type Event = ClassifiedAdEvents;

    fn ensure_valid_state(&self) -> Result<()> {
        let valid = self.uuid.is_some()
            && self._owner_id.is_some()
            && match self._state {
                ClassifiedAdState::PendingReview => {
                    self._title.is_some()
                        && self._text.is_some()
                        && self._price.is_some()
                        && !self._price.ok_or(anyhow!("No price"))?.is_zero()
                }
                ClassifiedAdState::Active => {
                    self._title.is_some()
                        && self._text.is_some()
                        && self._price.is_some()
                        && !self._price.ok_or(anyhow!("No price"))?.is_zero()
                        && self._approved_by.is_some()
                }
                _ => true,
            };
        if !valid {
            return Err(anyhow!("Post-checks failed in state {:?}", self._state));
        }
        Ok(())
    }

    fn when(&mut self, event: Self::Event) -> Result<()> {
        match event {
            ClassifiedAdEvents::Created(e) => {
                self._owner_id = Some(UserId::new(e.owner_id));
                self.uuid = Some(ClassifiedAdId::new(e.id));
                self._state = ClassifiedAdState::InActive;
            }
            ClassifiedAdEvents::TextUpdated(e) => {
                self._text = Some(ClassifiedAdText::new(e.ad_text))
            }
            ClassifiedAdEvents::TitleChanged(e) => {
                self._title = Some(ClassifiedAdTitle::new(e.title)?)
            }
            ClassifiedAdEvents::PriceUpdated(e) => {
                self._price = Some(Price::from_decimal(e.price, None, FakeCurrencyLookup)?)
            }
            ClassifiedAdEvents::SentForReview(_e) => self._state = ClassifiedAdState::PendingReview,
        };
        Ok(())
    }

    fn store_changes(&mut self, event: Self::Event) -> Result<()> {
        self._changes.push(event);
        Ok(())
    }
}

impl ClassifiedAdAggregate for ClassifiedAd {
    fn id(&self) -> Result<ClassifiedAdId> {
        self.uuid.ok_or(anyhow!("No uuid - illegal state"))
    }

    fn title(&self) -> Option<ClassifiedAdTitle> {
        self._title.clone()
    }

    fn text(&self) -> Option<ClassifiedAdText> {
        self._text.clone()
    }

    fn price(&self) -> Option<Price> {
        self._price
    }
}

pub struct FakeCurrencyLookup;

const CURRENCIES: &[CurrencyDetails] = &[CurrencyDetails {
    currency_code: CurrencyCode::EUR,
    in_use: true,
    decimal_places: 2,
}];

impl ICurrencyLookup for FakeCurrencyLookup {
    fn find_currency(&self, currency_code: CurrencyCode) -> Result<CurrencyDetails> {
        let currency = CURRENCIES
            .iter()
            .find(|&c| c.currency_code == currency_code)
            .cloned();

        match currency {
            Some(currency_details) => Ok(currency_details),
            None => Err(anyhow!(
                "Could not find currency with code {:?}",
                currency_code
            )),
        }
    }
}
