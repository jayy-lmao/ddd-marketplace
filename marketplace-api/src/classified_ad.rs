use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::{anyhow, Result};
use marketplace_contracts::classified_ads::v1::{self};
use marketplace_domain::{classified_ad::*, UserId};
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

    fn save(&mut self, ad: ClassifiedAd) -> Result<()> {
        self._store.insert(ad.id()?.value().to_string(), ad);
        Ok(())
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

    fn handle(&self, command: Self::Command) -> Result<()> {
        let classified_ad = ClassifiedAd::new(
            ClassifiedAdId::new(command.id),
            UserId::new(command.owner_id),
        );
        self._store.clone().lock().unwrap().save(classified_ad)?;
        Ok(())
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
    _repository: Arc<Mutex<dyn IEntityStore<Entity = ClassifiedAd>>>,
}

impl ClassifiedAdsApplicationService {
    pub fn new() -> Self {
        Self {
            _api: ClassifiedAdsCommandApi::new(),
            _repository: Arc::new(Mutex::new(ClassifiedAdStore::new())),
        }
    }
    fn handle_create(&self, cmd: v1::Create) -> Result<()> {
        if self._repository.lock().unwrap().exists(cmd.id.to_string()) {
            return Err(anyhow!("Classified Ad with this ID Already exists"));
        }
        let classified_ad =
            ClassifiedAd::new(ClassifiedAdId::new(cmd.id), UserId::new(cmd.owner_id));
        self._repository.lock().unwrap().save(classified_ad)?;
        Ok(())
    }
    fn handle_update<Cmd>(
        &self,
        id: ClassifiedAdId,
        cmd: Cmd,
        operation: fn(cmd: Cmd, c: &mut ClassifiedAd) -> Result<()>,
    ) -> Result<()> {
        let mut classified_ad = self
            ._repository
            .lock()
            .unwrap()
            .load(id.value().to_string());
        operation(cmd, &mut classified_ad)?;
        self._repository.lock().unwrap().save(classified_ad)?;
        Ok(())
    }
}

impl IApplicationService for ClassifiedAdsApplicationService {
    type Command = v1::Commands;
    fn handle(&self, command: impl Into<Self::Command>) -> Result<()> {
        match command.into() {
            v1::Commands::Create(cmd) => self.handle_create(cmd)?,
            v1::Commands::SetTitle(cmd) => {
                self.handle_update(ClassifiedAdId::new(cmd.id), cmd, |cmd, c| {
                    c.set_title(cmd.title).expect("Could not set title");
                    Ok(())
                })?;
            }
            v1::Commands::UpdateText(cmd) => {
                self.handle_update(ClassifiedAdId::new(cmd.id), cmd, |cmd, c| {
                    c.set_text(cmd.text).expect("Could not set text");
                    Ok(())
                })?
            }
            v1::Commands::UpdatePrice(_) => todo!(),
            v1::Commands::RequestToPublish(_) => todo!(),
        };
        Ok(())
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
