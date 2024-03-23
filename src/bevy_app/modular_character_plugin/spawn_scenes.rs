use crate::bevy_app::asset_loader_plugin::MyAssets;
use bevy::{gltf::Gltf, prelude::*, utils::HashMap};

#[derive(Component, Debug)]
pub struct SceneName(pub String);

#[derive(States, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub enum SpawnScenesState {
    #[default]
    Spawning,
    Spawned,
    Done,
}

#[derive(Resource, Debug, Default)]
pub struct SceneEntitiesByName(pub HashMap<String, Entity>);

#[derive(Resource, Debug)]
pub struct Animations(pub HashMap<String, Handle<AnimationClip>>);

#[derive(Component, Debug)]
pub struct SceneLoaded;

pub fn spawn_skeleton(
    mut commands: Commands,
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    mut next_state: ResMut<NextState<SpawnScenesState>>,
    mut scene_entities_by_name: ResMut<SceneEntitiesByName>,
) {
    let mut animations = HashMap::new();

    // SPAWN SCENES
    for (name, gltf_handle) in &asset_pack.main_skeletons_with_animations {
        spawn_and_register_scene(
            &mut commands,
            &assets_gltf,
            gltf_handle.clone(),
            name.clone(),
            Some(&mut animations),
            &mut scene_entities_by_name,
        );
    }

    commands.insert_resource(Animations(animations));

    next_state.set(SpawnScenesState::Spawned)
}

pub fn spawn_and_register_scene(
    commands: &mut Commands,
    assets_gltf: &Res<Assets<Gltf>>,
    gltf_handle: Handle<Gltf>,
    name: String,
    animations_option: Option<&mut HashMap<String, Handle<AnimationClip>>>,
    scene_entities_by_name: &mut ResMut<SceneEntitiesByName>,
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
        scene_entities_by_name.0.insert(name.clone(), entity);

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
