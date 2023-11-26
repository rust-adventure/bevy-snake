use bevy::{ecs::system::Command, prelude::*};
use std::collections::VecDeque;

use crate::{
    board::{Board, Position, TILE_SIZE},
    colors,
};

#[derive(Debug, Default, Resource)]
pub struct Snake {
    pub segments: VecDeque<Entity>,
}

pub fn spawn_snake(mut commands: Commands) {
    for position in
        [Position::new(3, 4), Position::new(4, 4)]
    {
        commands.add(SpawnSnakeSegment { position });
    }
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

        let entity = world
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
                        x, y, 2.0,
                    ),
                    ..default()
                },
                self.position,
            ))
            .id();

        let mut snake =
            world.get_resource_mut::<Snake>().unwrap();

        snake.segments.push_front(entity);
    }
}
