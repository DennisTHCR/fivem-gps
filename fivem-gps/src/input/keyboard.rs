use bevy::prelude::*;

use super::ParsedInput;

pub struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update);
    }
}

fn update(mut parsed_input: ResMut<ParsedInput>, keys: Res<ButtonInput<KeyCode>>) {
    let mut direction: Vec2 = Vec2::default();
    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    parsed_input.direction = direction.normalize_or_zero();
    direction = Vec2::default();
    if keys.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    if keys.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    parsed_input.offset_direction = direction.normalize_or_zero();
}
