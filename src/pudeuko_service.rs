use crate::{domain::Item, dropbox_storage::DropboxStorage};
use std::sync::{Arc, Mutex};

pub struct PudeukoService {
    storage: DropboxStorage,
}

pub type SharedPudeukoService = Arc<Mutex<PudeukoService>>;

impl PudeukoService {
    pub fn new(storage: DropboxStorage) -> Self {
        Self { storage }
    }

    pub fn make_shared(pudeuko_service: PudeukoService) -> SharedPudeukoService {
        Arc::new(Mutex::new(pudeuko_service))
    }

    pub fn get_all(&self) -> Vec<Item> {
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

    pub fn remove_item_by_id(&mut self, id: &str) {
        let mut list = self.storage.read();
        let index = list.iter().position(|item| item.id == id).unwrap();
        list.remove(index);

        self.storage.write(list);
    }
}
