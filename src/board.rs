use bevy::{
    ecs::system::Command, prelude::*, sprite::Anchor,
};
use itertools::Itertools;
use rand::{
    distributions::WeightedIndex, prelude::Distribution,
};

use crate::{
    assets::{FontAssets, ImageAssets},
    colors::COLORS,
    food::Food,
    scoring::{HighScore, Score},
    ui::{HighScoreDisplay, ScoreDisplay},
};

const TILE_SIZE: f32 = 30.0;
const TILE_SPACER: f32 = 0.0;
const OFFSET_TEXT_FROM_BOARD: f32 = 15.0;

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
    fonts: Res<FontAssets>,
    score: Res<Score>,
    high_score: Res<HighScore>,
) {
    let board = Board::new(20);

    let mut rng = rand::thread_rng();
    let weights = vec![3, 3, 1];
    let dist = WeightedIndex::new(weights).unwrap();

    let zero = board.cell_position_to_physical(0);
    let max = board.cell_position_to_physical(board.size);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: COLORS.board,
                custom_size: Some(Vec2::splat(
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
                        custom_size: Some(Vec2::splat(
                            TILE_SIZE,
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

    // spawn scoring
    let alfa_style = TextStyle {
        font: fonts.alfa_slab_one_regular.clone(),
        font_size: 25.0,
        color: Color::BLACK,
    };
    let roboto_style = TextStyle {
        font: fonts.roboto.clone(),
        font_size: 30.0,
        color: Color::BLACK,
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_sections(vec![
                TextSection {
                    value: "Current Score\n".to_string(),
                    style: alfa_style.clone(),
                },
                TextSection {
                    value: "0".to_string(),
                    style: roboto_style.clone(),
                },
                TextSection {
                    value: " apples".to_string(),
                    style: roboto_style.clone(),
                },
                TextSection {
                    value: "\nTime\n".to_string(),
                    style: alfa_style.clone(),
                },
                TextSection {
                    value: "0".to_string(),
                    style: roboto_style.clone(),
                },
                TextSection {
                    value: " seconds".to_string(),
                    style: roboto_style.clone(),
                },
            ])
            .with_alignment(TextAlignment::Right),
            transform: Transform::from_xyz(
                (zero - 0.5 * TILE_SIZE)
                    - OFFSET_TEXT_FROM_BOARD,
                max - 0.5 * TILE_SIZE,
                1.0,
            ),
            text_anchor: Anchor::TopRight,
            ..default()
        },
        ScoreDisplay,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_sections(vec![
                TextSection {
                    value: "High Score\n".to_string(),
                    style: alfa_style.clone(),
                },
                TextSection {
                    value: "".to_string(),
                    style: roboto_style.clone(),
                },
                TextSection {
                    value: " apples".to_string(),
                    style: roboto_style.clone(),
                },
                TextSection {
                    value: "\nBest Time\n".to_string(),
                    style: alfa_style.clone(),
                },
                TextSection {
                    value: high_score
                        .time
                        .as_secs()
                        .to_string(),
                    style: roboto_style.clone(),
                },
                TextSection {
                    value: " seconds".to_string(),
                    style: roboto_style.clone(),
                },
            ]),
            transform: Transform::from_xyz(
                (max - 0.5 * TILE_SIZE)
                    + OFFSET_TEXT_FROM_BOARD,
                max - 0.5 * TILE_SIZE,
                1.0,
            ),
            text_anchor: Anchor::TopLeft,
            ..default()
        },
        HighScoreDisplay,
    ));
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
