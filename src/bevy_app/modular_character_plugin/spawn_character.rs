use super::spawn_scenes::{spawn_scene, SceneEntitiesByName, SpawnScenesState};
use crate::bevy_app::asset_loader_plugin::MyAssets;
use crate::frontend_common::CharacterPartCategories;
use bevy::{gltf::Gltf, prelude::*, utils::HashMap};

#[derive(Resource, Debug)]
pub struct Animations(pub HashMap<String, Handle<AnimationClip>>);

#[derive(Component, Debug)]
pub struct MainSkeletonEntity(pub Entity);

#[derive(Component, Debug)]
pub struct MainSkeletonBonesAndArmature(pub HashMap<String, Entity>, pub Entity);

#[derive(Component, Default)]
pub struct CharacterAttachedPartScenes(pub HashMap<CharacterPartCategories, Entity>);

#[derive(Component, Default)]
pub struct CharacterPartScenesAwaitingSpawn(pub HashMap<CharacterPartCategories, Entity>);

#[derive(Component)]
pub struct CharacterName(pub String);

pub fn spawn_character(
    mut commands: Commands,
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    mut next_state: ResMut<NextState<SpawnScenesState>>,
    mut scene_entities_by_name: ResMut<SceneEntitiesByName>,
) {
    let mut animations = HashMap::new();

    // SPAWN SCENES
    let skeleton_handle = asset_pack
        .main_skeletons_with_animations
        .get("main_skeleton.glb")
        .expect("to have loaded the skeleton glb");

    let skeleton_entity = spawn_scene(
        &mut commands,
        &assets_gltf,
        skeleton_handle.clone(),
        "main_skeleton.glb".to_string(),
        Some(&mut animations),
    )
    .expect("to have a skeleton gltf handle");

    let character_entity = commands.spawn((
        CharacterName("r_chambers".to_string()),
        MainSkeletonEntity(skeleton_entity),
        CharacterAttachedPartScenes(HashMap::new()),
        CharacterPartScenesAwaitingSpawn(HashMap::new()),
    ));

    commands.insert_resource(Animations(animations));

    next_state.set(SpawnScenesState::AwaitingSkeletonAssignment)
}
