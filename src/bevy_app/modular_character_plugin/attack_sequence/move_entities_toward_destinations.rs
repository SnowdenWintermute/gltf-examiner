use std::time::Duration;

use crate::bevy_app::{
    modular_character_plugin::{
        spawn_character::{AnimationManagerComponent, MainSkeletonEntity},
        Animations, CharactersById, CombatantsExecutingAttacks, HomeLocation,
    },
    utils::link_animations::AnimationEntityLink,
};
use bevy::prelude::*;
use js_sys::Date;

const TIME_TO_ROTATE: u64 = 500;
const TIME_TO_STRIKE: u64 = 1500;

pub fn move_entities_toward_destinations(
    combatants_by_id: Res<CharactersById>,
    mut combatants: Query<(
        &MainSkeletonEntity,
        &mut AnimationManagerComponent,
        &HomeLocation,
    )>,
    mut transforms: Query<&mut Transform>,
    combatants_executing_attacks: ResMut<CombatantsExecutingAttacks>,
    animation_player_links: Query<&AnimationEntityLink>,
    mut animation_players: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
) {
    for combatant_id in combatants_executing_attacks.0.iter() {
        let combatant_entity = combatants_by_id
            .0
            .get(combatant_id)
            .expect("to have the combatant");
        let (skeleton_entity, mut animation_manager, home_location) = combatants
            .get_mut(*combatant_entity)
            .expect("to have the combatant");
        if let Some(destination) = animation_manager.destination {
            let mut combatant_transform = transforms
                .get_mut(skeleton_entity.0)
                .expect("to have the transform");
            let up = *combatant_transform.up().clone();
            let target_rotation = combatant_transform
                .looking_at(destination.translation, up)
                .rotation;

            let current_time = Date::new_0().get_time() as u64;

            let time_started = animation_manager
                .time_started
                .expect("to have marked the start time");
            let elapsed = current_time - time_started;
            let clamped_elapsed = std::cmp::min(elapsed, TIME_TO_ROTATE);
            let clamped_translation_time = std::cmp::min(elapsed, TIME_TO_STRIKE);
            let percent_of_complete_rotation = clamped_elapsed as f32 / TIME_TO_ROTATE as f32;
            let percent_of_complete_translation =
                clamped_translation_time as f32 / TIME_TO_STRIKE as f32;

            combatant_transform.rotation = home_location
                .0
                .rotation
                .lerp(target_rotation, percent_of_complete_rotation);
            combatant_transform.translation = home_location
                .0
                .translation
                .lerp(destination.translation, percent_of_complete_translation);

            if percent_of_complete_translation >= 0.8
                && animation_manager.current_animation_name != "Sword_Slash"
            {
                animation_manager.current_animation_name = "Sword_Slash".to_string();
                let animation_player_link = animation_player_links
                    .get(skeleton_entity.0)
                    .expect("to have an animation player link");
                let mut animation_player = animation_players
                    .get_mut(animation_player_link.0)
                    .expect("to have a player");
                let animation_handle = animations
                    .0
                    .get("Sword_Slash")
                    .expect("to have this animation");
                animation_player
                    .play_with_transition(animation_handle.clone(), Duration::from_millis(500));

                // anim
                animation_manager.destination = None;
            }
        } else if animation_manager.current_animation_name == "Sword_Slash" {
        }
    }
}
