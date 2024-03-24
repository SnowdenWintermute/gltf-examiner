use super::BonesAwaitingDespawn;
use bevy::prelude::*;
use gloo::console::log;

pub fn despawn_orphan_bones(
    mut commands: Commands,
    mut bones_awaiting_despawn: ResMut<BonesAwaitingDespawn>,
    all_entities_with_children: Query<&Children>,
    names: Query<&Name>,
) {
    while let Some(bone_entity) = bones_awaiting_despawn.0.pop() {
        if let Ok(children) = all_entities_with_children.get(bone_entity) {
            for child in children {
                let mut child_entity_commands = commands.entity(*child);
                log!(format!(
                    "despawning orphan bone child: {:?}",
                    names.get(*child)
                ));
                child_entity_commands.despawn();
            }
        };
        let mut bone_entity_commands = commands.entity(bone_entity);
        log!(format!(
            "despawning orphan bone: {:?}",
            names.get(bone_entity)
        ));
        bone_entity_commands.despawn();
    }
}
