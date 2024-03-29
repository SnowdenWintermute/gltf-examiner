use super::{
    process_combatant_approaching_melee_target::process_combatant_approaching_melee_target,
    process_combatant_returning_to_home_position::process_combatant_returning_to_home_position,
    process_combatant_swinging_weapons::process_combatant_swinging_weapons,
};
use crate::bevy_app::{
    modular_character_plugin::{
        animation_manager_component::{ActionSequenceStates, AnimationManagerComponent},
        spawn_character::MainSkeletonEntity,
        Animations, CharactersById, CombatantsExecutingAttacks, HomeLocation,
    },
    utils::link_animations::AnimationEntityLink,
};
use bevy::prelude::*;
use js_sys::Date;

pub const PERCENT_DISTANCE_TO_START_WEAPON_SWING: f32 = 0.8;

pub fn process_active_animation_states(
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
    assets_animation_clips: Res<Assets<AnimationClip>>,
) {
    for combatant_id in combatants_executing_attacks.0.iter() {
        let combatant_entity = combatants_by_id
            .0
            .get(combatant_id)
            .expect("to have the combatant");
        let (skeleton_entity, mut animation_manager, home_location) = combatants
            .get_mut(*combatant_entity)
            .expect("to have the combatant");
        let mut skeleton_entity_transform = transforms
            .get_mut(skeleton_entity.0)
            .expect("skeleton to have a tranform");
        let current_time = Date::new_0().get_time() as u64;

        let animation_player_link = animation_player_links
            .get(skeleton_entity.0)
            .expect("to have linked the skeleton to it's animation player");
        let mut animation_player = animation_players
            .get_mut(animation_player_link.0)
            .expect("to have a valid animation player entity in the link");

        let active_states = animation_manager.active_states.clone();

        // info!("active states: {:#?}", active_states);
        for (active_state, time_started_option) in active_states {
            match active_state {
                ActionSequenceStates::ApproachingTarget => {
                    process_combatant_approaching_melee_target(
                        &mut skeleton_entity_transform,
                        &mut animation_manager,
                        &home_location.0,
                        current_time - time_started_option.expect("to have marked the start time"),
                        &mut animation_player,
                        &animations,
                        current_time,
                    );
                }
                ActionSequenceStates::Swinging => process_combatant_swinging_weapons(
                    &mut animation_manager,
                    &home_location.0,
                    &mut animation_player,
                    &animations,
                    &assets_animation_clips,
                    current_time,
                ),
                ActionSequenceStates::Returning => process_combatant_returning_to_home_position(
                    &mut skeleton_entity_transform,
                    &mut animation_manager,
                    &home_location.0,
                    current_time - time_started_option.expect("to have marked the start time"),
                    &mut animation_player,
                    &animations,
                    current_time,
                ),
                ActionSequenceStates::Recentering => {
                    // recentering
                    // - start playing recentering animation if not already
                    // - if threshold passed, start idle animation
                    // - if recentering animation duration complete, deactivate recentering
                }
            }
        }
    }
}
