mod attach_newly_loaded_part_scenes;
mod attach_newly_spawned_parts;
mod delete_bones;
mod despawn_attached_part;
mod despawn_old_parts;
mod despawn_orphan_bones;
mod mark_scenes_as_loaded;
pub mod send_part_names_to_yew;
mod spawn_new_parts;
use self::{
    attach_newly_loaded_part_scenes::attach_newly_loaded_part_scenes,
    despawn_orphan_bones::despawn_orphan_bones, send_part_names_to_yew::send_part_names_to_yew,
    spawn_new_parts::spawn_new_parts,
};
use super::spawn_scenes::SpawnScenesState;
use crate::bevy_app::asset_loader_plugin::AssetLoaderState;
use bevy::{prelude::*, time::common_conditions::on_timer};
use std::time::Duration;

#[derive(Resource, Debug, Default)]
pub struct BonesAwaitingDespawn(Vec<Entity>);

pub struct PartChangePlugin;
impl Plugin for PartChangePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BonesAwaitingDespawn>()
            .add_systems(OnEnter(AssetLoaderState::Done), send_part_names_to_yew)
            .add_systems(
                PreUpdate,
                despawn_orphan_bones.run_if(on_timer(Duration::from_secs(5))),
            )
            .add_systems(
                Update,
                (
                    ((spawn_new_parts, attach_newly_loaded_part_scenes).chain())
                    // mark_scenes_as_loaded,
                    // attach_newly_spawned_parts,
                )
                .run_if(in_state(SpawnScenesState::Done)),
            );
    }
}

// character has part in each category
// character entity has component for each category holding entity of corresponding part scene
// spawn scene
// - find character by name
// - spawn new scene
// - mark new scene as loaded and attach it
// - despawn their current scene in new part's category
// - add newly attached scene to character's part entities by category list
