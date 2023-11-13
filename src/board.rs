use crate::colors;
use bevy::prelude::*;
use itertools::Itertools;

pub const TILE_SIZE: f32 = 30.0;
const TILE_SPACER: f32 = 0.0;

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
}

#[derive(Debug, Component)]
pub struct Position(pub IVec2);

pub fn spawn_board(
    mut commands: Commands,
    board: Res<Board>,
) {
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
        .with_children(|parent| {
            for (x, y) in (0..board.size)
                .cartesian_product(0..board.size)
            {
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: if (x + y) % 2 == 0 {
                            colors::TILE_PLACEHOLDER
                        } else {
                            colors::TILE_PLACEHOLDER_DARK
                        },
                        custom_size: Some(Vec2::splat(
                            TILE_SIZE,
                        )),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        board.cell_position_to_physical(
                            i32::from(x),
                        ),
                        board.cell_position_to_physical(
                            i32::from(y),
                        ),
                        1.0,
                    ),
                    ..default()
                });
            }
        });
}
