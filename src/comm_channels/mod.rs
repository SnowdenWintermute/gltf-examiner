pub mod comm_channel_bevy_plugin;
use bevy::prelude::*;
use tokio::sync::broadcast;
// use crossbeam_channel::Receiver;
// use crossbeam_channel::Sender;
use broadcast::Receiver;
use broadcast::Sender;

// YEW MESSAGES
#[derive(Debug, Clone)]
pub enum MessageFromYew {
    Counter(CounterEvent),
}
#[derive(Clone, Debug, Event)]
pub struct CounterEvent {
    pub value: i32,
}
// BEVY MESSAGES
#[derive(Debug, Clone)]
pub enum MessageFromBevy {
    Text(String),
}
// CHANNELS
#[derive(Clone, Resource, Deref)]
pub struct YewTransmitter(pub Sender<MessageFromYew>);
#[derive(Resource, Deref, DerefMut)]
pub struct YewReceiver(pub Receiver<MessageFromBevy>);

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
    //
}
