use super::BonesAwaitingDespawn;
use crate::bevy_app::modular_character_plugin::{
    assemble_parts::attach_part_to_main_skeleton::attach_part_to_main_skeleton,
    part_change_plugin::despawn_attached_part::despawn_attached_part,
    spawn_character::{
        CharacterAttachedPartScenes, CharacterName, CharacterPartScenesAwaitingSpawn,
        MainSkeletonBonesAndArmature, MainSkeletonEntity,
    },
    spawn_scenes::{SceneLoaded, SceneName},
    AttachedPartsReparentedEntities,
};
use bevy::{prelude::*, scene::SceneInstance};
use gloo::console::info;

pub fn attach_newly_loaded_part_scenes(
    mut commands: Commands,
    scene_manager: Res<SceneSpawner>,
    unloaded_instances: Query<(Entity, &SceneInstance, &SceneName), Without<SceneLoaded>>,
    mut characters: Query<(
        Entity,
        &CharacterName,
        &MainSkeletonEntity,
        &MainSkeletonBonesAndArmature,
        &mut CharacterPartScenesAwaitingSpawn,
        &mut CharacterAttachedPartScenes,
    )>,
    mut attached_parts_reparented_entities: ResMut<AttachedPartsReparentedEntities>,
    all_entities_with_children: Query<&Children>,
    mut transforms: Query<&mut Transform>,
    names: Query<&Name>,
    mut bones_awaiting_despawn: ResMut<BonesAwaitingDespawn>,
) {
    for (entity, instance, scene_name) in unloaded_instances.iter() {
        if scene_manager.instance_is_ready(**instance) {
            // MARK AS LOADED
            commands.entity(entity).insert(SceneLoaded);
            let name = &scene_name.0;
            info!(format!("marked scene as loaded: {} {:?}", name, entity));

            for (
                character_entity,
                character_name,
                main_skeleton_entity,
                main_skeleton_bones_and_armature,
                mut awaiting_spawn,
                mut attached,
            ) in characters.iter_mut()
            {
                if entity == main_skeleton_entity.0 {
                    continue;
                }
                info!(format!(
                    "attaching to main skeleton entity: {:?}",
                    main_skeleton_entity.0
                ));
                // ATTACH PART
                attach_part_to_main_skeleton(
                    &mut commands,
                    &all_entities_with_children,
                    &mut transforms,
                    &names,
                    &entity,
                    &main_skeleton_bones_and_armature.1,
                    &main_skeleton_bones_and_armature.0,
                    &mut attached_parts_reparented_entities,
                );
                // REMOVE NEW PART FROM CHARACTER'S AWAITING SPAWN LIST
                let mut matching_category = None;
                for (category, entity_awaiting_spawn) in awaiting_spawn.0.iter() {
                    info!(format!(
                        "category: {:?}, entity_awaiting_spawn: {:?}",
                        category, entity_awaiting_spawn
                    ));
                    if entity == *entity_awaiting_spawn {
                        matching_category = Some(category.clone());
                    }
                }
                info!(format!("matching category: {:?}", matching_category));
                if let Some(category) = matching_category {
                    awaiting_spawn.0.remove(&category);
                    // ADD NEW PART TO CHARACTER'S PART ENTITIES LIST
                    // REMOVE OLD PART FROM CHARACTER'S PART ENTITIES LIST
                    if let Some(old_part) = attached.0.remove(&category) {
                        // DESPAWN OLD PART
                        despawn_attached_part(
                            &mut commands,
                            &old_part,
                            &mut attached_parts_reparented_entities,
                            &all_entities_with_children,
                            &names,
                            &mut bones_awaiting_despawn,
                        );
                    };
                    attached.0.insert(category.clone(), entity);
                }
            }
        }
    }
}
