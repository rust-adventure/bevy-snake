use bevy::{ecs::system::Command, prelude::*};
use itertools::Itertools;
use rand::{
    distributions::WeightedIndex, prelude::Distribution,
};
pub mod position;
use position::*;

use crate::{
    assets::ImageAssets, colors, food::Food, snake::Snake,
};

const TILE_SIZE: f32 = 30.0;
const TILE_SPACER: f32 = 0.0;

#[derive(Component)]
pub struct Board {
    pub size: u16,
    physical_size: f32,
}

impl Board {
    fn new(size: u16) -> Self {
        let physical_size = f32::from(size) * TILE_SIZE
            + f32::from(size + 1) * TILE_SPACER;
        Board {
            size,
            physical_size,
        }
    }
    fn cell_position_to_physical(&self, pos: i32) -> f32 {
        // let pos_f32: f32 = pos.try_into().unwrap();
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
) {
    let board = Board::new(20);

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
                ..Sprite::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            for pos in board.tiles() {
                builder.spawn(SpriteSheetBundle {
                    texture_atlas: images.grass.clone(),
                    sprite: TextureAtlasSprite {
                        index: dist.sample(&mut rng),
                        custom_size: Some(Vec2::splat(
                            TILE_SIZE,
                        )),
                        ..TextureAtlasSprite::default()
                    },
                    transform: Transform::from_xyz(
                        board.cell_position_to_physical(
                            pos.x,
                        ),
                        board.cell_position_to_physical(
                            pos.y,
                        ),
                        1.0,
                    ),
                    ..Default::default()
                });
            }
        })
        .insert(board);
}

pub struct SpawnSnakeSegment {
    pub position: Position,
}

impl Command for SpawnSnakeSegment {
    fn apply(self, world: &mut World) {
        let board = world
            .query::<&Board>()
            .iter(world)
            .next()
            .unwrap();
        let x = board
            .cell_position_to_physical(self.position.x);
        let y = board
            .cell_position_to_physical(self.position.y);

        let snake_atlas = world
            .get_resource::<ImageAssets>()
            .unwrap()
            .snake
            .clone();

        let entity = world
            .spawn((
                SpriteSheetBundle {
                    texture_atlas: snake_atlas,
                    sprite: TextureAtlasSprite {
                        index: 8,
                        custom_size: Some(Vec2::splat(
                            TILE_SIZE,
                        )),
                        ..TextureAtlasSprite::default()
                    },
                    transform: Transform::from_xyz(
                        x, y, 2.0,
                    ),
                    ..Default::default()
                },
                self.position,
            ))
            .id();

        let mut snake =
            world.get_resource_mut::<Snake>().unwrap();

        snake.segments.push_front(entity);
    }
}

pub struct SpawnApple {
    pub position: Position,
}

impl Command for SpawnApple {
    fn apply(self, world: &mut World) {
        let board = world
            .query::<&Board>()
            .iter(world)
            .next()
            .unwrap();
        let x = board
            .cell_position_to_physical(self.position.x);
        let y = board
            .cell_position_to_physical(self.position.y);

        let apple = world
            .get_resource::<ImageAssets>()
            .unwrap()
            .apple
            .clone();

        world.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(
                        TILE_SIZE,
                    )),
                    ..Sprite::default()
                },
                texture: apple,
                transform: Transform::from_xyz(x, y, 2.0),
                ..Default::default()
            },
            self.position,
            Food,
        ));
    }
}
