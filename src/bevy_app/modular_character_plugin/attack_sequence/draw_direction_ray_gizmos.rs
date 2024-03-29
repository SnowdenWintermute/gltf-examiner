use crate::bevy_app::modular_character_plugin::{
    animation_manager_component::AnimationManagerComponent,
    spawn_character::{HitboxRadius, MainSkeletonEntity},
};
use bevy::prelude::*;

pub fn draw_directional_gizmos(
    combatants: Query<(
        &MainSkeletonEntity,
        &AnimationManagerComponent,
        &HitboxRadius,
    )>,
    transforms: Query<&Transform>,
    mut gizmos: Gizmos,
) {
    for (main_skeleton, animation_manager, hitbox_radius) in combatants.iter() {
        if let Ok(transform) = transforms.get(main_skeleton.0) {
            // for (other_skeleton, _, other_hitbox_radius) in combatants.iter() {
            // let target_transform = transforms.get(other_skeleton.0).expect("all skeletons to have transforms");
            //     let direction =
            //         (combatant_transform.translation - target_transform.translation).normalize();
            //     let destination =
            //         (target_transform.translation + direction) * other_hitbox_radius.0;
            //     let destination = Some(Transform::from_xyz(
            //         destination[0],
            //         destination[1],
            //         destination[2],
            //     ));

            //         gizmos.ray(
            //             transform.translation,
            //             transform.forward().into(),
            //             Color::RED,
            //         );
            // }

            gizmos.circle(
                transform.translation,
                Direction3d::Y,
                hitbox_radius.0 as f32,
                Color::GREEN,
            );

            gizmos.ray(
                transform.translation,
                transform.forward().into(),
                Color::WHITE,
            );
            if let Some(destination) = animation_manager.destination {
                let up = *transform.up().clone();

                gizmos.ray(
                    transform.translation,
                    transform
                        .looking_at(destination.translation, up)
                        .forward()
                        .into(),
                    Color::BLUE,
                );
            }
        }
        //
    }
}
