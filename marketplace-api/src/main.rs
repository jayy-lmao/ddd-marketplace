use std::str::FromStr;

use classified_ad::{
    ClassifiedAdV1UpdateText, ClassifiedAdsApplicationService, ClassifiedAdsCommandApi,
    ClassifiedAdsV1Create,
};

// use poem::{listener::TcpListener, middleware::AddData, EndpointExt, Route, Server};
use poem::{
    http::StatusCode, listener::TcpListener, middleware::Cors, web::Data, EndpointExt, Result,
    Route, Server,
};
use poem_openapi::{
    payload::{Json, PlainText},
    OpenApi, OpenApiService,
};
use traits::{IApplicationService, IHandleCommand};
use uuid::Uuid;
pub mod classified_ad;
pub mod traits;

struct ClassifiedAdApi;
#[OpenApi]
impl ClassifiedAdApi {
    /// Create a classified ad
    #[oai(path = "/ad", method = "post")]
    async fn create(
        &self,
        classified_ads_cmd_api: Data<&ClassifiedAdsApplicationService>,
        request: Json<ClassifiedAdsV1Create>,
    ) -> Result<PlainText<String>> {
        let id = Uuid::from_str(request.id.as_str()).unwrap();
        let owner_id = Uuid::from_str(request.owner_id.as_str()).unwrap();
        let cmd = marketplace_contracts::classified_ads::v1::Create { id, owner_id };
        classified_ads_cmd_api.handle(cmd);

        Ok(PlainText(String::from("Created")))
    }
    /// Update the text of an add
    #[oai(path = "/ad/text", method = "put")]
    async fn update_text(
        &self,
        classified_ads_cmd_api: Data<&ClassifiedAdsApplicationService>,
        request: Json<ClassifiedAdV1UpdateText>,
    ) -> Result<PlainText<String>> {
        let id = Uuid::from_str(request.id.as_str()).unwrap();
        let text = request.text.clone();
        let cmd = marketplace_contracts::classified_ads::v1::UpdateText { id, text };
        classified_ads_cmd_api.handle(cmd);

        Ok(PlainText(String::from("Created")))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();
    let classified_ads_api = ClassifiedAdsApplicationService::new();

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
