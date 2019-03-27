use crate::domain::{Item, ItemList};
use crate::infrastructure::Storage;

pub struct PudeukoService {
    storage: Box<dyn Storage>,
}

impl PudeukoService {
    pub fn new(storage: Box<dyn Storage>) -> Self {
        Self { storage }
    }

    pub fn get_all(self: &Self) -> ItemList {
        self.storage.read()
    }

    pub fn add_item(self: &Self, item: Item) {
        let mut list = self.storage.read();
        list.insert(0, item);
        self.storage.write(&list);
    }

    pub fn get_item_by_id(self: &Self, id: String) -> Option<Item> {
        self.storage
            .read()
            .iter()
            .find(|&item| item.id == id)
            .cloned()
    }
}
