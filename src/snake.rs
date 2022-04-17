use bevy::prelude::*;
use itertools::Itertools;
use std::collections::VecDeque;

use crate::{
    board::{
        Board, Position, SpawnApple, SpawnSnakeSegment,
    },
    food::NewFoodEvent,
};

pub struct SnakeTextureSelection(pub usize);

impl Default for SnakeTextureSelection {
    fn default() -> Self {
        SnakeTextureSelection(116)
    }
}

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

pub fn new_game_spawns(
    mut commands: Commands,
    snake: Res<SnakeBody>,
    mut food_events: EventWriter<NewFoodEvent>,
) {
    for position in snake.segments.iter() {
        commands.add(SpawnSnakeSegment {
            position: *position,
        });
    }

    food_events.send(NewFoodEvent);

    // commands.add(SpawnApple {
    //     position: Position {
    //         x: board.size / 2,
    //         y: board.size / 2,
    //     },
    // });
}

pub fn render_snake_segments(
    snake: Res<SnakeBody>,
    snake_index: Res<SnakeTextureSelection>,
    mut positions: Query<(
        &Position,
        &mut TextureAtlasSprite,
        &mut Transform,
    )>,
) {
    let snake_texture_index = snake_index.0;

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
                sprite.index = snake_texture_index;
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
                dbg!(snake_texture_index + 3);
                sprite.index = snake_texture_index + 3;
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
