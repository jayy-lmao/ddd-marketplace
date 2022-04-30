use std::{
    collections::HashMap,
    str::FromStr,
    sync::{Arc, Mutex},
};

use classified_ad::{ClassifiedAdsCommandApi, ClassifiedAdsV1Create};
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
use traits::IHandleCommand;
use uuid::Uuid;
pub mod classified_ad;
pub mod traits;

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
