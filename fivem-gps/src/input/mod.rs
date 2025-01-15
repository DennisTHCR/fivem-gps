mod cursor;
mod keyboard;
mod mouse;

use bevy::prelude::*;
use cursor::CursorPlugin;
use keyboard::KeyboardPlugin;
use mouse::MousePlugin;

#[derive(Resource, Default)]
pub struct ParsedInput {
    pub direction: Vec2,
    pub offset_direction: Vec2,
    pub cursor_position: Vec2,
    pub scroll_up: bool,
    pub scroll_down: bool,
    pub left_click: bool,
    pub right_click: bool,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ParsedInput::default())
            .add_plugins(KeyboardPlugin)
            .add_plugins(CursorPlugin)
            .add_plugins(MousePlugin);
    }
}
