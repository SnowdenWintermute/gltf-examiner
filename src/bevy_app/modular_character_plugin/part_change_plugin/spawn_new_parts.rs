use crate::{
    bevy_app::{
        asset_loader_plugin::MyAssets,
        modular_character_plugin::{
            spawn_scenes::{spawn_and_register_scene, SceneEntitiesByName},
            CharacterSpawnedPartSceneNamesByCategory,
        },
    },
    comm_channels::CharacterPartSelectionEvent,
    frontend_common::CharacterPartCategories,
};
use bevy::{gltf::Gltf, prelude::*};

pub fn spawn_new_parts(
    mut commands: Commands,
    mut part_selection_event_reader: EventReader<CharacterPartSelectionEvent>,
    mut character_spawned_part_scene_names_by_category: ResMut<
        CharacterSpawnedPartSceneNamesByCategory,
    >,
    mut scene_entities_by_name: ResMut<SceneEntitiesByName>,
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    for event in part_selection_event_reader.read() {
        info!("part spawner read part selection event: {:#?}", event);
        let file_name = &event.0.name;
        let category = &event.0.category;

        let gltf_handle_option = match category {
            CharacterPartCategories::Head => asset_pack.heads.get(file_name),
            CharacterPartCategories::Torso => asset_pack.torsos.get(file_name),
            CharacterPartCategories::Leg => asset_pack.legs.get(file_name),
            CharacterPartCategories::Weapon => asset_pack.weapons.get(file_name),
        };

        if let Some(gltf_handle) = gltf_handle_option {
            // // SPAWN SCENE FOR THAT PART
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
        }
    }
}
