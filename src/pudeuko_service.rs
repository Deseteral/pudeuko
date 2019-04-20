use crate::domain::{Item, ItemList};
use crate::infrastructure::Storage;
use std::sync::{Arc, Mutex};

pub struct PudeukoService {
    storage: Box<dyn Storage>,
}

pub type SharedPudeukoService = Arc<Mutex<PudeukoService>>;

impl PudeukoService {
    pub fn new(storage: Box<dyn Storage>) -> Self {
        Self { storage }
    }

    pub fn make_shared(pudeuko_service: PudeukoService) -> SharedPudeukoService {
        Arc::new(Mutex::new(pudeuko_service))
    }

    pub fn get_all(&self) -> ItemList {
        self.storage.read()
    }

    pub fn add_item(&mut self, item: Item) {
        let mut list = self.storage.read();
        list.insert(0, item);
        self.storage.write(list);
    }

    pub fn get_item_by_id(&self, id: &str) -> Option<Item> {
        self.storage
            .read()
            .iter()
            .find(|&item| item.id == id)
            .cloned()
    }
}
