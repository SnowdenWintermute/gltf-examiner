use crate::{
    bevy_app::modular_character_plugin::{
        animation_manager_component::{ActionSequenceStates, AnimationManagerComponent},
        Animations,
    },
    frontend_common::animation_names::{RUN_BACK, SWORD_SLASH},
};
use bevy::{math::u64, prelude::*};
use std::time::Duration;

pub const SWORD_SLASH_PERCENT_COMPLETE_TRANSITION_THRESHOLD: f32 = 0.65;

pub fn process_combatant_swinging_weapons(
    animation_manager: &mut AnimationManagerComponent,
    home_location: &Transform,
    animation_player: &mut AnimationPlayer,
    animations: &Res<Animations>,
    assets_animation_clips: &Res<Assets<AnimationClip>>,
    current_time: u64,
) {
    // - if duration threshold passed, activate returning
    let animation_handle = animations
        .0
        .get(SWORD_SLASH)
        .expect("to have this animation registered");
    let animation_clip = assets_animation_clips
        .get(animation_handle)
        .expect("to have the clip");
    let percent_completed = animation_player.elapsed() / animation_clip.duration();
    if percent_completed >= SWORD_SLASH_PERCENT_COMPLETE_TRANSITION_THRESHOLD {
        animation_manager
            .active_states
            .remove(&ActionSequenceStates::Swinging);

        animation_manager
            .active_states
            .insert(ActionSequenceStates::Returning, Some(current_time));
        // set new destination as home location and save prev location
        animation_manager.last_location = animation_manager.destination.take();
        animation_manager.destination = Some(home_location.clone());

        let animation_handle = animations.0.get(RUN_BACK).expect("to have this animation");
        animation_player
            .play_with_transition(animation_handle.clone(), Duration::from_millis(500))
            .repeat();
    }
}
