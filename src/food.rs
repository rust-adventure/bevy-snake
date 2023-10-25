use bevy::prelude::*;
use itertools::Itertools;
use rand::prelude::SliceRandom;

use crate::{
    board::{Board, Position, SpawnApple},
    snake::Snake,
    GameState,
};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NewFoodEvent>().add_systems(
            Update,
            food_event_listener
                .run_if(in_state(GameState::Playing)),
        );
    }
}
#[derive(Event)]
pub struct NewFoodEvent;

#[derive(Component)]
pub struct Food;

pub fn food_event_listener(
    mut commands: Commands,
    query_board: Query<&Board>,
    mut events: EventReader<NewFoodEvent>,
    snake: Res<Snake>,
) {
    let board = query_board.single();

    let possible_food_locations = (0..board.size)
        .cartesian_product(0..board.size)
        .map(|point| Position {
            x: point.0,
            y: point.1,
        })
        .filter(|pos| !snake.segments.contains(pos))
        .collect::<Vec<Position>>();

    let num_food = events.iter().count();

    let mut rng = rand::thread_rng();
    for pos in possible_food_locations
        .choose_multiple(&mut rng, num_food)
    {
        commands.add(SpawnApple { position: *pos });
    }
}
