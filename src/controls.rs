use crate::GameState;
use bevy::{
    ecs::schedule::IntoScheduleConfigs, prelude::*,
};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Direction>().add_systems(
            Update,
            user_input.run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Resource, Default)]
pub enum Direction {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

fn user_input(
    input: Res<ButtonInput<KeyCode>>,
    mut last_pressed: ResMut<Direction>,
) {
    if input.pressed(KeyCode::ArrowUp) {
        *last_pressed = Direction::Up;
    } else if input.pressed(KeyCode::ArrowDown) {
        *last_pressed = Direction::Down;
    } else if input.pressed(KeyCode::ArrowLeft) {
        *last_pressed = Direction::Left;
    } else if input.pressed(KeyCode::ArrowRight) {
        *last_pressed = Direction::Right;
    }
}
