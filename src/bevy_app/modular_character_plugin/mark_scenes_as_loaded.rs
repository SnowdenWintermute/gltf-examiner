use super::{
    assemble_parts::{
        attach_part_to_main_skeleton::attach_part_to_main_skeleton,
        get_main_skeleton_bones_and_armature::get_main_skeleton_bones_and_armature,
    },
    spawn_scenes::{SceneEntitiesByName, SceneLoaded, SceneName},
    AttachedPartsReparentedEntities,
};
use bevy::{prelude::*, scene::SceneInstance};

pub fn mark_scenes_as_loaded(
    mut commands: Commands,
    scene_manager: Res<SceneSpawner>,
    unloaded_instances: Query<(Entity, &SceneInstance, &SceneName), Without<SceneLoaded>>,
    mut attached_parts_reparented_entities: ResMut<AttachedPartsReparentedEntities>,
    scene_entities_by_name: ResMut<SceneEntitiesByName>,
    all_entities_with_children: Query<&Children>,
    mut transforms: Query<&mut Transform>,
    names: Query<&Name>,
) {
    for (entity, instance, scene_name) in unloaded_instances.iter() {
        if scene_manager.instance_is_ready(**instance) {
            commands.entity(entity).insert(SceneLoaded);
            let name = &scene_name.0;
            if scene_name.0 != "main_skeleton.glb" {
                let (main_skeleton_bones, main_armature_entity) =
                    get_main_skeleton_bones_and_armature(
                        &scene_entities_by_name,
                        &all_entities_with_children,
                        &names,
                    );

                // ATTACH REQUESTED PART
                attach_part_to_main_skeleton(
                    &mut commands,
                    &all_entities_with_children,
                    &mut transforms,
                    &names,
                    &name,
                    &entity,
                    &main_armature_entity,
                    &main_skeleton_bones,
                    &mut attached_parts_reparented_entities,
                );
            }
        }
    }
}
