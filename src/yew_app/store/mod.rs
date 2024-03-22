use crate::frontend_common::PartsByName;
use yewdux::Store;

#[derive(Store, Default, PartialEq, Clone)]
pub struct AppStore {
    pub parts_available: PartsByName,
}
