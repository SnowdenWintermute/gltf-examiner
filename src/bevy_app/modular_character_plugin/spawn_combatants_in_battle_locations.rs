// use super::{
//     spawn_combatant::spawn_combatant, CombatantsById, HomeLocation,
//     SkeletonsAwaitingCombatantAssignment,
// };
// use crate::{bevy_app::asset_loader_plugin::MyAssets, comm_channels::BevyTransmitter};
// use bevy::{gltf::Gltf, prelude::*};
// use std::f32::consts::PI;

// pub fn spawn_combatants_in_battle_locations(
//     mut commands: Commands,
//     asset_pack: Res<MyAssets>,
//     assets_gltf: Res<Assets<Gltf>>,
//     mut characters_by_id: ResMut<CombatantsById>,
//     mut skeletons_awaiting_combatant_assignment: ResMut<SkeletonsAwaitingCombatantAssignment>,
//     transmitter: ResMut<BevyTransmitter>,
// ) {
//     let mut home_location = HomeLocation(Transform::from_xyz(0.0, 0.0, -1.5));
//     home_location.0.rotate_y(PI);

//     for i in 0..=2 as u32 {
//         spawn_combatant(
//             &mut commands,
//             &asset_pack,
//             &assets_gltf,
//             &mut characters_by_id,
//             &mut skeletons_awaiting_combatant_assignment,
//             home_location.clone(),
//             i,
//             &transmitter,
//         );
//         home_location.0.translation.x += 1.0;
//     }

//     let mut home_location = HomeLocation(Transform::from_xyz(0.0, 0.0, 1.5));

//     for i in 3..=5 as u32 {
//         spawn_combatant(
//             &mut commands,
//             &asset_pack,
//             &assets_gltf,
//             &mut characters_by_id,
//             &mut skeletons_awaiting_combatant_assignment,
//             home_location.clone(),
//             i,
//             &transmitter,
//         );
//         home_location.0.translation.x += 1.0;
//     }
// }
