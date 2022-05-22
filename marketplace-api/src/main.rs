use std::str::FromStr;

use classified_ad::{
    ClassifiedAdV1RequestToPublish, ClassifiedAdV1SetTitle, ClassifiedAdV1UpdatePrice,
    ClassifiedAdV1UpdateText, ClassifiedAdsApplicationService,
    ClassifiedAdsV1Create,
};

// use poem::{listener::TcpListener, middleware::AddData, EndpointExt, Route, Server};
use poem::{
    listener::TcpListener, middleware::Cors, web::Data, EndpointExt, Result,
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
        application_service: Data<&ClassifiedAdsApplicationService>,
        request: Json<ClassifiedAdsV1Create>,
    ) -> Result<PlainText<String>> {
        let id = Uuid::from_str(request.id.as_str()).unwrap();
        let owner_id = Uuid::from_str(request.owner_id.as_str()).unwrap();
        let cmd = marketplace_contracts::classified_ads::v1::Create { id, owner_id };
        application_service.handle(cmd);

        Ok(PlainText(String::from("Created")))
    }
    /// Update the title of an add
    #[oai(path = "/ad/title", method = "put")]
    async fn update_title(
        &self,
        application_service: Data<&ClassifiedAdsApplicationService>,
        request: Json<ClassifiedAdV1SetTitle>,
    ) -> Result<PlainText<String>> {
        let id = Uuid::from_str(request.id.as_str()).unwrap();
        let title = request.title.clone();
        let cmd = marketplace_contracts::classified_ads::v1::SetTitle { id, title };
        application_service.handle(cmd);
        Ok(PlainText(String::from("Updated")))
    }
    /// Update the text of an add
    #[oai(path = "/ad/text", method = "put")]
    async fn update_text(
        &self,
        application_service: Data<&ClassifiedAdsApplicationService>,
        request: Json<ClassifiedAdV1UpdateText>,
    ) -> Result<PlainText<String>> {
        let id = Uuid::from_str(request.id.as_str()).unwrap();
        let text = request.text.clone();
        let cmd = marketplace_contracts::classified_ads::v1::UpdateText { id, text };
        application_service.handle(cmd);

        Ok(PlainText(String::from("Updated")))
    }
    /// Update the price
    #[oai(path = "/ad/price", method = "put")]
    async fn update_price(
        &self,
        application_service: Data<&ClassifiedAdsApplicationService>,
        request: Json<ClassifiedAdV1UpdatePrice>,
    ) -> Result<PlainText<String>> {
        let id = Uuid::from_str(request.id.as_str()).unwrap();
        let price = request.price;
        let currency = request.currency.clone();
        let cmd = marketplace_contracts::classified_ads::v1::UpdatePrice {
            id,
            price,
            currency,
        };
        application_service.handle(cmd);
        Ok(PlainText(String::from("Updated")))
    }
    /// Update the price
    #[oai(path = "/ad/publish", method = "put")]
    async fn publish(
        &self,
        application_service: Data<&ClassifiedAdsApplicationService>,
        request: Json<ClassifiedAdV1RequestToPublish>,
    ) -> Result<PlainText<String>> {
        let id = Uuid::from_str(request.id.as_str()).unwrap();
        let cmd = marketplace_contracts::classified_ads::v1::RequestToPublish { id };
        application_service.handle(cmd);
        Ok(PlainText(String::from("Updated")))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();
    let classified_ads_application_service = ClassifiedAdsApplicationService::new();

    let api_service = OpenApiService::new(ClassifiedAdApi, "Classified Ads", "1.0.0")
        .server("http://localhost:8000");
    let ui = api_service.swagger_ui();
    let spec = api_service.spec();
    let route = Route::new()
        .nest("/", api_service)
        .nest("/ui", ui)
        .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
        .with(Cors::new())
        .data(classified_ads_application_service);

    // let app = Route::new().nest("/ad", ad::route().with(AddData::new(classified_ads_api)));
    Server::new(TcpListener::bind("127.0.0.1:8000"))
        .run(route)
        .await?;
    Ok(())
}
