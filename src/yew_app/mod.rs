mod app;
use self::app::App;
use crate::comm_channels::BevyTransmitter;
use crate::comm_channels::YewReceiver;
use crate::comm_channels::YewTransmitter;
use crate::SharedState;
use std::sync::Arc;
use std::sync::Mutex;
use yew::prelude::*;

#[derive(Properties)]
pub struct Props {
    pub shared: Arc<Mutex<SharedState>>,
    pub transmitter: YewTransmitter,
    pub receiver: YewReceiver,
    pub bevy_transmitter: BevyTransmitter,
}

impl PartialEq for Props {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

pub fn yew_main(
    yew_transmitter: YewTransmitter,
    yew_receiver: YewReceiver,
    bevy_transmitter: BevyTransmitter,
    shared: Arc<Mutex<SharedState>>,
) {
    let document = gloo::utils::document();
    let root = document.query_selector("#yew").unwrap().unwrap();
    let props = Props {
        transmitter: yew_transmitter,
        receiver: yew_receiver,
        bevy_transmitter,
        shared,
    };
    yew::Renderer::<App>::with_root_and_props(root, props).render();
}
