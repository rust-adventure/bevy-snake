use crate::GameState;
use bevy::prelude::{
    in_state, App, ButtonInput, IntoSystemConfigs, KeyCode,
    Plugin, Res, ResMut, Resource, Update,
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
use Direction::*;

fn user_input(
    input: Res<ButtonInput<KeyCode>>,
    mut last_pressed: ResMut<Direction>,
) {
    if input.pressed(KeyCode::ArrowUp) {
        *last_pressed = Up;
    } else if input.pressed(KeyCode::ArrowDown) {
        *last_pressed = Down;
    } else if input.pressed(KeyCode::ArrowLeft) {
        *last_pressed = Left;
    } else if input.pressed(KeyCode::ArrowRight) {
        *last_pressed = Right;
    }
}
