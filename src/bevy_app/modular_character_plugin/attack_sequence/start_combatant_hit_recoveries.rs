use std::time::Duration;

use crate::bevy_app::modular_character_plugin::animation_manager_component::ActionSequenceStates;
use crate::bevy_app::modular_character_plugin::animation_manager_component::AnimationManagerComponent;
use crate::bevy_app::modular_character_plugin::spawn_character::MainSkeletonEntity;
use crate::bevy_app::modular_character_plugin::Animations;
use crate::bevy_app::modular_character_plugin::CharactersById;
use crate::bevy_app::modular_character_plugin::CombatantsExecutingAttacks;
use crate::bevy_app::modular_character_plugin::HitRecoveryActivationEvent;
use crate::bevy_app::utils::link_animations::AnimationEntityLink;
use crate::frontend_common::animation_names::HIT_RECOVERY;
use bevy::prelude::*;
use js_sys::Date;

pub fn start_combatant_hit_recoveries(
    combatants_by_id: Res<CharactersById>,
    mut combatants: Query<(&MainSkeletonEntity, &mut AnimationManagerComponent)>,
    mut combatants_executing_attacks: ResMut<CombatantsExecutingAttacks>,
    animation_player_links: Query<&AnimationEntityLink>,
    mut animation_players: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
    mut hit_recovery_activation_event_reader: EventReader<HitRecoveryActivationEvent>,
) {
    let current_time = Date::new_0().get_time() as u64;
    for event in hit_recovery_activation_event_reader.read() {
        info!("read hit recovery event");
        let HitRecoveryActivationEvent(targets_and_damages) = event;
        for (target_id, damage) in targets_and_damages {
            // TODO: change this to combatants_animating and remove it when they're done
            combatants_executing_attacks.0.insert(*target_id);
            let target_entity = combatants_by_id
                .0
                .get(target_id)
                .expect("to have the entity");
            let (skeleton_entity, mut animation_manager) = combatants
                .get_mut(*target_entity)
                .expect("to have the combatant");
            animation_manager
                .active_states
                .insert(ActionSequenceStates::HitRecovery, Some(current_time));

            let animation_player_link = animation_player_links
                .get(skeleton_entity.0)
                .expect("to have linked the skeleton to it's animation player");
            let mut animation_player = animation_players
                .get_mut(animation_player_link.0)
                .expect("to have a valid animation player entity in the link");

            let animation_handle = animations
                .0
                .get(HIT_RECOVERY)
                .expect("to have a run animation");
            animation_player
                .start_with_transition(animation_handle.clone(), Duration::from_millis(500));
        }
    }
}
