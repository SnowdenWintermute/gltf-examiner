use self::{
    assign_skeleton_bones_to_characters::assign_skeleton_bones_to_characters,
    part_change_plugin::PartChangePlugin, run_animations::run_animations,
    spawn_character::spawn_character, spawn_scenes::SpawnScenesState,
};
use super::utils::{
    link_animations::link_animations, paint_cubes_on_scene_children::paint_cubes_on_scene_children,
};
use crate::bevy_app::asset_loader_plugin::AssetLoaderState;
use bevy::{prelude::*, utils::HashMap};
mod assemble_parts;
mod assign_skeleton_bones_to_characters;
pub mod part_change_plugin;
mod run_animations;
mod spawn_character;
pub mod spawn_scenes;

pub type CharacterId = u32;

// RESOURCES
#[derive(Resource, Debug, Default)]
pub struct SkeletonsAwaitingCharacterAssignment(pub HashMap<CharacterId, Entity>);
#[derive(Resource, Debug, Default)]
pub struct Animations(pub HashMap<String, Handle<AnimationClip>>);
#[derive(Resource, Debug, Default)]
pub struct CharactersById(pub HashMap<CharacterId, Entity>);
#[derive(Resource, Default)]
pub struct AttachedPartsReparentedEntities {
    parts_and_entities: HashMap<Entity, Vec<Entity>>,
}

pub struct ModularCharacterPlugin;
impl Plugin for ModularCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SpawnScenesState>()
            .init_resource::<AttachedPartsReparentedEntities>()
            .init_resource::<CharactersById>()
            .init_resource::<SkeletonsAwaitingCharacterAssignment>()
            .init_resource::<Animations>()
            .add_plugins(PartChangePlugin)
            .add_systems(OnEnter(AssetLoaderState::Done), spawn_character)
            .add_systems(Update, assign_skeleton_bones_to_characters)
            .add_systems(
                OnEnter(SpawnScenesState::AwaitingAnimations),
                (link_animations, paint_cubes_on_scene_children),
            )
            .add_systems(OnEnter(SpawnScenesState::Done), run_animations);
    }
}
