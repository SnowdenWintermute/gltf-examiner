use bevy::{gltf::Gltf, prelude::*, utils::HashMap};

#[derive(Component, Debug)]
pub struct SceneName(pub String);

#[derive(States, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub enum SpawnScenesState {
    #[default]
    Spawning,
    AwaitingSkeletonAssignment,
    AwaitingAnimations,
    Done,
}

#[derive(Resource, Debug, Default)]
pub struct SceneEntitiesByName(pub HashMap<String, Entity>);

#[derive(Component, Debug)]
pub struct SceneLoaded;

pub fn spawn_scene(
    commands: &mut Commands,
    assets_gltf: &Res<Assets<Gltf>>,
    gltf_handle: Handle<Gltf>,
    name: String,
    animations_option: Option<&mut HashMap<String, Handle<AnimationClip>>>,
) -> Option<Entity> {
    if let Some(gltf) = assets_gltf.get(gltf_handle) {
        let entity_commands = commands.spawn((
            SceneBundle {
                scene: gltf.named_scenes["Scene"].clone(),
                ..Default::default()
            },
            SceneName(name.clone()),
        ));

        let entity = entity_commands.id();

        if let Some(animations) = animations_option {
            for named_animation in gltf.named_animations.iter() {
                info!("inserting animation: {}", named_animation.0);
                animations.insert(
                    named_animation.0.clone(),
                    gltf.named_animations[named_animation.0].clone(),
                );
            }
        }

        return Some(entity);
    }
    None
}
