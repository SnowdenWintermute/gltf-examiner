use crate::bevy_app::modular_character_plugin::{
    assemble_parts::{
        attach_part_to_main_skeleton::attach_part_to_main_skeleton,
        get_main_skeleton_bones_and_armature::get_main_skeleton_bones_and_armature,
    },
    spawn_scenes::{SceneEntitiesByName, SceneLoaded, SceneName},
    AttachedPartsReparentedEntities,
};
use bevy::prelude::*;

pub fn attach_newly_spawned_parts(
    mut commands: Commands,
    mut attached_parts_reparented_entities: ResMut<AttachedPartsReparentedEntities>,
    scene_entities_by_name: ResMut<SceneEntitiesByName>,
    recently_loaded_scenes_query: Query<
        (Entity, &SceneName),
        (With<SceneName>, Added<SceneLoaded>),
    >,
    all_entities_with_children: Query<&Children>,
    mut transforms: Query<&mut Transform>,
    names: Query<&Name>,
) {
    // for (entity, scene_name) in recently_loaded_scenes_query.iter() {
    //     info!("part attacher attaching: {:#?} {}", entity, scene_name.0);
    //     if scene_name.0 != "main_skeleton.glb" {
    //         let (main_skeleton_bones, main_armature_entity) =
    //             get_main_skeleton_bones_and_armature(&entity, &all_entities_with_children, &names);

    //         // ATTACH REQUESTED PART
    //         attach_part_to_main_skeleton(
    //             &mut commands,
    //             &all_entities_with_children,
    //             &mut transforms,
    //             &names,
    //             &scene_name.0,
    //             &entity,
    //             &main_armature_entity,
    //             &main_skeleton_bones,
    //             &mut attached_parts_reparented_entities,
    //         );
    //     }
    // }
}
