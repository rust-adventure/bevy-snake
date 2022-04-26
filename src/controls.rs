use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Direction>().add_system(
            user_input.run_in_state(GameState::Playing),
        );
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

use crate::GameState;

impl Default for Direction {
    fn default() -> Self {
        Right
    }
}

fn user_input(
    input: Res<Input<KeyCode>>,
    mut last_pressed: ResMut<Direction>,
) {
    if input.pressed(KeyCode::Up) {
        *last_pressed = Up;
    } else if input.pressed(KeyCode::Down) {
        *last_pressed = Down;
    } else if input.pressed(KeyCode::Left) {
        *last_pressed = Left;
    } else if input.pressed(KeyCode::Right) {
        *last_pressed = Right;
    }
}
