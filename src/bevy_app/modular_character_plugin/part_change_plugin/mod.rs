mod attach_newly_spawned_parts;
mod despawn_attached_part;
mod despawn_old_parts;
mod mark_scenes_as_loaded;
pub mod send_part_names_to_yew;
mod spawn_new_parts;
use self::{
    attach_newly_spawned_parts::attach_newly_spawned_parts, despawn_old_parts::despawn_old_parts,
    mark_scenes_as_loaded::mark_scenes_as_loaded, send_part_names_to_yew::send_part_names_to_yew,
    spawn_new_parts::spawn_new_parts,
};
use super::spawn_scenes::SpawnScenesState;
use crate::bevy_app::asset_loader_plugin::AssetLoaderState;
use bevy::prelude::*;

pub struct PartChangePlugin;
impl Plugin for PartChangePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AssetLoaderState::Done), send_part_names_to_yew)
            .add_systems(
                Update,
                (
                    ((
                        mark_scenes_as_loaded,
                        despawn_old_parts,
                        spawn_new_parts,
                        attach_newly_spawned_parts,
                    )
                        .chain())
                    // mark_scenes_as_loaded,
                    // attach_newly_spawned_parts,
                )
                .run_if(in_state(SpawnScenesState::Done)),
            );
    }
}
