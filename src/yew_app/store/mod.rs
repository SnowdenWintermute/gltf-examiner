use crate::{comm_channels::YewTransmitter, frontend_common::PartsByName};
use yewdux::Store;

#[derive(Store, Default, PartialEq, Clone)]
pub struct AppStore {
    pub transmitter_option: Option<YewTransmitter>,
    pub parts_available: PartsByName,
    pub next_character_id: u32,
    pub selected_character_id: u32,
    pub character_ids: Vec<u32>,
}
