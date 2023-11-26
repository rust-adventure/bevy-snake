use bevy::prelude::*;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Direction>()
            .add_systems(Update, user_input);
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
    input: Res<Input<KeyCode>>,
    mut last_pressed: ResMut<Direction>,
) {
    if input.pressed(KeyCode::Up) {
        *last_pressed = Direction::Up;
    } else if input.pressed(KeyCode::Down) {
        *last_pressed = Direction::Down;
    } else if input.pressed(KeyCode::Left) {
        *last_pressed = Direction::Left;
    } else if input.pressed(KeyCode::Right) {
        *last_pressed = Direction::Right;
    }
}
