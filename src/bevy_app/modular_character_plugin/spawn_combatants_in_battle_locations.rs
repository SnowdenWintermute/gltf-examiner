use super::{
    spawn_character::spawn_character, CharactersById, HomeLocation,
    SkeletonsAwaitingCharacterAssignment,
};
use crate::bevy_app::asset_loader_plugin::MyAssets;
use bevy::{gltf::Gltf, prelude::*};
use std::f32::consts::PI;

pub fn spawn_combatants_in_battle_locations(
    mut commands: Commands,
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    mut characters_by_id: ResMut<CharactersById>,
    mut skeletons_awaiting_character_assignment: ResMut<SkeletonsAwaitingCharacterAssignment>,
) {
    let mut home_location = HomeLocation {
        position: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.5,
        },
        rotation: PI,
    };

    for i in 0..=2 as u32 {
        spawn_character(
            &mut commands,
            &asset_pack,
            &assets_gltf,
            &mut characters_by_id,
            &mut skeletons_awaiting_character_assignment,
            home_location.clone(),
            i,
        );
        home_location.position.x += 1.0;
    }

    let mut home_location = HomeLocation {
        position: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.5,
        },
        rotation: 0.0,
    };

    for i in 3..=5 as u32 {
        spawn_character(
            &mut commands,
            &asset_pack,
            &assets_gltf,
            &mut characters_by_id,
            &mut skeletons_awaiting_character_assignment,
            home_location.clone(),
            i,
        );
        home_location.position.x += 1.0;
    }
}
