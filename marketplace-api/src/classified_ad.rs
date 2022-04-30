use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use marketplace_domain::*;
use poem_openapi::Object;

use crate::traits::{IEntityStore, IHandleCommand};

pub struct ClassifiedAdStore {
    _store: HashMap<String, ClassifiedAd>,
}

impl ClassifiedAdStore {
    pub fn new() -> Self {
        Self {
            _store: HashMap::new(),
        }
    }
}
impl IEntityStore for ClassifiedAdStore {
    type Entity = ClassifiedAd;
    fn save(&mut self, ad: ClassifiedAd) {
        self._store.insert(ad.uuid.value().to_string(), ad);
    }

    fn load(&self, id: String) -> ClassifiedAd {
        let ad = self._store.get(&id).unwrap();
        ad.clone()
    }
}

#[derive(Clone)]
pub struct CreateClassifiedAdHandler {
    _store: Arc<Mutex<dyn IEntityStore<Entity = ClassifiedAd>>>,
}

impl CreateClassifiedAdHandler {
    pub fn new_in_memory_store() -> Self {
        let store = Arc::new(Mutex::new(ClassifiedAdStore::new()));
        Self { _store: store }
    }
}
impl IHandleCommand for CreateClassifiedAdHandler {
    type Command = marketplace_contracts::classified_ads::v1::Create;

    fn handle(&self, command: Self::Command) {
        let classified_ad = ClassifiedAd::new(
            ClassifiedAdId::new(command.id),
            UserId::new(command.owner_id),
        );
        return self._store.clone().lock().unwrap().save(classified_ad);
    }
}
#[derive(Clone)]
pub struct ClassifiedAdsCommandApi {
    pub create_ad_command_handler: CreateClassifiedAdHandler,
}

impl ClassifiedAdsCommandApi {
    pub fn new() -> Self {
        Self {
            create_ad_command_handler: CreateClassifiedAdHandler::new_in_memory_store(),
        }
    }
}

/// Create
#[derive(Object)]
pub struct ClassifiedAdsV1Create {
    /// Uuid
    pub id: String,
    /// Uuid of owner
    pub owner_id: String,
}
