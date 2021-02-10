use crate::{domain::Item, infrastructure::Storage};

pub struct InMemoryStorage {
    items: Vec<Item>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
}

impl Storage for InMemoryStorage {
    fn read(&self) -> Vec<Item> {
        self.items.to_vec()
    }

    fn write(&mut self, list: Vec<Item>) {
        self.items = list;
    }
}
