use crate::domain::ItemList;

#[allow(clippy::ptr_arg)]
pub trait Storage: Send + Sync {
    fn read(&self) -> ItemList;
    fn write(&self, list: &ItemList);
}
