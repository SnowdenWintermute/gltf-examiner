use super::{
    spawn_scenes::{spawn_scene, SpawnScenesState},
    Animations, CharactersById, SkeletonsAwaitingCharacterAssignment,
};
use crate::bevy_app::asset_loader_plugin::MyAssets;
use crate::frontend_common::CharacterPartCategories;
use bevy::{gltf::Gltf, prelude::*, utils::HashMap, utils::HashSet};

// CHARACTER COMPONENTS
#[derive(Component)]
pub struct CharacterId(pub u32);
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

pub fn spawn_character(
    mut commands: Commands,
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    mut characters_by_id: ResMut<CharactersById>,
    mut animations: ResMut<Animations>,
    mut skeletons_awaiting_character_assignment: ResMut<SkeletonsAwaitingCharacterAssignment>,
    mut next_state: ResMut<NextState<SpawnScenesState>>,
) {
    let character_id = 0;
    // - spawn skeleton and store its entity id on the character
    let skeleton_handle = asset_pack
        .main_skeletons_with_animations
        .get("main_skeleton.glb")
        .expect("to have loaded the skeleton glb");

    let skeleton_entity = spawn_scene(
        &mut commands,
        &assets_gltf,
        skeleton_handle.clone(),
        "main_skeleton.glb".to_string(),
        Some(&mut animations.0),
        false,
    )
    .expect("to have a skeleton gltf handle");
    // - add skeleton entity to skeletons_awaiting_character_assignment resource
    skeletons_awaiting_character_assignment
        .0
        .insert(character_id, skeleton_entity);

    let character_entity_commands = commands.spawn((
        CharacterId(character_id),
        MainSkeletonEntity(skeleton_entity),
        CharacterAttachedPartScenes(HashMap::new()),
        CharacterPartScenesAwaitingSpawn(HashMap::new()),
    ));

    // - add character id to list of characters resource
    characters_by_id
        .0
        .insert(character_id, character_entity_commands.id());

    next_state.set(SpawnScenesState::AwaitingAnimations)
}
