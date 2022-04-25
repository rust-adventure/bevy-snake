use std::collections::VecDeque;

use bevy::{
    math::Quat,
    prelude::{Query, Res, Transform},
    sprite::TextureAtlasSprite,
};
use itertools::Itertools;

use crate::board::Position;

#[derive(Debug)]
pub struct Snake {
    pub segments: VecDeque<Position>,
}

impl Default for Snake {
    fn default() -> Self {
        Self {
            segments: VecDeque::from([
                Position { x: 4, y: 4 },
                Position { x: 3, y: 4 },
            ]),
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
fn detect_side(
    origin: &Position,
    other: &Position,
) -> Direction {
    // dbg!(origin, other);
    if other.y > origin.y {
        Direction::Up
    } else if other.y < origin.y {
        Direction::Down
    } else if other.x > origin.x {
        Direction::Right
    } else if other.x < origin.x {
        Direction::Left
    } else {
        panic!("should never happen");
    }
}

pub fn render_snake_segments(
    snake: Res<Snake>,
    mut positions: Query<(
        &Position,
        &mut TextureAtlasSprite,
        &mut Transform,
    )>,
) {
    let snake_texture_index = 0;

    let head = positions
        .iter_mut()
        .find(|pos| pos.0 == &snake.segments[0]);

    match head {
        Some((pos, mut sprite, mut transform)) => {
            let rotation = match detect_side(
                pos,
                &snake.segments[1],
            ) {
                Direction::Up => Quat::from_rotation_z(0.0),
                Direction::Down => Quat::from_rotation_z(
                    std::f32::consts::PI,
                ),
                Direction::Left => Quat::from_rotation_z(
                    std::f32::consts::FRAC_PI_2,
                ),
                Direction::Right => Quat::from_rotation_z(
                    -std::f32::consts::FRAC_PI_2,
                ),
            };
            sprite.index = snake_texture_index;
            transform.rotation = rotation;
        }
        None => {}
    }

    let tail = positions.iter_mut().find(|pos| {
        pos.0 == &snake.segments[snake.segments.len() - 1]
    });

    match tail {
        Some((pos, mut sprite, mut transform)) => {
            let rotation = match detect_side(
                pos,
                &snake.segments[snake.segments.len() - 2],
            ) {
                Direction::Up => Quat::from_rotation_z(0.0),
                Direction::Down => Quat::from_rotation_z(
                    std::f32::consts::PI,
                ),
                Direction::Left => Quat::from_rotation_z(
                    std::f32::consts::FRAC_PI_2,
                ),
                Direction::Right => Quat::from_rotation_z(
                    -std::f32::consts::FRAC_PI_2,
                ),
            };

            sprite.index = snake_texture_index + 3;
            transform.rotation = rotation;
        }
        None => {}
    }

    for (front, origin, back) in
        snake.segments.iter().tuple_windows()
    {
        let a = detect_side(origin, front);
        let b = detect_side(origin, back);

        let image = match (a, b) {
            // vertical
            (Direction::Down, Direction::Up)
            | (Direction::Up, Direction::Down) => (
                snake_texture_index + 1,
                Quat::from_rotation_z(0.0),
            ),
            // horizontal
            (Direction::Right, Direction::Left)
            | (Direction::Left, Direction::Right) => (
                snake_texture_index + 1,
                Quat::from_rotation_z(
                    std::f32::consts::FRAC_PI_2,
                ),
            ),
            // ⌞
            (Direction::Up, Direction::Right)
            | (Direction::Right, Direction::Up) => (
                snake_texture_index + 2,
                Quat::from_rotation_z(
                    std::f32::consts::FRAC_PI_2,
                ),
            ),
            // ⌜
            (Direction::Right, Direction::Down)
            | (Direction::Down, Direction::Right) => (
                snake_texture_index + 2,
                Quat::from_rotation_z(0.0),
            ),
            // ⌟
            (Direction::Left, Direction::Up)
            | (Direction::Up, Direction::Left) => (
                snake_texture_index + 2,
                Quat::from_rotation_z(std::f32::consts::PI),
            ),
            // ⌝
            (Direction::Left, Direction::Down)
            | (Direction::Down, Direction::Left) => (
                snake_texture_index + 2,
                Quat::from_rotation_z(
                    -std::f32::consts::FRAC_PI_2,
                ),
            ),
            _ => panic!("unhandled"),
        };
        let current_position = positions
            .iter_mut()
            .find(|pos| pos.0 == origin);

        match current_position {
            Some((_, mut sprite, mut transform)) => {
                sprite.index = image.0;
                transform.rotation = image.1;
            }
            None => {}
        }
    }
}
