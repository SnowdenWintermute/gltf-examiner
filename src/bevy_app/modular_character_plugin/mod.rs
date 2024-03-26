use self::{
    assign_skeleton_bones_to_characters::assign_skeleton_bones_to_characters,
    handle_animation_change_requests::handle_animation_change_requests,
    part_change_plugin::PartChangePlugin, register_animations::register_animations,
    run_animations::run_animations, spawn_character::spawn_character,
};
use super::utils::link_animations::link_animations;
use crate::bevy_app::asset_loader_plugin::AssetLoaderState;
use bevy::{prelude::*, utils::HashMap};
mod assemble_parts;
mod assign_skeleton_bones_to_characters;
mod handle_animation_change_requests;
pub mod part_change_plugin;
mod register_animations;
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
#[derive(Resource, Default)]
pub struct NextCharacterXLocation(f32);

pub struct ModularCharacterPlugin;
impl Plugin for ModularCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AttachedPartsReparentedEntities>()
            .init_resource::<CharactersById>()
            .init_resource::<SkeletonsAwaitingCharacterAssignment>()
            .init_resource::<Animations>()
            .init_resource::<NextCharacterXLocation>()
            .add_plugins(PartChangePlugin)
            .add_systems(
                OnEnter(AssetLoaderState::RegisteringAnimations),
                register_animations,
            )
            .add_systems(
                Update,
                (
                    spawn_character,
                    assign_skeleton_bones_to_characters,
                    link_animations,
                    run_animations,
                    handle_animation_change_requests,
                )
                    .run_if(in_state(AssetLoaderState::Done)),
            );
    }
}
