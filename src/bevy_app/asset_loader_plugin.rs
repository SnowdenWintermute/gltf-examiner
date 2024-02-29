use bevy::gltf::Gltf;
use bevy::prelude::*;
use gloo::console::log;

use crate::comm_channels::BevyTransmitter;
use crate::comm_channels::MessageFromBevy;

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, States)]
pub enum AssetLoaderState {
    #[default]
    Loading,
    Done,
}

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AssetLoaderState>()
            .add_systems(OnEnter(AssetLoaderState::Loading), load_assets)
            .add_systems(
                Update,
                check_for_load_complete.run_if(in_state(AssetLoaderState::Loading)),
            );
    }
}

#[derive(Resource, Debug)]
pub struct AssetPack(pub Handle<Gltf>);

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("Cleric.gltf");
    commands.insert_resource(AssetPack(handle));
}

fn check_for_load_complete(
    asset_pack: Res<AssetPack>,
    bevy_transmitter: Res<BevyTransmitter>,
    mut next_state: ResMut<NextState<AssetLoaderState>>,
    mut asset_events: EventReader<AssetEvent<Gltf>>,
) {
    for event in asset_events.read() {
        if event.is_loaded_with_dependencies(asset_pack.0.clone()) {
            log!("asset loaded (bevy log)");
            let send_message_result =
                bevy_transmitter.send(MessageFromBevy::Text(String::from("asset loaded")));
            match send_message_result {
                Ok(_) => log!("sent message to yew"),
                Err(error) => log!(format!("send message error: {:#?}", error)),
            };

            next_state.set(AssetLoaderState::Done)
        }
    }
}
