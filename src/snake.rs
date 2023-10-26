use std::collections::VecDeque;

use bevy::{
    math::Quat,
    prelude::{Query, Res, Resource, Transform},
    sprite::TextureAtlasSprite,
};
use itertools::Itertools;

use crate::{board::Position, settings::GameSettings};

#[derive(Debug, Resource)]
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

impl From<Direction> for Quat {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Quat::from_rotation_z(0.0),
            Direction::Down => {
                Quat::from_rotation_z(std::f32::consts::PI)
            }
            Direction::Left => Quat::from_rotation_z(
                std::f32::consts::FRAC_PI_2,
            ),
            Direction::Right => Quat::from_rotation_z(
                -std::f32::consts::FRAC_PI_2,
            ),
        }
    }
}

pub fn render_snake_segments(
    snake: Res<Snake>,
    mut positions: Query<(
        &Position,
        &mut TextureAtlasSprite,
        &mut Transform,
    )>,
    settings: Res<GameSettings>,
) {
    let snake_texture_index = settings.snake_index;

    let head = positions
        .iter_mut()
        .find(|pos| pos.0 == &snake.segments[0]);

    if let Some((pos, mut sprite, mut transform)) = head {
        let rotation = Quat::from(detect_side(
            pos,
            &snake.segments[1],
        ));

        sprite.index = snake_texture_index;
        transform.rotation = rotation;
    }

    let tail = positions.iter_mut().find(|pos| {
        pos.0 == &snake.segments[snake.segments.len() - 1]
    });

    if let Some((pos, mut sprite, mut transform)) = tail {
        let rotation = Quat::from(detect_side(
            pos,
            &snake.segments[snake.segments.len() - 2],
        ));

        sprite.index = snake_texture_index + 3;
        transform.rotation = rotation;
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

        if let Some((_, mut sprite, mut transform)) =
            current_position
        {
            sprite.index = image.0;
            transform.rotation = image.1;
        }
    }
}
