use bevy::prelude::*;
use board::{Board, Position, SpawnSnakeSegment};
use controls::Direction::*;
use food::{Food, NewFoodEvent};
use iyes_loopless::state::NextState;
use snake::Snake;

pub mod board;
pub mod colors;
pub mod controls;
pub mod food;
pub mod snake;
pub mod ui;

pub const STARTING_GAME_STATE: GameState = GameState::Menu;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Menu,
    Playing,
}

#[derive(PartialEq, Eq, Debug)]
enum GameOverReason {
    HitWall,
    HitSnake,
    Win,
}

pub fn tick(
    mut commands: Commands,
    mut snake: ResMut<Snake>,
    positions: Query<(Entity, &Position)>,
    input: Res<controls::Direction>,
    query_food: Query<(Entity, &Position), With<Food>>,
    mut food_events: EventWriter<NewFoodEvent>,
    query_board: Query<&Board>,
) {
    let board = query_board.single();

    let mut next_position = snake.segments[0].clone();
    let hit_wall = match *input {
        Up if next_position.y == board.size - 1 => {
            Some(GameOverReason::HitWall)
        }
        Up => {
            next_position.y += 1;
            None
        }
        Down if next_position.y == 0 => {
            Some(GameOverReason::HitWall)
        }
        Down => {
            next_position.y -= 1;
            None
        }
        Right if next_position.x == board.size - 1 => {
            Some(GameOverReason::HitWall)
        }
        Right => {
            next_position.x += 1;
            None
        }
        Left if next_position.x == 0 => {
            Some(GameOverReason::HitWall)
        }
        Left => {
            next_position.x -= 1;
            None
        }
    };

    let hit_self =
        if snake.segments.contains(&next_position) {
            Some(GameOverReason::HitSnake)
        } else {
            None
        };

    let has_won = if snake.segments.len()
        == board.size as usize * board.size as usize
    {
        Some(GameOverReason::Win)
    } else {
        None
    };
    match hit_wall.or(hit_self).or(has_won) {
        Some(GameOverReason::HitWall)
        | Some(GameOverReason::HitSnake)
        | Some(GameOverReason::Win) => {
            commands.insert_resource(NextState(
                GameState::Menu,
            ));
        }
        None => {
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
                    commands
                        .entity(entity)
                        .despawn_recursive();
                    food_events.send(NewFoodEvent);
                }
                None => {
                    let old_tail =
                        snake.segments.pop_back().unwrap();
                    if let Some((entity, _)) = positions
                        .iter()
                        .find(|(_, pos)| pos == &&old_tail)
                    {
                        commands
                            .entity(entity)
                            .despawn_recursive();
                    }
                }
            }
        }
    }
}
