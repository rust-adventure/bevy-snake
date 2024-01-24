pub mod board;
pub mod colors;
pub mod controls;
pub mod food;
pub mod snake;

use bevy::prelude::*;
use board::Position;
use food::{Food, NewFoodEvent};
use snake::{Snake, SpawnSnakeSegment};

#[derive(
    Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States,
)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
}

pub fn tick(
    mut commands: Commands,
    mut snake: ResMut<Snake>,
    positions: Query<&Position>,
    input: Res<controls::Direction>,
    query_food: Query<(Entity, &Position), With<Food>>,
    mut food_events: EventWriter<NewFoodEvent>,
) {
    let snake_head_entity = snake
        .segments
        .front()
        .expect("snake should have a head entity");

    let next_position = positions.get(*snake_head_entity)
        .map(|head| {
            let diff = match *input {
                controls::Direction::Up => Position::new(0,1),
                controls::Direction::Down => Position::new(0,-1),
                controls::Direction::Left => Position::new(-1,0),
                controls::Direction::Right => Position::new(1,0),
            };
            *head + diff
        })
        .expect("stored entities in a snake should have a Position component associated with them");

    commands.add(SpawnSnakeSegment {
        position: next_position,
    });

    let is_food = query_food
        .iter()
        .find(|(_, food_pos)| &&next_position == food_pos);
    match is_food {
        Some((food_entity, _)) => {
            commands
                .entity(food_entity)
                .despawn_recursive();
            food_events.send(NewFoodEvent);
        }
        None => {
            let old_tail = snake
                .segments
                .pop_back()
                .expect("snake should have a tail entity");
            commands.entity(old_tail).despawn_recursive();
        }
    }
}
