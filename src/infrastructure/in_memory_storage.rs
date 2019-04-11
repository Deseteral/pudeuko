use crate::domain::ItemList;
use crate::infrastructure::Storage;

pub struct InMemoryStorage {
    items: ItemList,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
}

impl Storage for InMemoryStorage {
    fn read(&self) -> ItemList {
        self.items.to_vec()
    }

    fn write(&mut self, list: ItemList) {
        self.items = list;
    }
}
