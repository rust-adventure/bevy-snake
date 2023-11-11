use bevy::prelude::*;
use rand::prelude::SliceRandom;

use crate::{
    board::{position::Position, Board, SpawnApple},
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
    board: Res<Board>,
    mut events: EventReader<NewFoodEvent>,
    positions: Query<&Position>,
) {
    let possible_food_locations = board
        .tiles()
        .filter(|tile| {
            !positions.iter().any(|pos| pos == tile)
        })
        .collect::<Vec<Position>>();

    let num_food = events.read().count();

    let mut rng = rand::thread_rng();
    for pos in possible_food_locations
        .choose_multiple(&mut rng, num_food)
    {
        commands.add(SpawnApple { position: *pos });
    }
}
