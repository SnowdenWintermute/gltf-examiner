pub mod comm_channel_bevy_plugin;
use std::collections::HashSet;

use crate::bevy_app::modular_character_plugin::CharacterId;
use crate::frontend_common::CharacterPartSelection;
use crate::frontend_common::PartsByName;
use bevy::prelude::*;
use broadcast::Receiver;
use broadcast::Sender;
use tokio::sync::broadcast;

// YEW MESSAGES
#[derive(Debug, Clone)]
pub enum MessageFromYew {
    SelectCharacterPart(CharacterPartSelection),
    SpawnCharacter(CharacterId),
}
#[derive(Clone, Debug, Event)]
pub struct TextFromYewEvent {
    pub text: String,
}
#[derive(Clone, Debug, Event)]
pub struct CharacterPartSelectionEvent(pub CharacterPartSelection);

#[derive(Clone, Debug, Event)]
pub struct CharacterSpawnEvent(pub CharacterId);

// BEVY MESSAGES
#[derive(Debug, Clone, PartialEq)]
pub enum MessageFromBevy {
    PartNames(PartsByName),
    AnimationsAvailable(HashSet<String>),
}
// CHANNELS
#[derive(Clone, Resource, Deref)]
pub struct YewTransmitter(pub Sender<MessageFromYew>);
#[derive(Resource, Deref, DerefMut)]
pub struct YewReceiver(pub Receiver<MessageFromBevy>);

// required so it can be passed as yew Props
impl PartialEq for YewTransmitter {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[derive(Resource, Deref, DerefMut, Clone)]
pub struct BevyTransmitter(pub Sender<MessageFromBevy>);
#[derive(Resource, Deref, DerefMut)]
pub struct BevyReceiver(pub Receiver<MessageFromYew>);

pub fn create_comm_channels() -> (
    (YewTransmitter, YewReceiver),
    (BevyTransmitter, BevyReceiver),
) {
    let (yew_transmitter, bevy_receiver) = broadcast::channel(50);
    let (bevy_transmitter, yew_receiver) = broadcast::channel(50);

    (
        (YewTransmitter(yew_transmitter), YewReceiver(yew_receiver)),
        (
            BevyTransmitter(bevy_transmitter),
            BevyReceiver(bevy_receiver),
        ),
    )
}
