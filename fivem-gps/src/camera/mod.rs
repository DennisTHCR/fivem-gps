mod movement;

use bevy::prelude::*;
use movement::{move_camera, zoom};

pub struct CameraManagementPlugin;

impl Plugin for CameraManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_camera)
            .add_systems(Update, (move_camera, zoom));
    }
}

#[derive(Component)]
pub struct MainCameraMarker;

fn init_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MainCameraMarker));
}
