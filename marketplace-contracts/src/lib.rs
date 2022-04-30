pub mod classified_ads {
    pub mod v1 {
        use serde_derive::Deserialize;
        use uuid::Uuid;

        #[derive(Deserialize)]
        pub struct Create {
            pub id: Uuid,
            pub owner_id: Uuid,
        }
        impl From<Create> for Commands {
            fn from(cmd: Create) -> Self {
                Commands::Create(cmd)
            }
        }
        #[derive(Deserialize)]
        pub struct SetTitle {
            pub id: Uuid,
            pub title: String,
        }
        impl From<SetTitle> for Commands {
            fn from(cmd: SetTitle) -> Self {
                Commands::SetTitle(cmd)
            }
        }
        #[derive(Deserialize)]
        pub struct UpdateText {
            pub id: Uuid,
            pub text: String,
        }
        impl From<UpdateText> for Commands {
            fn from(cmd: UpdateText) -> Self {
                Commands::UpdateText(cmd)
            }
        }
        #[derive(Deserialize)]
        pub struct UpdatePrice {
            pub id: Uuid,
            pub price: f64,
            pub currency: String,
        }
        impl From<UpdatePrice> for Commands {
            fn from(cmd: UpdatePrice) -> Self {
                Commands::UpdatePrice(cmd)
            }
        }
        #[derive(Deserialize)]
        pub struct RequestToPublish {
            pub id: Uuid,
        }
        impl From<RequestToPublish> for Commands {
            fn from(cmd: RequestToPublish) -> Self {
                Commands::RequestToPublish(cmd)
            }
        }

        #[derive(Deserialize)]
        pub enum Commands {
            Create(Create),
            SetTitle(SetTitle),
            UpdateText(UpdateText),
            UpdatePrice(UpdatePrice),
            RequestToPublish(RequestToPublish),
        }
    }
}
