use self::{
    assemble_parts::assemble_parts,
    link_animations::link_animations,
    paint_cubes_on_joints::paint_cubes_on_joints,
    part_change_plugin::PartChangePlugin,
    print_scene_tree::print_scene_tree,
    run_animations::run_animations,
    spawn_scenes::{spawn_skeleton, SceneEntitiesByName, SpawnScenesState},
};
use crate::{
    bevy_app::asset_loader_plugin::AssetLoaderState, frontend_common::CharacterPartCategories,
};
use bevy::{prelude::*, utils::HashMap};
mod assemble_parts;
mod link_animations;
mod paint_cubes_on_joints;
pub mod part_change_plugin;
mod print_scene_tree;
mod run_animations;
mod spawn_scenes;

#[derive(Resource, Default)]
pub struct AttachedPartsReparentedEntities {
    parts_and_entities: HashMap<String, Vec<Entity>>,
}

#[derive(Resource, Default)]
pub struct CharacterSpawnedPartSceneNamesByCategory(pub HashMap<CharacterPartCategories, String>);

#[derive(Event)]
pub struct SpawnedPartEvent(pub String);

pub struct ModularCharacterPlugin;
impl Plugin for ModularCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SpawnScenesState>()
            .init_resource::<AttachedPartsReparentedEntities>()
            .init_resource::<CharacterSpawnedPartSceneNamesByCategory>()
            .init_resource::<SceneEntitiesByName>()
            .init_resource::<Events<SpawnedPartEvent>>()
            .add_plugins(PartChangePlugin)
            .add_systems(OnEnter(AssetLoaderState::Done), spawn_skeleton)
            .add_systems(
                OnEnter(SpawnScenesState::Spawned),
                (
                    link_animations,
                    // print_scene_tree,
                    paint_cubes_on_joints,
                ),
            )
            .add_systems(OnEnter(SpawnScenesState::Done), run_animations);
        // .add_systems(Update, (mark_scenes_as_loaded, attach_newly_spawned_parts));
    }
}
