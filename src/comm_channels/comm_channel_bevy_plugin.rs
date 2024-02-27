use super::BevyReceiver;
use super::BevyTransmitter;
use super::CounterEvent;
use super::MessageFromYew;
use bevy::prelude::*;

#[derive(Clone)]
pub struct CommChannelPlugin {
    bevy_transmitter: BevyTransmitter,
    bevy_receiver: BevyReceiver,
}

impl CommChannelPlugin {
    pub fn new(bevy_transmitter: BevyTransmitter, bevy_receiver: BevyReceiver) -> Self {
        CommChannelPlugin {
            bevy_transmitter,
            bevy_receiver,
        }
    }
}

impl Plugin for CommChannelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.bevy_receiver.clone())
            .insert_resource(self.bevy_transmitter.clone())
            .init_resource::<Events<CounterEvent>>()
            .add_systems(PreUpdate, handle_yew_messages);
    }
}

fn handle_yew_messages(
    bevy_receiver: Res<BevyReceiver>,
    mut counter_event_writer: EventWriter<CounterEvent>,
) {
    for message_from_yew in bevy_receiver.try_iter() {
        match message_from_yew {
            MessageFromYew::Counter(event) => {
                counter_event_writer.send(event);
            }
        }
    }
}
