use anyhow::{anyhow, Result};

pub trait AggregateRoot {
    type Id;
    type Event: Clone;
    fn ensure_valid_state(&self) -> Result<()>;
    fn when(&mut self, event: Self::Event) -> Result<()>;
    fn store_changes(&mut self, event: Self::Event) -> Result<()>;

    fn apply(&mut self, event: impl Into<Self::Event>) -> Result<()> {
        let event: Self::Event = event.into();
        self.when(event.clone())?;
        self.ensure_valid_state()?;
        self.store_changes(event)?;
        Ok(())
    }
}

// #[derive(Clone)]
// pub struct Entity<T> {
//     _events: Vec<T>,
// }

// impl<T> Entity<T> {
//     pub fn new() -> Self {
//         Self { _events: vec![] }
//     }
//     pub fn raise(&mut self, event: T) {
//         self._events.push(event);
//     }

//     pub fn clear_changes(&mut self) {
//         self._events = vec![];
//     }
// }

// pub trait IAggregateRoot<Ev> {
//     fn when(&mut self, e: Ev) -> Result<()>;
//     fn ensure_valid_state(&self) -> Result<()>;
// }

// pub struct AggregateRoot<TId, Ev, Ent>
// where
//     Ev: Clone,
//     Ent: IAggregateRoot<Ev>,
// {
//     _id: TId,
//     _changes: Vec<Ev>,
//     _entity: Ent,
// }

// impl<TId, Ev: Clone, Ent: IAggregateRoot<Ev>> AggregateRoot<TId, Ev, Ent> {
//     pub fn new(id: TId, ent: Ent) -> Self {
//         Self {
//             _id: id,
//             _changes: vec![],
//             _entity: ent,
//         }
//     }
//     pub fn apply(&mut self, event: Ev) -> Result<()> {
//         self._entity.when(event.clone())?;
//         self._entity.ensure_valid_state()?;
//         self._changes.push(event);
//         Ok(())
//     }
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
