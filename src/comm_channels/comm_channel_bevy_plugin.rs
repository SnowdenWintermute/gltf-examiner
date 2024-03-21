use super::BevyReceiver;
use super::BevyTransmitter;
use super::CounterEvent;
use super::MessageFromBevy;
use super::MessageFromYew;
use super::YewReceiver;
use super::YewTransmitter;
use bevy::prelude::*;

pub struct CommChannelPlugin {
    bevy_transmitter: BevyTransmitter,
    bevy_receiver: BevyReceiver,
    yew_transmitter: YewTransmitter,
}

impl CommChannelPlugin {
    pub fn new(
        bevy_transmitter: BevyTransmitter,
        bevy_receiver: BevyReceiver,
        yew_transmitter: YewTransmitter,
    ) -> Self {
        CommChannelPlugin {
            bevy_transmitter,
            bevy_receiver,
            yew_transmitter,
        }
    }
}

impl Plugin for CommChannelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BevyReceiver(self.yew_transmitter.subscribe()))
            .insert_resource(self.bevy_transmitter.clone())
            .init_resource::<Events<CounterEvent>>()
            .add_systems(PreUpdate, handle_yew_messages);
    }
}

fn handle_yew_messages(
    mut bevy_receiver: ResMut<BevyReceiver>,
    transmitter: ResMut<BevyTransmitter>,
    mut counter_event_writer: EventWriter<CounterEvent>,
) {
    info!("checking for yew messages");
    let result = transmitter.send(MessageFromBevy::Text("ayylmao".to_string()));

    if let Ok(message_from_yew) = bevy_receiver.try_recv() {
        match message_from_yew {
            MessageFromYew::Counter(event) => {
                counter_event_writer.send(event);
            }
        }
    }
}
