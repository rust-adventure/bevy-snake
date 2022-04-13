use bevy::prelude::*;

use crate::snake::SnakeBody;

pub struct LastKeyPress(pub KeyCode);

impl Default for LastKeyPress {
    fn default() -> Self {
        Self(KeyCode::Right)
    }
}

pub fn user_input(
    input: Res<Input<KeyCode>>,
    snake: Res<SnakeBody>,
    mut last_pressed: ResMut<LastKeyPress>,
) {
    let head = snake.segments[0];
    let neck = snake.segments.get(1);

    if input.pressed(KeyCode::Up) {
        match (head, neck) {
            (h, Some(n)) if (h.y + 1) == n.y => {}
            _ => {
                last_pressed.0 = KeyCode::Up;
            }
        }
    } else if input.pressed(KeyCode::Down) {
        match (head, neck) {
            (h, Some(n))
                if h.y != 0 && (h.y - 1) == n.y => {}
            _ => {
                last_pressed.0 = KeyCode::Down;
            }
        }
    } else if input.pressed(KeyCode::Left) {
        match (head, neck) {
            (h, Some(n))
                if h.x != 0 && (h.x - 1) == n.x => {}
            _ => {
                last_pressed.0 = KeyCode::Left;
            }
        }
    } else if input.pressed(KeyCode::Right) {
        match (head, neck) {
            (h, Some(n)) if (h.x + 1) == n.x => {}
            _ => {
                last_pressed.0 = KeyCode::Right;
            }
        }
    }
}
