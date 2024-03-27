use crate::bevy_app::modular_character_plugin::{
    spawn_character::{AnimationManagerComponent, CharacterIdComponent, MainSkeletonEntity},
    CharactersById, CombatantsExecutingAttacks,
};
use bevy::prelude::*;

pub fn move_entities_toward_destinations(
    // mut commands: Commands,
    combatants_by_id: Res<CharactersById>,
    mut combatants: Query<(&MainSkeletonEntity, &mut AnimationManagerComponent)>,
    mut transforms: Query<&mut Transform>,
    mut combatants_executing_attacks: ResMut<CombatantsExecutingAttacks>,
    time: Res<Time>,
) {
    for combatant_id in combatants_executing_attacks.0.iter() {
        let combatant_entity = combatants_by_id
            .0
            .get(combatant_id)
            .expect("to have the combatant");
        let (skeleton_entity, mut animation_manager) = combatants
            .get_mut(*combatant_entity)
            .expect("to have the combatant");
        if let Some(destination) = animation_manager.destination {
            let mut combatant_transform = transforms
                .get(skeleton_entity.0)
                .expect("to have the transform")
                .clone();
            let up = *combatant_transform.up().clone();

            let target_rotation = combatant_transform.looking_at(destination.translation, up);
            // combatant_transform.rotation.lerp(target_rotation, s);
        }
    }
}
