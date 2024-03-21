use super::spawn_scenes::{SceneEntitiesByName, SceneName};
use bevy::prelude::*;

pub fn collect_parts(
    scene_query: Query<(Entity, &SceneName), With<SceneName>>,
    scene_entities_by_name: Res<SceneEntitiesByName>,
    names: Query<&Name>,
) {
    //
}
