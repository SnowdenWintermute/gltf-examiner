use crate::bevy_app::modular_character_plugin::{
    spawn_scenes::SceneEntitiesByName, AttachedPartsReparentedEntities,
};
use bevy::prelude::*;

pub fn despawn_attached_part(
    commands: &mut Commands,
    part_scene_name: &String,
    attached_parts_reparented_entities: &mut ResMut<AttachedPartsReparentedEntities>,
    scene_entities_by_name: &mut ResMut<SceneEntitiesByName>,
) {
    // remove it from the scene_entities_by_name register
    if let Some(part_scene_entity) = scene_entities_by_name.0.remove(part_scene_name) {
        info!("despawing {:?}", part_scene_entity);
        // remove any entities in the scene that weren't reparented during part attachment
        let part_scene_entity_commands = commands.entity(part_scene_entity);
        part_scene_entity_commands.despawn_recursive();

        // remove entities that were originally part of the scene but were reparented when attached
        // to the main skeleton
        let reparented_entities_option = attached_parts_reparented_entities
            .parts_and_entities
            .remove(part_scene_name);

        if let Some(reparented_entities) = reparented_entities_option {
            info!("despawing reparented for {:?}", part_scene_name);
            for entity in reparented_entities {
                let commands = commands.entity(entity);
                commands.despawn_recursive();
            }
        }
    }
}
