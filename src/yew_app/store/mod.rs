use crate::{
    comm_channels::{MessageFromBevy, YewTransmitter},
    frontend_common::PartsByName,
};
use std::collections::HashSet;
use yewdux::Store;

#[derive(Store, Default, PartialEq, Clone)]
pub struct AppStore {
    pub transmitter_option: Option<YewTransmitter>,
    pub messages_from_bevy: Vec<MessageFromBevy>,
    pub parts_available: PartsByName,
    pub animation_names: HashSet<String>,
    pub next_character_id: u32,
    pub selected_character_id: u32,
    pub character_ids: Vec<u32>,
}
