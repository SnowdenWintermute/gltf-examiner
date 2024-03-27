use super::HomeLocation;
use bevy::{gltf::Gltf, prelude::*};

#[derive(Component, Debug)]
pub struct SceneName(pub String);

#[derive(Component, Debug)]
pub struct SceneLoaded;

pub fn spawn_scene(
    commands: &mut Commands,
    assets_gltf: &Res<Assets<Gltf>>,
    gltf_handle: Handle<Gltf>,
    name: String,
    spawn_hidden: bool,
    start_position: HomeLocation,
) -> Option<Entity> {
    if let Some(gltf) = assets_gltf.get(gltf_handle) {
        let visibility = if spawn_hidden {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
        let mut transform = Transform::from_xyz(
            start_position.position.x,
            start_position.position.y,
            start_position.position.z,
        );
        transform.rotate_y(start_position.rotation);

        let entity_commands = commands.spawn((
            SceneBundle {
                scene: gltf.named_scenes["Scene"].clone(),
                transform,
                visibility,
                ..Default::default()
            },
            SceneName(name.clone()),
        ));

        let entity = entity_commands.id();

        return Some(entity);
    }
    None
}
