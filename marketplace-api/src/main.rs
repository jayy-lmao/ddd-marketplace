use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use marketplace_domain::*;
use poem::{listener::TcpListener, middleware::AddData, EndpointExt, Route, Server};

pub trait IHandleCommand {
    type Command;
    fn handle(&mut self, command: Self::Command);
}

pub trait IEntityStore: Sync + Send {
    type Entity;
    fn load(&self, id: String) -> Self::Entity;
    fn save(&mut self, entity: Self::Entity);
}

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

    fn handle(&mut self, command: Self::Command) {
        let classified_ad = ClassifiedAd::new(
            ClassifiedAdId::new(command.id),
            UserId::new(command.owner_id),
        );
        return self._store.lock().unwrap().save(classified_ad);
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

pub mod ad {
    use std::sync::Arc;

    use marketplace_contracts::classified_ads;
    use poem::{handler, http::StatusCode, post, web, Route};
    use tokio::sync::Mutex;

    use crate::{ClassifiedAdsCommandApi, IHandleCommand};

    pub fn route() -> Route {
        Route::new().at("/", post(create_ad))
    }

    #[handler]
    async fn create_ad(
        web::Data(classified_ads_cmd_api): web::Data<&Arc<Mutex<ClassifiedAdsCommandApi>>>,
        web::Json(request): web::Json<marketplace_contracts::classified_ads::v1::Create>,
    ) -> StatusCode {
        let res = classified_ads_cmd_api
            .lock()
            .await
            .create_ad_command_handler
            .handle(request);
        StatusCode::CREATED
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // let classified_ads_service = Arc::new(ClassifiedAdsApplicationService::new());
    let classified_ads_api = ClassifiedAdsCommandApi::new();

    let app = Route::new().nest(
        "/ad",
        ad::route().with(AddData::new(Arc::new(Mutex::new(classified_ads_api)))),
    );
    Server::new(TcpListener::bind("127.0.0.1:8000"))
        .run(app)
        .await
}
