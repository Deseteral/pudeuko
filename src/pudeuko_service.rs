use crate::domain::{Item, ItemList};
use crate::infrastructure::Storage;
use std::sync::{Arc, RwLock};

pub struct PudeukoService {
    storage: Box<dyn Storage>,
}

pub type SharedPudeukoService = Arc<RwLock<PudeukoService>>;

impl PudeukoService {
    pub fn new(storage: Box<dyn Storage>) -> Self {
        Self { storage }
    }

    pub fn get_all(&self) -> ItemList {
        self.storage.read()
    }

    pub fn add_item(&self, item: Item) {
        let mut list = self.storage.read();
        list.insert(0, item);
        self.storage.write(&list);
    }

    pub fn get_item_by_id(&self, id: &str) -> Option<Item> {
        self.storage
            .read()
            .iter()
            .find(|&item| item.id == id)
            .cloned()
    }

    pub fn remove_item_by_id(&self, id: &str) {
        let mut list = self.storage.read();
        let index = list
            .iter()
            .position(|item| item.id == id)
            .unwrap();
        list.remove(index);

        self.storage.write(&list);
    }
}
