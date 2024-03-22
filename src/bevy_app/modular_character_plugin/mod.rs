use self::{
    assemble_parts::assemble_parts,
    attach_newly_spawned_parts::attach_newly_spawned_parts,
    collect_parts::CollectPartsPlugin,
    handle_part_change_request::handle_part_change_request,
    link_animations::link_animations,
    paint_cubes_on_joints::paint_cubes_on_joints,
    print_scene_tree::print_scene_tree,
    run_animations::run_animations,
    spawn_scenes::{spawn_scenes, SceneEntitiesByName, SpawnScenesState},
};
use crate::{
    bevy_app::asset_loader_plugin::AssetLoaderState,
    frontend_common::{CharacterPartCategories, CharacterPartSelection},
};
use bevy::{prelude::*, utils::HashMap};
mod assemble_parts;
mod attach_newly_spawned_parts;
pub mod collect_parts;
mod despawn_attached_part;
mod handle_part_change_request;
mod link_animations;
mod paint_cubes_on_joints;
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
pub struct SpawnedPartEvent(pub CharacterPartSelection);

pub struct ModularCharacterPlugin;
impl Plugin for ModularCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SpawnScenesState>()
            .init_resource::<AttachedPartsReparentedEntities>()
            .init_resource::<CharacterSpawnedPartSceneNamesByCategory>()
            .init_resource::<SceneEntitiesByName>()
            .init_resource::<Events<SpawnedPartEvent>>()
            .add_plugins(CollectPartsPlugin)
            .add_systems(OnEnter(AssetLoaderState::Done), spawn_scenes)
            .add_systems(
                OnEnter(SpawnScenesState::Spawned),
                (
                    link_animations,
                    // print_scene_tree,
                    paint_cubes_on_joints,
                ),
            )
            .add_systems(
                OnEnter(SpawnScenesState::Done),
                (run_animations, assemble_parts),
            )
            .add_systems(
                Update,
                handle_part_change_request.run_if(in_state(SpawnScenesState::Done)),
            )
            // .add_systems(Update, attach_newly_spawned_parts)
            ;
    }
}
