use bevy::prelude::*;
use itertools::Itertools;
use rand::prelude::SliceRandom;

use crate::{
    board::{Board, Position, SpawnApple},
    snake::SnakeBody,
};

pub struct NewFoodEvent;

#[derive(Component)]
pub struct Food;

pub fn food_event_listener(
    mut commands: Commands,
    query_board: Query<&Board>,
    mut events: EventReader<NewFoodEvent>,
    snake: Res<SnakeBody>,
) {
    let board = query_board.single();
    let mut rng = rand::thread_rng();

    let possible_food_locations = (0..board.size)
        .cartesian_product(0..board.size)
        .map(|point| Position {
            x: point.0,
            y: point.1,
        })
        .filter(|pos| !snake.segments.contains(pos))
        .collect::<Vec<Position>>();

    let mut num_food = 0;
    for _ in events.iter() {
        num_food += 1;
    }
    for pos in possible_food_locations
        .choose_multiple(&mut rng, num_food)
    {
        commands.add(SpawnApple { position: *pos });
    }
}
