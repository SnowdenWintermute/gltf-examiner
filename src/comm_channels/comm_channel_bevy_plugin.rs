use super::BevyReceiver;
use super::BevyTransmitter;
use super::CharacterPartSelectionEvent;
use super::CharacterSpawnEvent;
use super::MessageFromYew;
use super::SelectAnimationEvent;
use super::YewTransmitter;
use bevy::prelude::*;

pub struct CommChannelPlugin {
    bevy_transmitter: BevyTransmitter,
    yew_transmitter: YewTransmitter,
}

impl CommChannelPlugin {
    pub fn new(bevy_transmitter: BevyTransmitter, yew_transmitter: YewTransmitter) -> Self {
        CommChannelPlugin {
            bevy_transmitter,
            yew_transmitter,
        }
    }
}

impl Plugin for CommChannelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BevyReceiver(self.yew_transmitter.subscribe()))
            .insert_resource(self.bevy_transmitter.clone())
            .init_resource::<Events<CharacterPartSelectionEvent>>()
            .init_resource::<Events<CharacterSpawnEvent>>()
            .init_resource::<Events<SelectAnimationEvent>>()
            .add_systems(PreUpdate, handle_yew_messages);
    }
}

fn handle_yew_messages(
    mut bevy_receiver: ResMut<BevyReceiver>,
    mut part_selection_event_writer: EventWriter<CharacterPartSelectionEvent>,
    mut spawn_character_event_writer: EventWriter<CharacterSpawnEvent>,
    mut select_animation_event_writer: EventWriter<SelectAnimationEvent>,
) {
    if let Ok(message_from_yew) = bevy_receiver.try_recv() {
        match message_from_yew {
            MessageFromYew::SelectCharacterPart(part_selection) => {
                part_selection_event_writer.send(CharacterPartSelectionEvent(part_selection));
            }
            MessageFromYew::SpawnCharacter(character_id) => {
                spawn_character_event_writer.send(CharacterSpawnEvent(character_id));
            }
            MessageFromYew::SelectAnimation(animation_name) => {
                select_animation_event_writer.send(SelectAnimationEvent(animation_name));
            }
        }
    }
}
