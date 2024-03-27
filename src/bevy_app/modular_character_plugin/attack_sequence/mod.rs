pub mod draw_direction_ray_gizmos;
mod move_entities_toward_destinations;
use std::time::Duration;

use super::{
    spawn_character::{AnimationManagerComponent, CharacterIdComponent, MainSkeletonEntity},
    Animations, CharactersById, CombatantsExecutingAttacks,
};
use crate::{
    bevy_app::utils::link_animations::AnimationEntityLink, comm_channels::StartAttackSequenceEvent,
    frontend_common::AttackCommand,
};
use bevy::{prelude::*, reflect::List};

// get home location of target
// set destination on combatant
// set direction vector
// set running animation on combatant
// on update
// - move translation toward direction vector
// - check if within threshold distance of destination
// - if so, start sword_slash animation and set current_animation component to sword_slash
// - if current animation is sword_slash check if .is_finished()
// - if so, set destination to home_location
pub fn handle_attack_sequence_start_requests(
    combatants_by_id: Res<CharactersById>,
    mut combatants: Query<(
        &CharacterIdComponent,
        &MainSkeletonEntity,
        &mut AnimationManagerComponent,
    )>,
    animation_player_links: Query<&AnimationEntityLink>,
    mut animation_players: Query<&mut AnimationPlayer>,
    transforms: Query<&mut Transform>,
    mut attack_sequence_commands_event_reader: EventReader<StartAttackSequenceEvent>,
    animations: Res<Animations>,
    mut combatants_executing_attacks: ResMut<CombatantsExecutingAttacks>,
) {
    for event in attack_sequence_commands_event_reader.read() {
        let AttackCommand {
            combatant_id,
            target_id,
        } = event.0;

        // get locations of combatant and target
        let target_entity = combatants_by_id
            .0
            .get(&target_id)
            .expect("to have the entity");
        let (_, target_skeleton_entity, _) = combatants
            .get(*target_entity)
            .expect("to have the combatant");
        let target_transform = transforms
            .get(target_skeleton_entity.0)
            .expect("to have the transform")
            .clone();
        let combatant_entity = combatants_by_id
            .0
            .get(&combatant_id)
            .expect("to have the entity");
        let (_, combatant_skeleton_entity, mut combatant_animation_manager) = combatants
            .get_mut(*combatant_entity)
            .expect("to have the combatant");
        combatant_animation_manager.destination = Some(target_transform.clone());
        combatant_animation_manager.current_animation_name = "Run".to_string();
        combatants_executing_attacks.0.insert(combatant_id);

        let animation_player_link = animation_player_links
            .get(combatant_skeleton_entity.0)
            .expect("to have linked the skeleton to it's animation player");
        let mut animation_player = animation_players
            .get_mut(animation_player_link.0)
            .expect("to have a valid animation player entity in the link");
        let animation_handle = animations.0.get("Run").expect("to have a run animation");
        animation_player
            .play_with_transition(animation_handle.clone(), Duration::from_millis(500))
            .repeat();
    }
}
