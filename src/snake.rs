use bevy::prelude::*;
use std::collections::VecDeque;

use crate::{
    board::{Board, Position, TILE_SIZE},
    colors,
};

#[derive(Debug, Default, Resource)]
pub struct Snake {
    pub segments: VecDeque<Entity>,
}

pub fn spawn_snake(
    mut commands: Commands,
    board: Res<Board>,
    mut snake: ResMut<Snake>,
) {
    for position in
        [Position::new(3, 4), Position::new(4, 4)]
    {
        let entity = commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: colors::SNAKE,
                        custom_size: Some(Vec2::splat(
                            TILE_SIZE,
                        )),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        board.cell_position_to_physical(
                            position.x,
                        ),
                        board.cell_position_to_physical(
                            position.y,
                        ),
                        2.0,
                    ),
                    ..default()
                },
                position,
            ))
            .id();

        snake.segments.push_front(entity);
    }
}
