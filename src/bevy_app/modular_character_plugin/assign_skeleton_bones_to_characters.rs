use super::{
    assemble_parts::get_main_skeleton_bones_and_armature::get_main_skeleton_bones_and_armature,
    spawn_character::MainSkeletonBonesAndArmature, spawn_scenes::SceneLoaded, CharactersById,
    SkeletonsAwaitingCharacterAssignment,
};
use bevy::{prelude::*, scene::SceneInstance};

pub fn assign_skeleton_bones_to_characters(
    mut commands: Commands,
    scene_manager: Res<SceneSpawner>,
    unloaded_instances: Query<(Entity, &SceneInstance), Without<SceneLoaded>>,
    mut skeletons_awaiting_character_assignment: ResMut<SkeletonsAwaitingCharacterAssignment>,
    characters_by_id: Res<CharactersById>,
    all_entities_with_children: Query<&Children>,
    names: Query<&Name>,
) {
    //   - loop unspawned skeletons and check for readiness
    let mut character_ids_of_skeletons_readied = Vec::new();
    for (character_id, skeleton_entity) in skeletons_awaiting_character_assignment.0.iter() {
        if let Ok((entity, scene_instance)) = unloaded_instances.get(*skeleton_entity) {
            if scene_manager.instance_is_ready(**scene_instance) {
                // mark as loaded
                commands.entity(entity).insert(SceneLoaded);
                // remove skeleton entity from skeletons_awaiting_character_assignment resource
                character_ids_of_skeletons_readied.push(*character_id);
                // for any spawned, add its bones to the corresponding character
                let (main_skeleton_bones, main_armature_entity) =
                    get_main_skeleton_bones_and_armature(
                        &skeleton_entity,
                        &all_entities_with_children,
                        &names,
                    );
                let character_entity = characters_by_id
                    .0
                    .get(character_id)
                    .expect("for this character to exist");
                let mut character_entity_commands = commands.entity(*character_entity);
                character_entity_commands.insert(MainSkeletonBonesAndArmature(
                    main_skeleton_bones,
                    main_armature_entity,
                ));
            }
        }
    }

    for character_id in character_ids_of_skeletons_readied {
        skeletons_awaiting_character_assignment
            .0
            .remove(&character_id);
    }
}
