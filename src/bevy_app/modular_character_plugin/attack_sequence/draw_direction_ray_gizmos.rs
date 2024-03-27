use crate::bevy_app::modular_character_plugin::spawn_character::MainSkeletonEntity;
use bevy::prelude::*;

pub fn draw_directional_gizmos(
    combatants: Query<&MainSkeletonEntity>,
    transforms: Query<&Transform>,
    mut gizmos: Gizmos,
) {
    for main_skeleton in combatants.iter() {
        if let Ok(transform) = transforms.get(main_skeleton.0) {
            gizmos.ray(
                transform.translation,
                transform.forward().into(),
                Color::WHITE,
            );
        }
        //
    }
}
