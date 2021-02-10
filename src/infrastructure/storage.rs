use crate::domain::Item;

pub trait Storage: Send + Sync {
    fn read(&self) -> Vec<Item>;
    fn write(&mut self, list: Vec<Item>);
}
