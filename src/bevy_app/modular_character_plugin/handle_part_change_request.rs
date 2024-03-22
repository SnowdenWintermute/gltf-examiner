use super::{
    spawn_scenes::SceneEntitiesByName, AttachedPartsReparentedEntities,
    CharacterSpawnedPartSceneNamesByCategory, SpawnedPartEvent,
};
use crate::{
    bevy_app::{
        asset_loader_plugin::MyAssets,
        modular_character_plugin::{
            assemble_parts::{
                attach_part_to_main_skeleton::attach_part_to_main_skeleton,
                get_main_skeleton_bones_and_armature::get_main_skeleton_bones_and_armature,
            },
            despawn_attached_part::despawn_attached_part,
            print_scene_tree::walk_tree,
            spawn_scenes::spawn_and_register_scene,
        },
    },
    comm_channels::CharacterPartSelectionEvent,
    frontend_common::CharacterPartCategories,
};
use bevy::{gltf::Gltf, prelude::*};

pub fn handle_part_change_request(
    mut commands: Commands,
    mut part_selection_event_reader: EventReader<CharacterPartSelectionEvent>,
    mut attach_newly_spawned_parts_event_writer: EventWriter<SpawnedPartEvent>,
    mut attached_parts_reparented_entities: ResMut<AttachedPartsReparentedEntities>,
    mut character_spawned_part_scene_names_by_category: ResMut<
        CharacterSpawnedPartSceneNamesByCategory,
    >,
    mut scene_entities_by_name: ResMut<SceneEntitiesByName>,
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    all_entities_with_children: Query<&Children>,
    mut transforms: Query<&mut Transform>,
    names: Query<&Name>,
) {
    for event in part_selection_event_reader.read() {
        info!("read part selection event: {:#?}", event);
        let file_name = &event.0.name;
        let category = &event.0.category;

        let gltf_handle_option = match category {
            CharacterPartCategories::Head => asset_pack.heads.get(file_name),
            CharacterPartCategories::Torso => asset_pack.torsos.get(file_name),
            CharacterPartCategories::Leg => asset_pack.legs.get(file_name),
            CharacterPartCategories::Weapon => asset_pack.weapons.get(file_name),
        };

        if let Some(gltf_handle) = gltf_handle_option {
            info!("handle: {:#?}", gltf_handle);
            // DESPAWN ANY CURRENT PARTS OF REQUESTED CATEGORY
            let current_part_in_category_option = character_spawned_part_scene_names_by_category
                .0
                .get(category);

            if let Some(scene_name) = current_part_in_category_option {
                despawn_attached_part(
                    &mut commands,
                    &scene_name,
                    &mut attached_parts_reparented_entities,
                    &mut scene_entities_by_name,
                );
            }

            // SPAWN SCENE FOR THAT PART
            let part_scene_entity = spawn_and_register_scene(
                &mut commands,
                &assets_gltf,
                gltf_handle.clone(),
                file_name.clone(),
                None,
                &mut scene_entities_by_name,
            )
            .expect("to spawn the scene");

            character_spawned_part_scene_names_by_category
                .0
                .insert(category.clone(), file_name.clone());

            info!("spawned part scene: {:?}", part_scene_entity);
            attach_newly_spawned_parts_event_writer.send(SpawnedPartEvent(event.0.clone()));
        }
    }
}
