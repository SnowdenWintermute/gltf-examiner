use bevy::prelude::*;
pub fn zero_transform(entity: Entity, transforms: &mut Query<&mut Transform>) {
    if let Ok(mut transform) = transforms.get_mut(entity) {
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
        transform.translation.z = 0.0;
        transform.rotation = Quat::from_xyzw(0.0, 0.0, 0.0, 0.0);
    }
}
