use anyhow::Result;
use poem::async_trait;

pub trait IHandleCommand {
    type Command;
    fn handle(&self, command: Self::Command);
}

pub trait IEntityStore: Sync + Send {
    type Entity;
    /// Loads an entity by id
    fn load(&self, id: String) -> Self::Entity;
    /// Check if entity with a given id already exists
    fn exists(&self, id: String) -> bool;
    /// Persists an entity
    fn save(&mut self, entity: Self::Entity);
}

pub trait IApplicationService {
    type Command;
    fn handle(&self, command: impl Into<Self::Command>) -> Result<()>;
}

