use super::domain::{ItemList, Item};
use super::dropbox_client::DropboxClient;

pub struct PudeukoService {
    client: DropboxClient,
}

impl PudeukoService {
    pub fn new(dropbox_token: &String) -> Self {
        let client = DropboxClient::new(dropbox_token);
        Self { client }
    }

    pub fn get_all(self: &Self) -> ItemList {
        self.client.fetch()
    }

    pub fn add_item(self: &Self, item: Item) {
        let mut list = self.client.fetch();
        list.insert(0, item);
        self.client.upload(&list);
    }

    pub fn get_item_by_id(self: &Self, id: String) -> Option<Item> {
        self.client
            .fetch().iter()
            .find(|&item| item.id == id)
            .map(|item| item.clone())
    }
}
