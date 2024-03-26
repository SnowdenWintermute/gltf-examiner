use super::Animations;
use crate::{
    bevy_app::asset_loader_plugin::{AssetLoaderState, MyAssets},
    comm_channels::{BevyTransmitter, MessageFromBevy},
};
use bevy::{gltf::Gltf, prelude::*};
use std::collections::HashSet;

pub fn register_animations(
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    mut animations: ResMut<Animations>,
    mut next_state: ResMut<NextState<AssetLoaderState>>,
    transmitter: ResMut<BevyTransmitter>,
) {
    let handle = asset_pack
        .main_skeletons_with_animations
        .get("main_skeleton.glb")
        .expect("to have loaded the main_skeleton.glb");
    let gltf = assets_gltf
        .get(handle)
        .expect("to have loaded the main_skeleton.glb");

    let mut animation_names_for_yew: HashSet<String> = HashSet::new();

    for named_animation in gltf.named_animations.iter() {
        info!("inserting animation: {}", named_animation.0);
        animations.0.insert(
            named_animation.0.clone(),
            gltf.named_animations[named_animation.0].clone(),
        );
        animation_names_for_yew.insert(named_animation.0.clone());
    }

    transmitter.0.send(MessageFromBevy::AnimationsAvailable(
        animation_names_for_yew,
    ));

    next_state.set(AssetLoaderState::Done)
}
