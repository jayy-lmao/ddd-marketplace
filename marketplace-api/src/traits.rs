use poem::async_trait;

pub trait IHandleCommand {
    type Command;
    fn handle(&self, command: Self::Command);
}

pub trait IEntityStore: Sync + Send {
    type Entity;
    fn load(&self, id: String) -> Self::Entity;
    fn exists(&self, id: String) -> bool;
    fn save(&mut self, entity: Self::Entity);
}

pub trait IApplicationService {
    type Command;
    fn handle(&self, command: impl Into<Self::Command>);
}
