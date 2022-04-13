use bevy::prelude::*;
use itertools::Itertools;
use std::collections::VecDeque;

use crate::board::{
    Board, Position, SpawnApple, SpawnSnakeSegment,
};

#[derive(Debug)]
pub struct SnakeBody {
    pub segments: VecDeque<Position>,
}

impl Default for SnakeBody {
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

pub fn spawn_snake(
    mut commands: Commands,
    query_board: Query<&Board>,
    snake: Res<SnakeBody>,
) {
    let board = query_board.single();

    for position in snake.segments.iter() {
        commands.add(SpawnSnakeSegment {
            position: *position,
        });
    }

    commands.add(SpawnApple {
        position: Position {
            x: board.size / 2,
            y: board.size / 2,
        },
    });
}

#[tracing::instrument(skip(positions,))]
pub fn render_snake_segments(
    snake: Res<SnakeBody>,
    mut positions: Query<(
        &Position,
        &mut TextureAtlasSprite,
        &mut Transform,
    )>,
) {
    if snake.segments.len() > 1 {
        let current_position = positions
            .iter_mut()
            .find(|pos| pos.0 == &snake.segments[0]);

        match current_position {
            Some((pos, mut sprite, mut transform)) => {
                let rotation = match detect_side(
                    pos,
                    &snake.segments[1],
                ) {
                    Direction::Up => {
                        Quat::from_rotation_z(0.0)
                    }
                    Direction::Down => {
                        Quat::from_rotation_z(
                            std::f32::consts::PI,
                        )
                    }
                    Direction::Left => {
                        Quat::from_rotation_z(
                            std::f32::consts::FRAC_PI_2,
                        )
                    }
                    Direction::Right => {
                        Quat::from_rotation_z(
                            -std::f32::consts::FRAC_PI_2,
                        )
                    }
                };
                sprite.index = 116;
                transform.rotation = rotation;
            }
            None => {}
        }
    }

    if snake.segments.len() > 1 {
        let current_position =
            positions.iter_mut().find(|pos| {
                pos.0
                    == &snake.segments
                        [snake.segments.len() - 1]
            });

        match current_position {
            Some((pos, mut sprite, mut transform)) => {
                let rotation = match detect_side(
                    pos,
                    &snake.segments
                        [snake.segments.len() - 2],
                ) {
                    Direction::Up => {
                        Quat::from_rotation_z(0.0)
                    }
                    Direction::Down => {
                        Quat::from_rotation_z(
                            std::f32::consts::PI,
                        )
                    }
                    Direction::Left => {
                        Quat::from_rotation_z(
                            std::f32::consts::FRAC_PI_2,
                        )
                    }
                    Direction::Right => {
                        Quat::from_rotation_z(
                            -std::f32::consts::FRAC_PI_2,
                        )
                    }
                };
                sprite.index = 119;
                transform.rotation = rotation;
            }
            None => {}
        }
    }

    for (front, origin, back) in
        snake.segments.iter().tuple_windows()
    {
        let a = detect_side(origin, front);
        let b = detect_side(origin, back);

        let image = match (a, b) {
            // vertical
            (Direction::Down, Direction::Up) => {
                (117, Quat::from_rotation_z(0.0))
            }
            (Direction::Up, Direction::Down) => {
                (117, Quat::from_rotation_z(0.0))
            }
            // horizontal
            (Direction::Right, Direction::Left) => (
                117,
                Quat::from_rotation_z(
                    std::f32::consts::FRAC_PI_2,
                ),
            ),
            (Direction::Left, Direction::Right) => (
                117,
                Quat::from_rotation_z(
                    std::f32::consts::FRAC_PI_2,
                ),
            ),
            // ⌞
            (Direction::Up, Direction::Right) => (
                118,
                Quat::from_rotation_z(
                    std::f32::consts::FRAC_PI_2,
                ),
            ),
            (Direction::Right, Direction::Up) => (
                118,
                Quat::from_rotation_z(
                    std::f32::consts::FRAC_PI_2,
                ),
            ),
            // ⌜
            (Direction::Right, Direction::Down) => {
                (118, Quat::from_rotation_z(0.0))
            }
            (Direction::Down, Direction::Right) => {
                (118, Quat::from_rotation_z(0.0))
            }
            // ⌟
            (Direction::Left, Direction::Up) => (
                118,
                Quat::from_rotation_z(std::f32::consts::PI),
            ),
            (Direction::Up, Direction::Left) => (
                118,
                Quat::from_rotation_z(std::f32::consts::PI),
            ),
            // ⌝
            (Direction::Left, Direction::Down) => (
                118,
                Quat::from_rotation_z(
                    -std::f32::consts::FRAC_PI_2,
                ),
            ),
            (Direction::Down, Direction::Left) => (
                118,
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

#[tracing::instrument]
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
        info!(?origin, ?other);
        panic!("should never happen");
    }
}
