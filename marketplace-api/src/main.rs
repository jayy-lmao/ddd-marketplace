use std::{
    collections::HashMap,
    str::FromStr,
    sync::{Arc, Mutex},
};

use marketplace_contracts::classified_ads;
use marketplace_domain::*;
// use poem::{listener::TcpListener, middleware::AddData, EndpointExt, Route, Server};
use poem::{
    error::InternalServerError, http::StatusCode, listener::TcpListener, middleware::Cors,
    web::Data, EndpointExt, Result, Route, Server,
};
use poem_openapi::{
    param::Path,
    payload::{Json, PlainText},
    ApiResponse, Object, OpenApi, OpenApiService,
};
use uuid::Uuid;

pub trait IHandleCommand {
    type Command;
    fn handle(&self, command: Self::Command);
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

// pub mod ad {
//     use poem::{handler, http::StatusCode, post, web, Route};

//     use crate::{ClassifiedAdsCommandApi, IHandleCommand};

//     pub fn route() -> Route {
//         Route::new().at("/", post(create_ad))
//     }

//     #[handler]
//     async fn create_ad(
//         web::Data(classified_ads_cmd_api): web::Data<&ClassifiedAdsCommandApi>,
//         web::Json(request): web::Json<marketplace_contracts::classified_ads::v1::Create>,
//     ) -> StatusCode {
//         let res = classified_ads_cmd_api
//             .create_ad_command_handler
//             .clone()
//             .handle(request);
//         StatusCode::CREATED
//     }
// }

// Create
#[derive(Object)]
pub struct ClassifiedAdsV1Create {
    /// Uuid
    id: String,
    /// Uuid of owner
    owner_id: String,
}

struct ClassifiedAdApi;
#[OpenApi]
impl ClassifiedAdApi {
    /// Create an item
    #[oai(path = "/ad", method = "post")]
    async fn create(
        &self,
        classified_ads_cmd_api: Data<&ClassifiedAdsCommandApi>,
        request: Json<ClassifiedAdsV1Create>,
    ) -> Result<Json<i64>> {
        let id = Uuid::from_str(request.id.as_str()).unwrap();
        let owner_id = Uuid::from_str(request.owner_id.as_str()).unwrap();
        let cmd = marketplace_contracts::classified_ads::v1::Create { id, owner_id };
        classified_ads_cmd_api.create_ad_command_handler.handle(cmd);

        Ok(Json(34))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();
    let classified_ads_api = ClassifiedAdsCommandApi::new();

    let api_service = OpenApiService::new(ClassifiedAdApi, "Classified Ads", "1.0.0")
        .server("http://localhost:8000");
    let ui = api_service.swagger_ui();
    let spec = api_service.spec();
    let route = Route::new()
        .nest("/", api_service)
        .nest("/ui", ui)
        .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
        .with(Cors::new())
        .data(classified_ads_api);

    // let app = Route::new().nest("/ad", ad::route().with(AddData::new(classified_ads_api)));
    Server::new(TcpListener::bind("127.0.0.1:8000"))
        .run(route)
        .await?;
    Ok(())
}
