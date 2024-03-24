use self::{
    assign_skeleton_bones_to_character::assign_skeleton_bones_to_character,
    link_animations::link_animations,
    paint_cubes_on_joints::paint_cubes_on_joints,
    part_change_plugin::PartChangePlugin,
    run_animations::run_animations,
    spawn_character::spawn_character,
    spawn_scenes::{SceneEntitiesByName, SpawnScenesState},
};
use crate::bevy_app::asset_loader_plugin::AssetLoaderState;
use bevy::{prelude::*, utils::HashMap};
mod assemble_parts;
mod assign_skeleton_bones_to_character;
mod link_animations;
mod paint_cubes_on_joints;
pub mod part_change_plugin;
mod print_scene_tree;
mod run_animations;
mod spawn_character;
mod spawn_scenes;

#[derive(Resource, Default)]
pub struct AttachedPartsReparentedEntities {
    parts_and_entities: HashMap<Entity, Vec<Entity>>,
}

#[derive(Event)]
pub struct SpawnedPartEvent(pub String);

pub struct ModularCharacterPlugin;
impl Plugin for ModularCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SpawnScenesState>()
            .init_resource::<AttachedPartsReparentedEntities>()
            .init_resource::<SceneEntitiesByName>()
            .init_resource::<Events<SpawnedPartEvent>>()
            .add_plugins(PartChangePlugin)
            .add_systems(OnEnter(AssetLoaderState::Done), spawn_character)
            .add_systems(
                OnEnter(SpawnScenesState::AwaitingSkeletonAssignment),
                assign_skeleton_bones_to_character,
            )
            .add_systems(
                OnEnter(SpawnScenesState::AwaitingAnimations),
                (link_animations, paint_cubes_on_joints),
            )
            .add_systems(OnEnter(SpawnScenesState::Done), run_animations);
        // .add_systems(Update, (mark_scenes_as_loaded, attach_newly_spawned_parts));
    }
}
