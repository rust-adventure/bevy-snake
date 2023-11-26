use bevy::{ecs::system::Command, prelude::*};

use crate::{
    board::{Board, Position, TILE_SIZE},
    colors,
};

pub fn spawn_apple(mut commands: Commands) {
    commands.add(SpawnApple {
        position: Position::new(15, 15),
    })
}

#[derive(Component)]
pub struct Food;

pub struct SpawnApple {
    pub position: Position,
}

impl Command for SpawnApple {
    fn apply(self, world: &mut World) {
        let board = world.get_resource::<Board>().unwrap();
        let x = board
            .cell_position_to_physical(self.position.x);
        let y = board
            .cell_position_to_physical(self.position.y);

        world.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(
                        TILE_SIZE,
                    )),
                    color: colors::FOOD,
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 2.0),
                ..default()
            },
            self.position,
            Food,
        ));
    }
}
