use super::{
    print_scene_tree::walk_tree, spawn_scenes::SceneEntitiesByName,
    AttachedPartsReparentedEntities, SpawnedPartEvent,
};
use crate::bevy_app::modular_character_plugin::assemble_parts::{
    attach_part_to_main_skeleton::attach_part_to_main_skeleton,
    get_main_skeleton_bones_and_armature::get_main_skeleton_bones_and_armature,
};
use bevy::{prelude::*, scene::SceneInstance};

pub fn attach_newly_spawned_parts(
    mut commands: Commands,
    mut attach_newly_spawned_parts_event_reader: EventReader<SpawnedPartEvent>,
    mut attached_parts_reparented_entities: ResMut<AttachedPartsReparentedEntities>,
    scene_entities_by_name: ResMut<SceneEntitiesByName>,
    all_entities_with_children: Query<&Children>,
    mut transforms: Query<&mut Transform>,
    names: Query<&Name>,
    scene_manager: Res<SceneSpawner>,
    // unloaded_instances: Query<(Entity, &SceneInstance, &SceneHook), Without<SceneHooked>>,
) {
    scene_manager.iter_instance_entities(instance_id);
    //
    for event in attach_newly_spawned_parts_event_reader.read() {
        let part_entity_option = scene_entities_by_name.0.get(&event.0.name);
        if let Some(part_scene_entity) = part_entity_option {
            walk_tree(&all_entities_with_children, &names, part_scene_entity, 0);

            let (main_skeleton_bones, main_armature_entity) = get_main_skeleton_bones_and_armature(
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
                &event.0.name,
                part_scene_entity,
                &main_armature_entity,
                &main_skeleton_bones,
                &mut attached_parts_reparented_entities,
            );
        }
    }
}
