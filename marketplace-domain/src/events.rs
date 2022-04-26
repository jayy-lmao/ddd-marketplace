use uuid::Uuid;
pub enum ClassifiedAdEvents {
    Created(ClassifiedAdCreated),
    TextUpdated(ClassifiedAdTextUpdated),
    TitleChanged(ClassifiedAdTitleChanged),
    PriceUpdated(ClassifiedAdPriceUpdated),
    SentForReview(ClassifiedAdSentForReview),
}

#[derive(Clone)]
pub struct ClassifiedAdCreated {
    pub id: Uuid,
    pub owner_id: Uuid,
}
impl From<ClassifiedAdCreated> for ClassifiedAdEvents {
    fn from(e: ClassifiedAdCreated) -> Self {
        ClassifiedAdEvents::Created(e)
    }
}

#[derive(Clone)]
pub struct ClassifiedAdTitleChanged {
    pub id: Uuid,
    pub title: String,
}

impl From<ClassifiedAdTitleChanged> for ClassifiedAdEvents {
    fn from(e: ClassifiedAdTitleChanged) -> Self {
        ClassifiedAdEvents::TitleChanged(e)
    }
}
#[derive(Clone)]
pub struct ClassifiedAdTextUpdated {
    pub id: Uuid,
    pub ad_text: String,
}
impl From<ClassifiedAdTextUpdated> for ClassifiedAdEvents {
    fn from(e: ClassifiedAdTextUpdated) -> Self {
        ClassifiedAdEvents::TextUpdated(e)
    }
}

#[derive(Clone)]
pub struct ClassifiedAdPriceUpdated {
    pub id: Uuid,
    pub price: f64,
}
impl From<ClassifiedAdPriceUpdated> for ClassifiedAdEvents {
    fn from(e: ClassifiedAdPriceUpdated) -> Self {
        ClassifiedAdEvents::PriceUpdated(e)
    }
}
#[derive(Clone)]
pub struct ClassifiedAdSentForReview {
    pub id: Uuid,
}
impl From<ClassifiedAdSentForReview> for ClassifiedAdEvents {
    fn from(e: ClassifiedAdSentForReview) -> Self {
        ClassifiedAdEvents::SentForReview(e)
    }
}
