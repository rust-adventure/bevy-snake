use assets::AudioAssets;
use bevy::prelude::*;
use bevy_kira_audio::Audio;
use board::{Board, Position, SpawnSnakeSegment};
use common::{Game, RunState};
use control::LastKeyPress;
use food::{Food, NewFoodEvent};
use snake::SnakeBody;

pub mod assets;
pub mod board;
pub mod colors;
pub mod common;
pub mod control;
pub mod food;
pub mod scoring;
pub mod settings;
pub mod snake;
pub mod ui;

#[derive(PartialEq, Eq, Debug)]
enum GameOverReason {
    HitWall,
    HitSnake,
    Win,
}

pub fn snake_movement(
    mut commands: Commands,
    query_board: Query<&Board>,
    mut snake: ResMut<SnakeBody>,
    positions: Query<(Entity, &Position)>,
    last_pressed: Res<LastKeyPress>,
    query_food: Query<(Entity, &Position), With<Food>>,
    mut food_events: EventWriter<NewFoodEvent>,
    mut run_state: ResMut<State<RunState>>,
    mut game: ResMut<Game>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    sounds: Res<AudioAssets>,
) {
    let board = query_board.single();

    let mut new_segment = snake.segments[0].clone();

    // did the snake hit the wall?
    let hit_wall = match last_pressed.0 {
            KeyCode::Up if new_segment.y == board.size-1 => Some(GameOverReason::HitWall),
            KeyCode::Up => {
                new_segment.y += 1;
                None
            }
            KeyCode::Down if new_segment.y == 0 => Some(GameOverReason::HitWall),
            KeyCode::Down => {
                new_segment.y -= 1;
                None
            }
            KeyCode::Left if new_segment.x == 0 => Some(GameOverReason::HitWall),
            KeyCode::Left => {
                new_segment.x -= 1;
                None
            }
            KeyCode::Right if new_segment.x == board.size-1 => Some(GameOverReason::HitWall),
            KeyCode::Right => {
                new_segment.x += 1;
                None
            }
            _ => panic!(
                "a key should always have been pressed, even if it's just the default input"
            ),
        };
    // did the snake hit itself?
    let hit_self = snake
        .segments
        .iter()
        .find(|position| position == &&new_segment)
        .map(|_| GameOverReason::HitSnake);

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
            // send game over event
            audio.play(sounds.gameover.clone());
            run_state.set(RunState::Menu).unwrap();
        }
        None => {
            snake.segments.push_front(new_segment);

            commands.add(SpawnSnakeSegment {
                position: snake.segments[0],
            });

            // remove old snake segment, unless snake just
            // ate food
            let is_food = query_food
                .iter()
                .find(|(_, pos)| &&new_segment == pos);
            match is_food {
                Some((entity, _)) => {
                    game.score += 1;

                    audio.play(sounds.apple.clone());
                    commands
                        .entity(entity)
                        .despawn_recursive();
                    food_events.send(NewFoodEvent);
                }
                None => {
                    let segment =
                        snake.segments.pop_back().unwrap();

                    let position_to_remove = positions
                        .iter()
                        .find(|(_, position)| {
                            position == &&segment
                        });
                    if let Some(position) =
                        position_to_remove
                    {
                        commands
                            .entity(position.0)
                            .despawn_recursive();
                    }
                }
            }
        }
    };

    if game.score_best < game.score {
        game.score_best = game.score;
    };
}

pub fn reset_game(
    mut commands: Commands,
    mut snake: ResMut<SnakeBody>,
    positions: Query<(
        Entity,
        &Position,
        &TextureAtlasSprite,
    )>,
    mut last_pressed: ResMut<LastKeyPress>,
    food_query: Query<Entity, With<Food>>,
    mut food_events: EventWriter<NewFoodEvent>,
) {
    for entity in food_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for position in positions.iter() {
        commands.entity(position.0).despawn_recursive();
    }

    food_events.send(NewFoodEvent);
    *snake = SnakeBody::default();
    *last_pressed = LastKeyPress(KeyCode::Right);
}
