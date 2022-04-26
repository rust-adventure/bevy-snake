use bevy::{ecs::system::Command, prelude::*};
use itertools::Itertools;

use crate::{colors::COLORS, food::Food, snake::Snake};

const TILE_SIZE: f32 = 30.0;
const TILE_SPACER: f32 = 0.0;

#[derive(Component)]
pub struct Board {
    pub size: u8,
    physical_size: f32,
}

impl Board {
    fn new(size: u8) -> Self {
        let physical_size = f32::from(size) * TILE_SIZE
            + f32::from(size + 1) * TILE_SPACER;
        Board {
            size,
            physical_size,
        }
    }
    fn cell_position_to_physical(&self, pos: u8) -> f32 {
        let offset =
            -self.physical_size / 2.0 + 0.5 * TILE_SIZE;

        offset
            + f32::from(pos) * TILE_SIZE
            + f32::from(pos + 1) * TILE_SPACER
    }
}

#[derive(
    Debug, PartialEq, Copy, Clone, Eq, Hash, Component,
)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

pub fn spawn_board(
    mut commands: Commands,
    snake: Res<Snake>,
) {
    let board = Board::new(20);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: COLORS.board,
                custom_size: Some(Vec2::new(
                    board.physical_size,
                    board.physical_size,
                )),
                ..Sprite::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            for (x, y) in (0..board.size)
                .cartesian_product(0..board.size)
            {
                builder.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: if (x + y) % 2 == 0 {
                            COLORS.tile_placeholder
                        } else {
                            COLORS.tile_placeholder_dark
                        },
                        custom_size: Some(Vec2::new(
                            TILE_SIZE, TILE_SIZE,
                        )),
                        ..Sprite::default()
                    },
                    transform: Transform::from_xyz(
                        board.cell_position_to_physical(x),
                        board.cell_position_to_physical(y),
                        1.0,
                    ),
                    ..Default::default()
                });
            }
        })
        .insert(board);

    for segment in snake.segments.iter() {
        commands.add({
            SpawnSnakeSegment { position: *segment }
        });
    }
    commands.add(SpawnApple {
        position: Position { x: 15, y: 15 },
    })
}

pub struct SpawnSnakeSegment {
    pub position: Position,
}

impl Command for SpawnSnakeSegment {
    fn write(self, world: &mut World) {
        let board = world
            .query::<&Board>()
            .iter(&world)
            .next()
            .unwrap();

        let x = board
            .cell_position_to_physical(self.position.x);
        let y = board
            .cell_position_to_physical(self.position.y);

        world
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: COLORS.snake,
                    custom_size: Some(Vec2::new(
                        TILE_SIZE, TILE_SIZE,
                    )),
                    ..Sprite::default()
                },
                transform: Transform::from_xyz(x, y, 2.0),
                ..Default::default()
            })
            .insert(self.position);
    }
}

pub struct SpawnApple {
    pub position: Position,
}

impl Command for SpawnApple {
    fn write(self, world: &mut World) {
        let board = world
            .query::<&Board>()
            .iter(&world)
            .next()
            .unwrap();
        let x = board
            .cell_position_to_physical(self.position.x);
        let y = board
            .cell_position_to_physical(self.position.y);

        world
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: COLORS.food,
                    custom_size: Some(Vec2::new(
                        TILE_SIZE, TILE_SIZE,
                    )),
                    ..Sprite::default()
                },
                transform: Transform::from_xyz(x, y, 2.0),
                ..Default::default()
            })
            .insert(self.position)
            .insert(Food);
    }
}
