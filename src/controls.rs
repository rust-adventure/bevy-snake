use crate::GameState;
use bevy::{ecs::schedule::IntoScheduleConfigs, prelude::*};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Direction>()
            .add_systems(Update, user_input.run_if(in_state(GameState::Playing)));
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

impl Direction {
    pub fn as_ivec2(&self) -> IVec2 {
        match self {
            Direction::Up => IVec2::Y,
            Direction::Down => IVec2::NEG_Y,
            Direction::Right => IVec2::X,
            Direction::Left => IVec2::NEG_X,
        }
    }
}

fn user_input(input: Res<ButtonInput<KeyCode>>, mut last_pressed: ResMut<Direction>) {
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
