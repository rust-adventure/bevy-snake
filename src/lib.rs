pub mod board;
pub mod colors;
pub mod controls;
pub mod snake;

use bevy::prelude::*;
use board::Position;
use snake::{Snake, SpawnSnakeSegment};

pub fn tick(
    mut commands: Commands,
    mut snake: ResMut<Snake>,
    positions: Query<&Position>,
    input: Res<controls::Direction>,
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

    let old_tail = snake
        .segments
        .pop_back()
        .expect("snake should have a tail entity");
    commands.entity(old_tail).despawn_recursive();
}
