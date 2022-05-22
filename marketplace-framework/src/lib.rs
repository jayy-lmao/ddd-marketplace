use anyhow::Result;

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
