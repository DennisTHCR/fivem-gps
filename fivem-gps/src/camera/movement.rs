use bevy::prelude::*;

use crate::input::ParsedInput;

use super::MainCameraMarker;

pub fn move_camera(
    parsed_input: Res<ParsedInput>,
    mut camera: Query<&mut Transform, With<MainCameraMarker>>,
    time: Res<Time>,
) {
    camera.single_mut().translation +=
        1000. * parsed_input.direction.extend(0.) * time.delta_secs();
}

pub fn zoom(
    parsed_input: Res<ParsedInput>,
    mut camera: Query<&mut OrthographicProjection, With<MainCameraMarker>>,
) {
    let mut projection = camera.single_mut();
    if parsed_input.scroll_down {
        projection.scale *= 0.9;
    }
    if parsed_input.scroll_up {
        projection.scale /= 0.9;
    }
}
