use super::{
    assemble_parts::get_main_skeleton_bones_and_armature::get_main_skeleton_bones_and_armature,
    spawn_character::{CharacterName, MainSkeletonBonesAndArmature, MainSkeletonEntity},
    spawn_scenes::{SceneLoaded, SceneName, SpawnScenesState},
};
use bevy::{prelude::*, scene::SceneInstance};

pub fn assign_skeleton_bones_to_character(
    mut commands: Commands,
    scene_manager: Res<SceneSpawner>,
    unloaded_instances: Query<(Entity, &SceneInstance, &SceneName), Without<SceneLoaded>>,
    mut characters: Query<(Entity, &CharacterName, &MainSkeletonEntity)>,
    all_entities_with_children: Query<&Children>,
    names: Query<&Name>,
    mut next_state: ResMut<NextState<SpawnScenesState>>,
) {
    for (entity, instance, scene_name) in unloaded_instances.iter() {
        if scene_manager.instance_is_ready(**instance) {
            commands.entity(entity).insert(SceneLoaded);
            if scene_name.0 == "main_skeleton.glb" {
                for (character_entity, _, main_skeleton_entity) in characters.iter_mut() {
                    let (main_skeleton_bones, main_armature_entity) =
                        get_main_skeleton_bones_and_armature(
                            &entity,
                            &all_entities_with_children,
                            &names,
                        );
                    if main_skeleton_entity.0 == entity {
                        let mut character_entity_commands = commands.entity(character_entity);
                        character_entity_commands.insert(MainSkeletonBonesAndArmature(
                            main_skeleton_bones,
                            main_armature_entity,
                        ));
                        next_state.set(SpawnScenesState::AwaitingAnimations)
                    }
                }
            }
        }
    }
}
