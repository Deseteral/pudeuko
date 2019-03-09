use super::domain::{ItemList};

pub trait Storage {
    fn read(self: &Self) -> ItemList;
    fn write(self: &Self, list: &ItemList);
}
