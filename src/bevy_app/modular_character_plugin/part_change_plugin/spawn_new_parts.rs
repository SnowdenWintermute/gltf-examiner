use crate::{
    bevy_app::{
        asset_loader_plugin::MyAssets,
        modular_character_plugin::{
            spawn_character::{
                CharacterId, CharacterPartScenesAwaitingSpawn, MainSkeletonBonesAndArmature,
            },
            spawn_scenes::spawn_scene,
            CharactersById,
        },
    },
    comm_channels::CharacterPartSelectionEvent,
    frontend_common::CharacterPartCategories,
};
use bevy::{gltf::Gltf, prelude::*};

pub fn spawn_new_parts(
    mut commands: Commands,
    mut characters_with_spawned_skeletons: Query<(
        Entity,
        &CharacterId,
        &mut CharacterPartScenesAwaitingSpawn,
        &MainSkeletonBonesAndArmature,
    )>,
    mut part_selection_event_reader: EventReader<CharacterPartSelectionEvent>,
    character_by_id: Res<CharactersById>,
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

        //  - get associated character
        let character_entity = character_by_id.0.get(&0).expect("to have this character");
        // ensure it has an assigned skeleton
        if let Ok((_, _, mut parts_awaiting_spawn, skeleton_bones_and_armature)) =
            characters_with_spawned_skeletons.get_mut(*character_entity)
        {
            //  - spawn new part and store entity id and category on character "awaiting spawn"
            let gltf_handle = gltf_handle_option.expect("to have loaded the gltf file asset");
            let part_scene_entity = spawn_scene(
                &mut commands,
                &assets_gltf,
                gltf_handle.clone(),
                file_name.clone(),
                None,
                true,
            )
            .expect("to spawn the scene");
            info!("spawned part scene: {:?}", part_scene_entity);

            parts_awaiting_spawn
                .0
                .entry(category.clone())
                .or_default()
                .insert(part_scene_entity);
        }
    }
}
