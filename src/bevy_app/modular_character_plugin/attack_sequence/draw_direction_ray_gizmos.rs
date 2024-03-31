use std::f32::consts::PI;

use crate::bevy_app::modular_character_plugin::animation_manager_component::AnimationManagerComponent;
use crate::bevy_app::modular_character_plugin::spawn_combatant::HitboxRadius;
use crate::bevy_app::modular_character_plugin::spawn_combatant::MainSkeletonEntity;
use crate::bevy_app::modular_character_plugin::update_scene_aabbs::SceneAabb;
use bevy::prelude::*;

pub fn draw_directional_gizmos(
    combatants: Query<(
        &MainSkeletonEntity,
        &AnimationManagerComponent,
        &HitboxRadius,
    )>,
    scene_aabbs: Query<&SceneAabb>,
    transforms: Query<&Transform>,
    mut gizmos: Gizmos,
    // mut config_store: ResMut<GizmoConfigStore>,
) {
    // config_store.config_mut::<AabbGizmoConfigGroup>().1.draw_all ^= true;
    // scene aabbs
    // for scene_aabb in scene_aabbs.iter() {
    //     gizmos.rect(
    //         scene_aabb.min,
    //         Quat::IDENTITY,
    //         Vec2::from_array([0.1, 0.1]),
    //         Color::BLUE,
    //     );
    //     // let quat = Quat::from_rotation_x(PI / 2.0);
    //     // let quat = Quat::IDENTITY;
    //     // gizmos.rect(
    //     //     scene_aabb.max - scene_aabb.min / 2.0,
    //     //     quat,
    //     //     Vec2::from_array([0.5, 0.5]),
    //     //     Color::GREEN,
    //     // );
    //     gizmos.rect(
    //         scene_aabb.max,
    //         Quat::IDENTITY,
    //         Vec2::from_array([0.2, 0.2]),
    //         Color::RED,
    //     );
    // }
    // skeletons
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

            // gizmos.cuboid(
            //     Transform::from_translation(Vec3::Y * 0.5).with_scale(Vec3::splat(1.25)),
            //     Color::BLACK,
            // );

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
