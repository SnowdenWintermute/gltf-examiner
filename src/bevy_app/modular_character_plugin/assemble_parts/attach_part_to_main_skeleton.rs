use crate::bevy_app::{
    modular_character_plugin::{
        assemble_parts::{
            collect_bones::collect_bones,
            find_child_with_name_containing::find_child_with_name_containing,
        },
        AttachedPartsReparentedEntities,
    },
    utils::zero_transform,
};
use bevy::{prelude::*, utils::HashMap};

pub fn attach_part_to_main_skeleton(
    commands: &mut Commands,
    all_entities_with_children: &Query<&Children>,
    transforms: &mut Query<&mut Transform>,
    names: &Query<&Name>,
    part_scene_entity: &Entity,
    main_armature_entity: &Entity,
    main_skeleton_bones: &HashMap<String, Entity>,
    attached_parts_reparented_entities: &mut ResMut<AttachedPartsReparentedEntities>,
    visibility_query: &mut Query<&mut Visibility>,
) {
    let mut reparented_entities = Vec::new();

    let root_bone_option = find_child_with_name_containing(
        all_entities_with_children,
        names,
        &part_scene_entity,
        "Root",
    );

    let part_armature_option = find_child_with_name_containing(
        all_entities_with_children,
        names,
        &part_scene_entity,
        "CharacterArmature",
    );

    if let Some(part_armature) = part_armature_option {
        let mut part_armature_entity_commands = commands.entity(part_armature);
        zero_transform(part_armature, transforms);
        reparented_entities.push(part_armature);
        part_armature_entity_commands.set_parent(*main_armature_entity);
        // set visibility
        if let Ok(mut visibility) = visibility_query.get_mut(part_armature) {
            info!(
                "changing visibility for {:?}, was {:?}",
                names.get(*part_scene_entity),
                visibility
            );
            *visibility = Visibility::Visible;
        }
    }

    if let Some(root_bone) = root_bone_option {
        let mut part_bones = HashMap::new();
        collect_bones(
            all_entities_with_children,
            names,
            &root_bone,
            &mut part_bones,
        );

        for (name, part_bone) in part_bones {
            let mut entity_commands = commands.entity(part_bone);
            let new_parent_option = main_skeleton_bones.get(&name);

            if let Some(new_parent) = new_parent_option {
                zero_transform(part_bone, transforms);
                reparented_entities.push(part_bone);
                entity_commands.set_parent(*new_parent);
            }
        }

        attached_parts_reparented_entities
            .parts_and_entities
            .insert(*part_scene_entity, reparented_entities);
    }
}
