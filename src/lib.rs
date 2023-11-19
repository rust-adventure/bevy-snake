pub mod board;
pub mod colors;
pub mod snake;

use bevy::prelude::*;
use board::Position;
use snake::Snake;

pub fn tick(
    mut commands: Commands,
    mut snake: ResMut<Snake>,
    positions: Query<&Position>,
) {
    let snake_head_entity = snake
        .segments
        .front()
        .expect("snake should have a head entity");

    let next_position = positions.get(*snake_head_entity)
        .map(|head| {
            Position::new(head.x + 1, head.y)
        })
        .expect("stored entities in a snake should have a Position component associated with them");

    // TODO: Spawn sprite in here
    let entity = commands.spawn(next_position).id();

    snake.segments.push_front(entity);

    let old_tail = snake
        .segments
        .pop_back()
        .expect("snake should have a tail entity");
    commands.entity(old_tail).despawn_recursive();

    info!(?snake);
}
