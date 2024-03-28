use self::{
    assign_skeleton_bones_to_characters::assign_skeleton_bones_to_characters,
    attack_sequence::{
        draw_direction_ray_gizmos::draw_directional_gizmos, handle_attack_sequence_start_requests,
        move_entities_toward_destinations::move_entities_toward_destinations,
    },
    handle_animation_change_requests::handle_animation_change_requests,
    part_change_plugin::PartChangePlugin,
    register_animations::register_animations,
    run_animations::run_animations,
    spawn_character::{spawn_character, spawn_characters},
    spawn_combatants_in_battle_locations::spawn_combatants_in_battle_locations,
};
use super::utils::link_animations::link_animations;
use crate::bevy_app::asset_loader_plugin::AssetLoaderState;
use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
mod assemble_parts;
mod assign_skeleton_bones_to_characters;
mod attack_sequence;
mod handle_animation_change_requests;
pub mod part_change_plugin;
mod register_animations;
mod run_animations;
mod spawn_character;
mod spawn_combatants_in_battle_locations;
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
#[derive(Resource, Default)]
pub struct CombatantsExecutingAttacks(HashSet<CharacterId>);

#[derive(Default, Debug, Clone, Component)]
pub struct HomeLocation(Transform);

pub struct ModularCharacterPlugin;
impl Plugin for ModularCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AttachedPartsReparentedEntities>()
            .init_resource::<CharactersById>()
            .init_resource::<SkeletonsAwaitingCharacterAssignment>()
            .init_resource::<Animations>()
            .init_resource::<NextCharacterXLocation>()
            .init_resource::<CombatantsExecutingAttacks>()
            .add_plugins(PartChangePlugin)
            .add_systems(
                OnEnter(AssetLoaderState::RegisteringAnimations),
                register_animations,
            )
            .add_systems(
                Update,
                (
                    spawn_characters,
                    assign_skeleton_bones_to_characters,
                    link_animations,
                    run_animations,
                    handle_animation_change_requests,
                    draw_directional_gizmos,
                    handle_attack_sequence_start_requests,
                    move_entities_toward_destinations,
                )
                    .run_if(in_state(AssetLoaderState::Done)),
            )
            .add_systems(
                OnEnter(AssetLoaderState::Done),
                spawn_combatants_in_battle_locations,
            );
    }
}
