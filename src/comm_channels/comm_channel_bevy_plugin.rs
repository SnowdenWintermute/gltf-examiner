use super::BevyReceiver;
use super::BevyTransmitter;
use super::CounterEvent;
use super::MessageFromBevy;
use super::MessageFromYew;
use super::YewReceiver;
use super::YewTransmitter;
use bevy::prelude::*;
use rand::Rng;

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

#[derive(Resource, Default)]
pub struct SentMessageCounterResource {
    pub value: i32,
}

impl Plugin for CommChannelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BevyReceiver(self.yew_transmitter.subscribe()))
            .insert_resource(self.bevy_transmitter.clone())
            .init_resource::<SentMessageCounterResource>()
            .init_resource::<Events<CounterEvent>>()
            .add_systems(PreUpdate, handle_yew_messages);
    }
}

fn handle_yew_messages(
    mut bevy_receiver: ResMut<BevyReceiver>,
    transmitter: ResMut<BevyTransmitter>,
    mut counter_event_writer: EventWriter<CounterEvent>,
    mut sent_message_count: ResMut<SentMessageCounterResource>,
) {
    info!("checking for yew messages");
    let mut rng = rand::thread_rng();
    let randomnum = rng.gen_range(0..1000);
    sent_message_count.value += 1;
    let result = transmitter.send(MessageFromBevy::Text(format!(
        "{}",
        sent_message_count.value
    )));

    if let Ok(message_from_yew) = bevy_receiver.try_recv() {
        match message_from_yew {
            MessageFromYew::Counter(event) => {
                counter_event_writer.send(event);
            }
        }
    }
}
