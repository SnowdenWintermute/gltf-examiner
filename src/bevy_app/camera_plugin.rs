use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(3.0, 3.0, 3.0),
            ..Default::default()
        },
        PanOrbitCamera {
            radius: Some(45.0),
            focus: Vec3::new(0.0, 2.0, 0.0),
            alpha: Some(0.5),
            ..Default::default()
        },
    ));
}
