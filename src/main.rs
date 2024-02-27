use bevy_app::bevy_main;
use comm_channels::comm_channel_bevy_plugin::CommChannelPlugin;
use comm_channels::create_comm_channels;
use std::sync::Arc;
use std::sync::Mutex;
use yew_app::yew_main;
mod bevy_app;
mod comm_channels;
mod yew_app;

pub struct SharedState {
    pub name: String,
}

pub type Shared<T> = Arc<Mutex<T>>;

fn main() {
    let shared_state = Arc::new(Mutex::new(SharedState {
        name: "This can be used for shared state".to_string(),
    }));
    let (yew_channels, bevy_channels) = create_comm_channels();
    let comm_channel_bevy_plugin = CommChannelPlugin::new(bevy_channels.0, bevy_channels.1);

    yew_main();
    bevy_main(comm_channel_bevy_plugin, shared_state);
    //
}
