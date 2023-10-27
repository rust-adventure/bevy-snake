use assets::AudioAssets;
use bevy::prelude::*;
use board::{position::Position, Board, SpawnSnakeSegment};
use controls::Direction::*;
use food::{Food, NewFoodEvent};
use scoring::Score;
use settings::{AudioSettings, GameSettings};
use snake::Snake;

pub mod assets;
pub mod board;
pub mod colors;
pub mod controls;
pub mod food;
pub mod scoring;
pub mod settings;
pub mod snake;
pub mod ui;

#[derive(
    Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States,
)]
pub enum GameState {
    #[default]
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
    positions: Query<(Entity, &Position), Without<Food>>,
    input: Res<controls::Direction>,
    query_food: Query<(Entity, &Position), With<Food>>,
    mut food_events: EventWriter<NewFoodEvent>,
    query_board: Query<&Board>,
    sounds: Res<AudioAssets>,
    settings: Res<GameSettings>,
    mut score: ResMut<Score>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let board = query_board.single();

    let mut next_position = *positions.get(snake.segments[0])
    .expect("expect stored entities in a snake to have Position components associated with them").1;

    match *input {
        Up => {
            next_position.y += 1;
        }
        Down => {
            next_position.y -= 1;
        }
        Right => {
            next_position.x += 1;
        }
        Left => {
            next_position.x -= 1;
        }
    };

    let hit_wall = board
        .tiles()
        .all(|pos| pos != next_position)
        .then_some(GameOverReason::HitWall);

    // did the snake hit itself?
    let hit_self = positions
        .iter()
        .find(|(_, pos)| pos == &&next_position)
        .map(|_| GameOverReason::HitSnake);

    let has_won = (snake.segments.len()
        == (board.size as usize).pow(2))
    .then_some(GameOverReason::Win);

    // if the game is over, stop processing and go to
    // main menu
    if hit_wall.or(hit_self).or(has_won).is_some() {
        next_state.set(GameState::Menu);
        if settings.audio == AudioSettings::ON {
            commands.spawn(AudioBundle {
                source: sounds.gameover.clone(),
                ..default()
            });
        }
        return;
    }

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
        Some((food_entity, _)) => {
            commands
                .entity(food_entity)
                .despawn_recursive();
            food_events.send(NewFoodEvent);
            score.score += 1;
            if settings.audio == AudioSettings::ON {
                commands.spawn(AudioBundle {
                    source: sounds.apple.clone(),
                    ..default()
                });
            }
        }
        None => {
            let old_tail =
                snake.segments.pop_back().unwrap();
            commands.entity(old_tail).despawn_recursive();
        }
    }
}

pub fn reset_game(
    mut commands: Commands,
    mut snake: ResMut<Snake>,
    positions: Query<Entity, With<Position>>,
    mut last_pressed: ResMut<controls::Direction>,
    mut food_events: EventWriter<NewFoodEvent>,
    mut score: ResMut<Score>,
) {
    for entity in positions.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.add({
        SpawnSnakeSegment {
            position: Position::new(3, 4),
        }
    });
    commands.add({
        SpawnSnakeSegment {
            position: Position::new(4, 4),
        }
    });

    food_events.send(NewFoodEvent);
    *snake = Default::default();
    *last_pressed = Default::default();
    *score = Default::default();
}
