use bevy::{ecs::system::Command, prelude::*};
use itertools::Itertools;
use rand::{
    distributions::WeightedIndex, prelude::Distribution,
};

use crate::{
    assets::ImageAssets, colors::COLORS, food::Food,
};

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
    images: Res<ImageAssets>,
) {
    let board = Board::new(20);

    let mut rng = rand::thread_rng();
    let weights = vec![3, 3, 1];
    let dist = WeightedIndex::new(weights).unwrap();

    commands
        .spawn(SpriteBundle {
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
                builder.spawn(SpriteSheetBundle {
                    texture_atlas: images.grass.clone(),
                    sprite: TextureAtlasSprite {
                        index: dist.sample(&mut rng),
                        custom_size: Some(Vec2::new(
                            TILE_SIZE, TILE_SIZE,
                        )),
                        ..TextureAtlasSprite::default()
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
}

pub struct SpawnSnakeSegment {
    pub position: Position,
}

impl Command for SpawnSnakeSegment {
    fn apply(self, world: &mut World) {
        let board = world
            .query::<&Board>()
            .iter(&world)
            .next()
            .unwrap();
        let x = board
            .cell_position_to_physical(self.position.x);
        let y = board
            .cell_position_to_physical(self.position.y);

        let snake = world
            .get_resource::<ImageAssets>()
            .unwrap()
            .snake
            .clone();

        world.spawn((
            SpriteSheetBundle {
                texture_atlas: snake,
                sprite: TextureAtlasSprite {
                    index: 8,
                    custom_size: Some(Vec2::new(
                        TILE_SIZE, TILE_SIZE,
                    )),
                    ..TextureAtlasSprite::default()
                },
                transform: Transform::from_xyz(x, y, 2.0),
                ..Default::default()
            },
            self.position,
        ));
    }
}

pub struct SpawnApple {
    pub position: Position,
}

impl Command for SpawnApple {
    fn apply(self, world: &mut World) {
        let board = world
            .query::<&Board>()
            .iter(&world)
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
                    custom_size: Some(Vec2::new(
                        TILE_SIZE, TILE_SIZE,
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
