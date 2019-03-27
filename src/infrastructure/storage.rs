use crate::domain::ItemList;

#[allow(clippy::ptr_arg)]
pub trait Storage {
    fn read(&self) -> ItemList;
    fn write(&mut self, list: ItemList);
}
