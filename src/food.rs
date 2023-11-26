use bevy::{ecs::system::Command, prelude::*};
use rand::seq::IteratorRandom;

use crate::{
    board::{Board, Position, TILE_SIZE},
    colors,
};

pub fn spawn_apple(mut commands: Commands) {
    commands.add(SpawnApple {
        position: Position::new(15, 15),
    })
}

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NewFoodEvent>()
            .add_systems(Update, food_event_listener);
    }
}

#[derive(Event)]
pub struct NewFoodEvent;

#[derive(Component)]
pub struct Food;

pub fn food_event_listener(
    mut commands: Commands,
    board: Res<Board>,
    mut events: EventReader<NewFoodEvent>,
    positions: Query<&Position>,
) {
    let num_food = events.read().count();

    let mut rng = rand::thread_rng();
    for pos in board
        .tiles()
        .filter(|tile| {
            !positions.iter().any(|pos| pos == tile)
        })
        .choose_multiple(&mut rng, num_food)
    {
        commands.add(SpawnApple { position: pos });
    }
}

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
