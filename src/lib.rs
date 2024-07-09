use assets::{AudioAssets, FontAssets};
use bevy::prelude::*;
use board::{position::Position, Board, SpawnSnakeSegment};
use controls::Direction::*;
use food::{Food, NewFoodEvent};
use snake::Snake;

pub mod assets;
pub mod board;
pub mod colors;
pub mod controls;
pub mod food;
pub mod snake;

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
    board: Res<Board>,
    sounds: Res<AudioAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
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
        commands.spawn(AudioBundle {
            source: sounds.gameover.clone(),
            ..default()
        });
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
            commands.spawn(AudioBundle {
                source: sounds.apple.clone(),
                ..default()
            });
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
}

pub fn spawn_menu(
    mut commands: Commands,
    fonts: Res<FontAssets>,
) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(65.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "New Game",
                TextStyle {
                    font: fonts.outfit.clone(),
                    font_size: 40.0,
                    color: Color::rgb(0.1, 0.1, 0.1),
                },
            ));
        });
}

const NORMAL_BUTTON: Color = Color::rgb(0.95, 0.95, 0.95);
const HOVERED_BUTTON: Color = Color::rgb(0.85, 0.85, 0.85);
const PRESSED_BUTTON: Color = Color::rgb(0.75, 0.75, 0.75);

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
