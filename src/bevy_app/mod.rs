mod camera_plugin;
mod plane_plugin;
use crate::Shared;
use crate::SharedState;
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;

use self::camera_plugin::CameraPlugin;
use self::plane_plugin::PlanePlugin;

#[derive(Resource)]
pub struct SharedResource(Shared<SharedState>);

pub fn bevy_main(comm_channel_plugin: impl Plugin, shared_state: Shared<SharedState>) {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.95,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy".to_string()),
                // resolution:WindowResolution::default(),
                fit_canvas_to_parent: true,
                ..Default::default()
            }),
            ..default()
        }))
        .add_plugins(comm_channel_plugin)
        .insert_resource(SharedResource(shared_state))
        .add_plugins(PlanePlugin)
        .add_plugins(CameraPlugin)
                .add_plugins(PanOrbitCameraPlugin)

        // .add_systems(Startup, setup)
        // .add_systems(Update, punch_cube)
        .run();
}
