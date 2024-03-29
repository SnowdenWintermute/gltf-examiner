use crate::comm_channels::MessageFromBevy;
use crate::comm_channels::YewTransmitter;
use crate::frontend_common::PartsByName;
use std::collections::HashSet;
use yewdux::Store;

#[derive(Store, PartialEq, Clone)]
pub struct AppStore {
    pub transmitter_option: Option<YewTransmitter>,
    pub messages_from_bevy: Vec<MessageFromBevy>,
    pub parts_available: PartsByName,
    pub animation_names: HashSet<String>,
    pub next_character_id: u32,
    pub selected_character_id: u32,
    pub selected_target_id: u32,
    pub character_ids: Vec<u32>,
    pub bevy_assets_loaded: bool,
}

impl Default for AppStore {
    fn default() -> Self {
        Self {
            selected_target_id: 5,
            transmitter_option: None,
            messages_from_bevy: Vec::new(),
            parts_available: PartsByName::default(),
            animation_names: HashSet::new(),
            next_character_id: 0,
            selected_character_id: 0,
            character_ids: Vec::new(),
            bevy_assets_loaded: false,
        }
    }
}
