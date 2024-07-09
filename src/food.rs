use bevy::prelude::*;
use rand::prelude::SliceRandom;

use crate::board::{position::Position, Board, SpawnApple};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NewFoodEvent>()
            .observe(food_event_listener);
    }
}
#[derive(Event)]
pub struct NewFoodEvent;

#[derive(Component)]
pub struct Food;

pub fn food_event_listener(
    _trigger: Trigger<NewFoodEvent>,
    mut commands: Commands,
    board: Res<Board>,
    positions: Query<&Position>,
) {
    let possible_food_locations = board
        .tiles()
        .filter(|tile| {
            !positions.iter().any(|pos| pos == tile)
        })
        .collect::<Vec<Position>>();

    let mut rng = rand::thread_rng();
    if let Some(pos) =
        possible_food_locations.choose(&mut rng)
    {
        commands.add(SpawnApple { position: *pos });
    } else {
        error!("can't find valid apple spawning space");
    }
}
