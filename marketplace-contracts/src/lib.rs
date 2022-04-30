pub mod classified_ads {
    pub mod v1 {
        use serde_derive::Deserialize;
        use uuid::Uuid;

        #[derive(Deserialize)]
        pub struct Create {
            pub id: Uuid,
            pub owner_id: Uuid,
        }
    }
}
