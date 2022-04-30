pub trait IHandleCommand {
    type Command;
    fn handle(&self, command: Self::Command);
}

pub trait IEntityStore: Sync + Send {
    type Entity;
    fn load(&self, id: String) -> Self::Entity;
    fn save(&mut self, entity: Self::Entity);
}
