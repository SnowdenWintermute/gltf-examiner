use bevy::prelude::*;

use crate::bevy_app::modular_character_plugin::{
    assemble_parts::find_child_with_name_containing::find_child_with_name_containing,
    spawn_scenes::SceneName, AttachedPartsReparentedEntities,
};

pub fn delete_bones(
    mut commands: Commands,
    scene_query: Query<Entity, With<SceneName>>,
    all_entities_with_children: Query<&Children>,
    names: Query<&Name>,
) {
    for entity in scene_query.iter() {
        if let Some(root_bone) =
            find_child_with_name_containing(&all_entities_with_children, &names, &entity, "Root")
        {
            let entity_commands = commands.entity(root_bone);
            entity_commands.despawn_recursive();
        };
    }
}
