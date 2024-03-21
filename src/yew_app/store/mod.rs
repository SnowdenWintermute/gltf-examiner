use yewdux::Store;

#[derive(Store, Default, PartialEq, Clone)]
pub struct AppStore {
    pub value: usize,
}
