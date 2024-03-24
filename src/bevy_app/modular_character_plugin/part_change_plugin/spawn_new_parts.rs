use crate::{
    bevy_app::{
        asset_loader_plugin::MyAssets,
        modular_character_plugin::{
            spawn_character::{CharacterName, CharacterPartScenesAwaitingSpawn},
            spawn_scenes::spawn_scene,
        },
    },
    comm_channels::CharacterPartSelectionEvent,
    frontend_common::CharacterPartCategories,
};
use bevy::{gltf::Gltf, prelude::*};

pub fn spawn_new_parts(
    mut commands: Commands,
    mut characters: Query<(
        Entity,
        &CharacterName,
        &mut CharacterPartScenesAwaitingSpawn,
    )>,
    mut part_selection_event_reader: EventReader<CharacterPartSelectionEvent>,
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
            let part_scene_entity = spawn_scene(
                &mut commands,
                &assets_gltf,
                gltf_handle.clone(),
                file_name.clone(),
                None,
            )
            .expect("to spawn the scene");

            for (entity, character_name, mut parts_awaiting_spawn) in characters.iter_mut() {
                if character_name.0 == "r_chambers" {
                    parts_awaiting_spawn
                        .0
                        .insert(category.clone(), part_scene_entity);
                }
            }

            info!("spawned part scene: {:?}", part_scene_entity);
        }
    }
}

// get part change request
// spawn new part and store entity id and category on character "awaiting spawn"
// check all character awaiting spawn lists and check all recently spawned scenes for readiness
// if ready,
//   remove from awaiting spawn list and attach to main skeleton
//   remove any previous entity from character's currently attached parts by category list
//   despawn previous entity's mesh and bones
//   add entity to character's currently attached parts by category list
//
