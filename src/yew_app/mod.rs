mod app;
mod battle_combatant_spawner;
mod character_part_selection_menu;
mod character_select_input;
mod character_selection_menu;
mod execute_attack_sequence_button;
mod select_animation_button;
mod select_character_part_button;
mod store;
use self::app::App;
use crate::comm_channels::BevyTransmitter;
use crate::comm_channels::YewTransmitter;
use crate::SharedState;
use std::sync::Arc;
use std::sync::Mutex;
use yew::prelude::*;

#[derive(Properties)]
pub struct Props {
    pub shared: Arc<Mutex<SharedState>>,
    pub transmitter: YewTransmitter,
    pub bevy_transmitter: BevyTransmitter,
}

impl PartialEq for Props {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

pub fn yew_main(
    yew_transmitter: YewTransmitter,
    bevy_transmitter: BevyTransmitter,
    shared: Arc<Mutex<SharedState>>,
) {
    let document = gloo::utils::document();
    let root = document.query_selector("#yew").unwrap().unwrap();
    let props = Props {
        transmitter: yew_transmitter,
        bevy_transmitter,
        shared,
    };
    yew::Renderer::<App>::with_root_and_props(root, props).render();
}
