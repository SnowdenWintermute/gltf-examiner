use crate::{
    bevy_app::modular_character_plugin::{
        part_change_plugin::despawn_attached_part::despawn_attached_part,
        spawn_scenes::SceneEntitiesByName, AttachedPartsReparentedEntities,
    },
    comm_channels::CharacterPartSelectionEvent,
};
use bevy::prelude::*;

pub fn despawn_old_parts(
    mut commands: Commands,
    mut part_selection_event_reader: EventReader<CharacterPartSelectionEvent>,
    mut scene_entities_by_name: ResMut<SceneEntitiesByName>,
    mut attached_parts_reparented_entities: ResMut<AttachedPartsReparentedEntities>,
    all_entities_with_children: Query<&Children>,
    names: Query<&Name>,
) {
    for event in part_selection_event_reader.read() {
        // info!("despawner read part selection event: {:#?}", event);
        // // let file_name = &event.0.name;
        // let category = &event.0.category;

        // // DESPAWN ANY CURRENT PARTS OF REQUESTED CATEGORY
        // if let Some(scene_name) = character_spawned_part_scene_names_by_category
        //     .0
        //     .remove(category)
        // {
        //     despawn_attached_part(
        //         &mut commands,
        //         &scene_name,
        //         &mut attached_parts_reparented_entities,
        //         &mut scene_entities_by_name,
        //     );
        // }
    }
}
