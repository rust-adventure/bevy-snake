use bevy::{ecs::world::Command, prelude::*};
use itertools::Itertools;
use rand::{
    distributions::WeightedIndex, prelude::Distribution,
};
pub mod position;
use position::*;

use crate::{assets::ImageAssets, colors, snake::Snake};

pub const TILE_SIZE: f32 = 30.0;
pub const TILE_SPACER: f32 = 0.0;

#[derive(Resource)]
pub struct Board {
    pub size: u16,
    physical_size: f32,
}

impl Board {
    pub fn new(size: u16) -> Self {
        let physical_size = f32::from(size) * TILE_SIZE
            + f32::from(size + 1) * TILE_SPACER;
        Board {
            size,
            physical_size,
        }
    }
    pub fn cell_position_to_physical(
        &self,
        pos: i32,
    ) -> f32 {
        let offset =
            -self.physical_size / 2.0 + 0.5 * TILE_SIZE;

        offset
            + pos as f32 * TILE_SIZE
            + (pos + 1) as f32 * TILE_SPACER
    }
    pub fn low_edge(&self) -> f32 {
        -self.physical_size / 2.0
    }
    pub fn high_edge(&self) -> f32 {
        self.physical_size / 2.0
    }
    pub fn tiles(&self) -> impl Iterator<Item = Position> {
        (0..self.size).cartesian_product(0..self.size).map(
            |(x, y)| {
                Position(IVec2::new(
                    i32::from(x),
                    i32::from(y),
                ))
            },
        )
    }
}

pub fn spawn_board(
    mut commands: Commands,
    images: Res<ImageAssets>,
    board: Res<Board>,
) {
    let mut rng = rand::thread_rng();
    let weights = vec![3, 3, 1];
    let dist = WeightedIndex::new(weights).unwrap();

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: colors::BOARD,
                custom_size: Some(Vec2::splat(
                    board.physical_size,
                )),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            for pos in board.tiles() {
                builder.spawn((
                    SpriteBundle {
                        texture: images.grass.clone(),
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(
                                TILE_SIZE,
                            )),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            board
                                .cell_position_to_physical(
                                    pos.x,
                                ),
                            board
                                .cell_position_to_physical(
                                    pos.y,
                                ),
                            1.0,
                        ),
                        ..default()
                    },
                    TextureAtlas {
                        layout: images.grass_layout.clone(),
                        index: dist.sample(&mut rng),
                    },
                ));
            }
        });
}

pub struct SpawnSnakeSegment {
    pub position: Position,
}

impl Command for SpawnSnakeSegment {
    fn apply(self, world: &mut World) {
        let board = world.get_resource::<Board>().unwrap();
        let x = board
            .cell_position_to_physical(self.position.x);
        let y = board
            .cell_position_to_physical(self.position.y);

        let snake_image = world
            .get_resource::<ImageAssets>()
            .unwrap()
            .snake
            .clone();

        let snake_layout = world
            .get_resource::<ImageAssets>()
            .unwrap()
            .snake_layout
            .clone();

        let entity = world
            .spawn((
                SpriteBundle {
                    texture: snake_image,
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(
                            TILE_SIZE,
                        )),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        x, y, 2.0,
                    ),
                    ..default()
                },
                TextureAtlas {
                    index: 8,
                    layout: snake_layout.clone(),
                },
                self.position,
            ))
            .id();

        let mut snake =
            world.get_resource_mut::<Snake>().unwrap();

        snake.segments.push_front(entity);
    }
}
