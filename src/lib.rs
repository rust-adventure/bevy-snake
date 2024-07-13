use assets::{AudioAssets, FontAssets};
use bevy::prelude::*;
use bevy_ecs_tilemap::{
    map::TilemapSize,
    tiles::{TilePos, TileStorage},
};
use board::SnakeLayer;
use controls::Direction::*;
use food::{Food, NewFoodEvent};
use itertools::Itertools;
use snake::{Snake, SnakeSegment, SpawnSnakeSegmentEvent};

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
    Startup,
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
    positions: Query<
        &TilePos,
        (Without<Food>, With<SnakeSegment>),
    >,
    input: Res<controls::Direction>,
    query_food: Query<
        (Entity, &TilePos),
        (With<Food>, Without<SnakeSegment>),
    >,
    sounds: Res<AudioAssets>,
    mut next_state: ResMut<NextState<GameState>>,
    mut tilemap: Query<
        (&TilemapSize, &mut TileStorage),
        With<SnakeLayer>,
    >,
) {
    let Ok((tilemap_size, mut tile_storage)) =
        tilemap.get_single_mut()
    else {
        error!(
            "expected a tilemap with TilemapSize and TileStorage to exist"
        );
        return;
    };

    let next_position = *positions.get(snake.segments[0])
    .expect("expect stored entities in a snake to have Position components associated with them");

    let mut next_position = IVec2::new(
        next_position.x as i32,
        next_position.y as i32,
    );

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

    let hit_wall = (0..(tilemap_size.x as i32))
        .cartesian_product(0..(tilemap_size.y as i32))
        .all(|pos| {
            pos != (next_position.x, next_position.y)
        })
        .then_some(GameOverReason::HitWall);

    let next_position = TilePos {
        x: next_position.x as u32,
        y: next_position.y as u32,
    };
    // did the snake hit itself?
    let hit_self = positions
        .iter()
        .find(|pos| pos == &&next_position)
        .map(|_| GameOverReason::HitSnake);

    let has_won = (snake.segments.len()
        == (tilemap_size.x * tilemap_size.y) as usize)
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

    commands.trigger({
        SpawnSnakeSegmentEvent {
            position: next_position,
        }
    });

    // remove old snake segment, unless snake just
    // ate food
    let is_food = query_food
        .iter()
        .find(|(_, pos)| &&next_position == pos);
    match is_food {
        Some((food_entity, tile_pos)) => {
            commands
                .entity(food_entity)
                .despawn_recursive();
            tile_storage.remove(tile_pos);
            commands.trigger(NewFoodEvent);
            commands.spawn(AudioBundle {
                source: sounds.apple.clone(),
                ..default()
            });
        }
        None => {
            let old_tail =
                snake.segments.pop_back().unwrap();
            commands.entity(old_tail).despawn_recursive();
            let old_tail_tile_pos =
                positions.get(old_tail).unwrap();
            tile_storage.remove(old_tail_tile_pos);
        }
    }
}

pub fn reset_game(
    mut commands: Commands,
    mut snake: ResMut<Snake>,
    positions: Query<
        Entity,
        (
            With<TilePos>,
            Or<(With<SnakeSegment>, With<Food>)>,
        ),
    >,
    mut last_pressed: ResMut<controls::Direction>,
    mut tile_storage: Query<
        (&TilemapSize, &mut TileStorage),
        With<SnakeLayer>,
    >,
) {
    let Ok((tilemap_size, mut tile_storage)) =
        tile_storage.get_single_mut()
    else {
        error!(
            "expected a tilemap with TileStorage to exist"
        );
        return;
    };

    for entity in &positions {
        commands.entity(entity).despawn_recursive();
    }

    commands.trigger({
        SpawnSnakeSegmentEvent {
            position: TilePos::new(3, 4),
        }
    });
    commands.trigger({
        SpawnSnakeSegmentEvent {
            position: TilePos::new(4, 4),
        }
    });

    commands.trigger(NewFoodEvent);
    *snake = Default::default();
    *last_pressed = Default::default();
    *tile_storage = TileStorage::empty(*tilemap_size);
}

pub fn spawn_menu(
    mut commands: Commands,
    fonts: Res<FontAssets>,
) {
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(65.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            StateScoped(GameState::Menu),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "New Game",
                TextStyle {
                    font: fonts.outfit.clone(),
                    font_size: 40.0,
                    color: Color::srgb(0.1, 0.1, 0.1),
                },
            ));
        });
}

const NORMAL_BUTTON: Color = Color::srgb(0.95, 0.95, 0.95);
const HOVERED_BUTTON: Color = Color::srgb(0.85, 0.85, 0.85);
const PRESSED_BUTTON: Color = Color::srgb(0.75, 0.75, 0.75);

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
