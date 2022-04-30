#[derive(Clone)]
pub struct Entity<T> {
    _events: Vec<T>,
}
impl<T> Entity<T> {
    pub fn new() -> Self {
        Self { _events: vec![] }
    }
    pub fn raise(&mut self, event: T) {
        self._events.push(event);
    }

    pub fn clear_changes(&mut self) {
        self._events = vec![];
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
