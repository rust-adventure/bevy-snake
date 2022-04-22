use bevy::prelude::*;
use board::{Position, SpawnSnakeSegment};
use food::Food;
use snake::Snake;

pub mod board;
pub mod colors;
pub mod controls;
pub mod food;
pub mod snake;

pub fn tick(
    mut commands: Commands,
    mut snake: ResMut<Snake>,
    positions: Query<(Entity, &Position)>,
    input: Res<controls::Direction>,
    query_food: Query<(Entity, &Position), With<Food>>,
) {
    let mut next_position = snake.segments[0].clone();
    match *input {
        controls::Direction::Up => {
            next_position.y += 1;
        }
        controls::Direction::Down => {
            next_position.y -= 1;
        }
        controls::Direction::Right => {
            next_position.x += 1;
        }
        controls::Direction::Left => {
            next_position.x -= 1;
        }
    };

    snake.segments.push_front(next_position);
    commands.add({
        SpawnSnakeSegment {
            position: next_position,
        }
    });

    // remove old snake segment, unless snake just
    // ate food
    let is_food = query_food
        .iter()
        .find(|(_, pos)| &&next_position == pos);
    match is_food {
        Some((entity, _)) => {
            commands.entity(entity).despawn_recursive();
            // food_events.send(NewFoodEvent);
        }
        None => {
            let old_tail =
                snake.segments.pop_back().unwrap();
            if let Some((entity, _)) = positions
                .iter()
                .find(|(_, pos)| pos == &&old_tail)
            {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
