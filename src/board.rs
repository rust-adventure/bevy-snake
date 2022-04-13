use bevy::{
    ecs::system::Command,
    prelude::*,
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
};
use itertools::Itertools;
use rand::{
    distributions::WeightedIndex, prelude::Distribution,
};

use crate::{colors::MATERIALS, food::Food};

const TILE_SIZE: f32 = 30.0;
const TILE_SPACER: f32 = 0.0;

#[derive(
    Debug, PartialEq, Copy, Clone, Eq, Hash, Component,
)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

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

pub fn spawn_board(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let board = Board::new(20);

    let texture_handle = asset_server.load("grass.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16.0, 16.0),
        3,
        1,
    );
    let texture_atlas_handle =
        texture_atlases.add(texture_atlas);

    let mut rng = rand::thread_rng();
    let weights = vec![3, 3, 1];
    let dist = WeightedIndex::new(weights).unwrap();

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: MATERIALS.board,
                custom_size: Some(Vec2::new(
                    board.physical_size,
                    board.physical_size,
                )),
                ..Sprite::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            for tile in (0..board.size)
                .cartesian_product(0..board.size)
            {
                builder.spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle
                        .clone(),
                    sprite: TextureAtlasSprite {
                        index: dist.sample(&mut rng),
                        custom_size: Some(Vec2::new(
                            TILE_SIZE, TILE_SIZE,
                        )),
                        ..TextureAtlasSprite::default()
                    },
                    transform: Transform::from_xyz(
                        board.cell_position_to_physical(
                            tile.0,
                        ),
                        board.cell_position_to_physical(
                            tile.1,
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
    fn write(self, world: &mut World) {
        let texture_atlas_handle = {
            let asset_server = world
                .get_resource::<AssetServer>()
                .unwrap();
            let texture_handle =
                asset_server.load("snake-sprites.png");

            let mut texture_atlases = world
                .get_resource_mut::<Assets<TextureAtlas>>()
                .unwrap();

            let texture_atlas = TextureAtlas::from_grid(
                texture_handle,
                Vec2::new(136.0, 136.0),
                4,
                30,
            );

            texture_atlases.add(texture_atlas)
        };
        let (x, y) = {
            let board = world
                .query::<&Board>()
                .iter(&world)
                .next()
                .unwrap();
            (
                board.cell_position_to_physical(
                    self.position.x,
                ),
                board.cell_position_to_physical(
                    self.position.y,
                ),
            )
        };

        // Get resources, edit entity, etc.
        world
            .spawn()
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_xyz(x, y, 2.0),
                sprite: TextureAtlasSprite {
                    index: 116,
                    custom_size: Some(Vec2::new(
                        TILE_SIZE, TILE_SIZE,
                    )),
                    ..Default::default()
                },
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
        let apple = {
            let asset_server = world
                .get_resource::<AssetServer>()
                .unwrap();

            asset_server.load("apple.png")
        };
        let (x, y) = {
            let board = world
                .query::<&Board>()
                .iter(&world)
                .next()
                .unwrap();
            (
                board.cell_position_to_physical(
                    self.position.x,
                ),
                board.cell_position_to_physical(
                    self.position.y,
                ),
            )
        };

        // Get resources, edit entity, etc.
        world
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    // color: MATERIALS.food,
                    custom_size: Some(Vec2::new(
                        TILE_SIZE, TILE_SIZE,
                    )),
                    ..Sprite::default()
                },
                texture: apple,
                transform: Transform::from_xyz(x, y, 2.0),
                ..Default::default()
            })
            .insert(self.position)
            .insert(Food);
    }
}
