use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use marketplace_contracts::classified_ads::v1;
use marketplace_domain::*;
use poem_openapi::Object;

use crate::traits::{IApplicationService, IEntityStore, IHandleCommand};

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

    fn exists(&self, id: String) -> bool {
        let exists = self._store.get(&id).is_some();
        exists
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

#[derive(Clone)]
pub struct ClassifiedAdsApplicationService {
    _api: ClassifiedAdsCommandApi,
}

impl ClassifiedAdsApplicationService {
    pub fn new() -> Self {
        Self {
            _api: ClassifiedAdsCommandApi::new(),
        }
    }
}

impl IApplicationService for ClassifiedAdsApplicationService {
    type Command = v1::Commands;
    fn handle(&self, command: impl Into<Self::Command>) {
        match command.into() {
            v1::Commands::Create(cmd) => self._api.create_ad_command_handler.handle(cmd),
            v1::Commands::SetTitle(_) => todo!(),
            v1::Commands::UpdateText(_) => todo!(),
            v1::Commands::UpdatePrice(_) => todo!(),
            v1::Commands::RequestToPublish(_) => todo!(),
        }
    }
}

/// Create
#[derive(Object)]
pub struct ClassifiedAdsV1Create {
    /// String
    pub id: String,
    /// String of owner
    pub owner_id: String,
}

#[derive(Object)]
pub struct ClassifiedAdV1Create {
    pub id: String,
    pub owner_id: String,
}
#[derive(Object)]
pub struct ClassifiedAdV1SetTitle {
    pub id: String,
    pub title: String,
}
#[derive(Object)]
pub struct ClassifiedAdV1UpdateText {
    pub id: String,
    pub text: String,
}
#[derive(Object)]
pub struct ClassifiedAdV1UpdatePrice {
    pub id: String,
    pub price: f64,
    pub currency: String,
}
#[derive(Object)]
pub struct ClassifiedAdV1RequestToPublish {
    pub id: String,
}
