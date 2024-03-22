pub mod send_part_names_to_yew;
use self::send_part_names_to_yew::send_part_names_to_yew;
use crate::bevy_app::asset_loader_plugin::AssetLoaderState;
use bevy::prelude::*;

pub struct CollectPartsPlugin;

impl Plugin for CollectPartsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AssetLoaderState::Done), send_part_names_to_yew);
    }
}
