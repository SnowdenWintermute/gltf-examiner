use super::BevyReceiver;
use super::BevyTransmitter;
use super::CharacterPartSelectionEvent;
use super::CharacterSpawnEvent;
use super::MessageFromBevy;
use super::MessageFromYew;
use super::TextFromYewEvent;
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

#[derive(Resource, Default)]
pub struct SentMessageCounterResource {
    pub value: i32,
}

impl Plugin for CommChannelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BevyReceiver(self.yew_transmitter.subscribe()))
            .insert_resource(self.bevy_transmitter.clone())
            .init_resource::<Events<TextFromYewEvent>>()
            .init_resource::<Events<CharacterPartSelectionEvent>>()
            .init_resource::<Events<CharacterSpawnEvent>>()
            .add_systems(PreUpdate, handle_yew_messages);
    }
}

fn handle_yew_messages(
    mut bevy_receiver: ResMut<BevyReceiver>,
    mut counter_event_writer: EventWriter<TextFromYewEvent>,
    mut part_selection_event_writer: EventWriter<CharacterPartSelectionEvent>,
    mut spawn_character_event_writer: EventWriter<CharacterSpawnEvent>,
) {
    if let Ok(message_from_yew) = bevy_receiver.try_recv() {
        match message_from_yew {
            MessageFromYew::Text(event) => {
                counter_event_writer.send(event);
            }
            MessageFromYew::SelectCharacterPart(part_selection) => {
                part_selection_event_writer.send(CharacterPartSelectionEvent(part_selection));
            }
            MessageFromYew::SpawnCharacter(character_id) => {
                spawn_character_event_writer.send(CharacterSpawnEvent(character_id));
            }
        }
    }
}
