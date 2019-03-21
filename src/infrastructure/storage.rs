use crate::domain::ItemList;

#[allow(clippy::ptr_arg)]
pub trait Storage {
    fn read(self: &Self) -> ItemList;
    fn write(self: &Self, list: &ItemList);
}
