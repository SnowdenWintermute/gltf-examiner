use super::{spawn_scenes::SceneName, Animations};
use crate::bevy_app::utils::link_animations::AnimationEntityLink;
use bevy::prelude::*;

pub fn run_animations(
    mut animation_player_query: Query<&mut AnimationPlayer>,
    scene_and_animation_player_link_query: Query<
        (&SceneName, &AnimationEntityLink),
        Added<AnimationEntityLink>,
    >,
    animations: Res<Animations>,
) {
    for (_, animation_player_entity_link) in scene_and_animation_player_link_query.iter() {
        let mut animation_player = animation_player_query
            .get_mut(animation_player_entity_link.0)
            .expect("to have an animation player on the main skeleton");

        info!("RUNNING ANIMATIONS");

        animation_player
            .play(
                animations
                    .0
                    .get("Sword_Slash")
                    .expect("to have an animation by this name")
                    .clone_weak(),
            )
            .repeat()
            .set_speed(0.5);
    }
}
