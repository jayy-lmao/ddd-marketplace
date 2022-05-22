/**
 * This approach of using almost exclusively traits
 * is a much more recent iteration on the approach
 * used for marketplace ads
 *
 * TODO: I should attempt to migrate the classified ad approach to this model, As it is more simple
 */
use crate::UserId;
use anyhow::Result;
use marketplace_framework::AggregateRoot;

// ================================================================================
// Value Objects
// ================================================================================

#[derive(Clone)]
pub struct FullName {}
#[derive(Clone)]
pub struct DisplayName {}

// ================================================================================
// Events
// ================================================================================

#[derive(Clone)]
pub struct UserRegistered {
    id: UserId,
    full_name: FullName,
    display_name: DisplayName,
}

impl From<UserRegistered> for UserEvents {
    fn from(e: UserRegistered) -> Self {
        UserEvents::UserRegistered(e)
    }
}

#[derive(Clone)]
pub struct UserFullNameUpdated {
    full_name: FullName,
    id: UserId,
}
impl From<UserFullNameUpdated> for UserEvents {
    fn from(e: UserFullNameUpdated) -> Self {
        UserEvents::UserFullNameUpdated(e)
    }
}
#[derive(Clone)]
pub struct UserDisplayNameUpdated {
    display_name: DisplayName,
    id: UserId,
}
impl From<UserDisplayNameUpdated> for UserEvents {
    fn from(e: UserDisplayNameUpdated) -> Self {
        UserEvents::UserDisplayNameUpdated(e)
    }
}

#[derive(Clone)]
pub enum UserEvents {
    UserRegistered(UserRegistered),
    UserFullNameUpdated(UserFullNameUpdated),
    UserDisplayNameUpdated(UserDisplayNameUpdated),
}

// ================================================================================
// Aggregate
// ================================================================================

pub trait UserProfileAggregate: AggregateRoot<Id = UserId, Event = UserEvents> {
    // Aggregate State Properties
    fn full_name(&self) -> FullName;
    fn id(&self) -> UserId;

    fn display_name(&self) -> DisplayName;

    fn db_id(&self) -> String {
        let id = self.id().value();
        format!("UserProfile/{id}")
    }

    // Commands
    fn create_new_profile(
        &mut self,
        id: UserId,
        full_name: FullName,
        display_name: DisplayName,
    ) -> Result<()> {
        self.apply(UserRegistered {
            id,
            full_name,
            display_name,
        })
    }

    fn update_full_name(&mut self, id: UserId, full_name: FullName) -> Result<()> {
        self.apply(UserFullNameUpdated { id, full_name })
    }
    fn update_display_name(&mut self, id: UserId, display_name: DisplayName) -> Result<()> {
        self.apply(UserDisplayNameUpdated { id, display_name })
    }
}

pub struct UserProfile {
    _id: Option<UserId>,
    _full_name: Option<FullName>,
    _display_name: Option<DisplayName>,
    _changes: Vec<UserEvents>,
}

impl UserProfile {
    pub fn new_empty() -> Self {
        Self {
            _id: None,
            _full_name: None,
            _display_name: None,
            _changes: vec![],
        }
    }
}

impl AggregateRoot for UserProfile {
    type Id = UserId;
    type Event = UserEvents;

    fn ensure_valid_state(&self) -> Result<()> {
        Ok(())
    }

    fn when(&mut self, event: Self::Event) -> Result<()> {
        match event.into() {
            UserEvents::UserRegistered(e) => {
                self._display_name = Some(e.display_name);
                self._full_name = Some(e.full_name);
            }
            UserEvents::UserFullNameUpdated(e) => {
                self._full_name = Some(e.full_name);
            }
            UserEvents::UserDisplayNameUpdated(e) => {
                self._display_name = Some(e.display_name);
            }
        };
        Ok(())
    }

    fn store_changes(&mut self, event: Self::Event) -> Result<()> {
        self._changes.push(event);
        Ok(())
    }
}

impl UserProfileAggregate for UserProfile {
    fn full_name(&self) -> FullName {
        self._full_name.clone().unwrap() // Can be none
    }

    fn id(&self) -> UserId {
        self._id.clone().unwrap() // Can be none
    }

    fn display_name(&self) -> DisplayName {
        self._display_name.clone().unwrap() // Can be none
    }
}
