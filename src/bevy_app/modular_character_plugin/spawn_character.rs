use super::animation_manager_component::AnimationManagerComponent;
use super::{spawn_scenes::spawn_scene, CharactersById, SkeletonsAwaitingCharacterAssignment};
use super::{CharacterId, HomeLocation};
use crate::frontend_common::CharacterPartCategories;
use crate::{bevy_app::asset_loader_plugin::MyAssets, comm_channels::CharacterSpawnEvent};
use bevy::{gltf::Gltf, prelude::*, utils::HashMap, utils::HashSet};
use bevy_mod_billboard::BillboardTextBundle;

// CHARACTER COMPONENTS
#[derive(Component)]
pub struct CharacterIdComponent(pub u32);
#[derive(Component, Debug)]
pub struct MainSkeletonEntity(pub Entity);
#[derive(Component, Debug)]
pub struct MainSkeletonBonesAndArmature(pub HashMap<String, Entity>, pub Entity);
/// Queue of part entities waiting for spawn. Using Vec in case multiple part scenes get queued
/// from part change requests before they are spawned
#[derive(Component, Default)]
pub struct CharacterPartScenesAwaitingSpawn(pub HashMap<CharacterPartCategories, HashSet<Entity>>);
#[derive(Component, Default)]
pub struct CharacterAttachedPartScenes(pub HashMap<CharacterPartCategories, Entity>);
#[derive(Component, Default, Clone)]
pub struct HitboxRadius(pub f32);

pub fn spawn_characters(
    mut commands: Commands,
    mut character_spawn_event_reader: EventReader<CharacterSpawnEvent>,
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    mut characters_by_id: ResMut<CharactersById>,
    mut skeletons_awaiting_character_assignment: ResMut<SkeletonsAwaitingCharacterAssignment>,
) {
    for event in character_spawn_event_reader.read() {
        let character_id = event.0;
        spawn_character(
            &mut commands,
            &asset_pack,
            &assets_gltf,
            &mut characters_by_id,
            &mut skeletons_awaiting_character_assignment,
            HomeLocation(Transform::from_xyz(0.0, 0.0, 0.0)),
            character_id,
        )
    }
}

pub fn spawn_character(
    commands: &mut Commands,
    asset_pack: &Res<MyAssets>,
    assets_gltf: &Res<Assets<Gltf>>,
    characters_by_id: &mut ResMut<CharactersById>,
    skeletons_awaiting_character_assignment: &mut ResMut<SkeletonsAwaitingCharacterAssignment>,
    home_location: HomeLocation,
    character_id: CharacterId,
) {
    // - spawn skeleton and store its entity id on the character
    let skeleton_handle = asset_pack
        .main_skeletons_with_animations
        .get("main_skeleton.glb")
        .expect("to have loaded the skeleton glb");

    let skeleton_entity = spawn_scene(
        commands,
        &assets_gltf,
        skeleton_handle.clone(),
        "main_skeleton.glb".to_string(),
        false,
        home_location.clone(),
    )
    .expect("to have a skeleton gltf handle");

    // - add skeleton entity to skeletons_awaiting_character_assignment resource
    skeletons_awaiting_character_assignment
        .0
        .insert(character_id, skeleton_entity);

    let character_entity_commands = commands.spawn((
        CharacterIdComponent(character_id),
        MainSkeletonEntity(skeleton_entity),
        CharacterAttachedPartScenes(HashMap::new()),
        CharacterPartScenesAwaitingSpawn(HashMap::new()),
        home_location,
        AnimationManagerComponent::default(),
        HitboxRadius(0.7),
    ));

    let character_entity = character_entity_commands.id();
    // - add character id to list of characters resource
    characters_by_id.0.insert(character_id, character_entity);

    // BILLBOARD
    let font_handle = asset_pack
        .font_files
        .get("FiraSans-Regular.ttf")
        .expect("to have loaded the font");

    let mut billboard_entity_commands = commands.spawn(BillboardTextBundle {
        transform: Transform::from_xyz(0.0, 2.0, 0.0).with_scale(Vec3::splat(0.003)),
        text: Text::from_sections([TextSection {
            value: format!("Character {}", character_id),
            style: TextStyle {
                font_size: 60.0,
                font: font_handle.clone(),
                color: Color::WHITE,
            },
        }]),
        ..Default::default()
    });

    billboard_entity_commands.set_parent(skeleton_entity);
}
