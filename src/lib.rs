use bevy::{color::palettes::tailwind::*, prelude::*};
use bevy_ecs_tilemap::{
    map::TilemapSize,
    tiles::{TilePos, TileStorage},
};
use board::SnakeLayer;
use controls::Direction::*;
use food::{Food, NewFoodEvent};
use itertools::Itertools;
use snake::{SegmentOf, Snake, SpawnSnakeSegmentEvent};

pub mod assets;
pub mod board;
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
    snake: Single<&Snake>,
    positions: Query<
        &TilePos,
        (Without<Food>, With<SegmentOf>),
    >,
    input: Res<controls::Direction>,
    query_food: Query<
        (Entity, &TilePos),
        (With<Food>, Without<SegmentOf>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    tilemap: Single<
        (&TilemapSize, &mut TileStorage),
        With<SnakeLayer>,
    >,
) -> Result {
    let (tilemap_size, mut tile_storage) =
        tilemap.into_inner();

    let next_position =
        positions.get(snake.iter().last().ok_or(
            "snake should have at least one segment",
        )?)?;

    let mut next_position = IVec2::new(
        next_position.x as i32,
        next_position.y as i32,
    );

    next_position += match *input {
        Up => IVec2::Y,
        Down => IVec2::NEG_Y,
        Right => IVec2::X,
        Left => IVec2::NEG_X,
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

    let has_won = (snake.iter().count()
        == (tilemap_size.x * tilemap_size.y) as usize)
        .then_some(GameOverReason::Win);

    // if the game is over, stop processing and go to
    // main menu
    if hit_wall.or(hit_self).or(has_won).is_some() {
        next_state.set(GameState::Menu);
        return Ok(());
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
            commands.entity(food_entity).despawn();
            tile_storage.remove(tile_pos);
            commands.trigger(NewFoodEvent);
        }
        None => {
            let old_tail = snake
                .iter()
                .next()
                .ok_or("expect a snake to have a tail")?;
            commands.entity(old_tail).despawn();
            let old_tail_tile_pos =
                positions.get(old_tail)?;
            tile_storage.remove(old_tail_tile_pos);
        }
    }
    Ok(())
}

pub fn reset_game(
    mut commands: Commands,
    snake: Option<Single<Entity, With<Snake>>>,
    food: Query<Entity, (With<TilePos>, With<Food>)>,
    mut last_pressed: ResMut<controls::Direction>,
    tile_storage: Single<
        (&TilemapSize, &mut TileStorage),
        With<SnakeLayer>,
    >,
) {
    let (tilemap_size, mut tile_storage) =
        tile_storage.into_inner();

    for entity in &food {
        commands.entity(entity).despawn();
    }

    if let Some(snake) = snake {
        commands.entity(*snake).despawn();
    }
    commands.spawn(Snake::default());
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
    *last_pressed = default();
    *tile_storage = TileStorage::empty(*tilemap_size);
}

pub fn spawn_menu(mut commands: Commands) {
    commands.spawn((
        Button,
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(65.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        StateScoped(GameState::Menu),
        children![(
            Text::new("New Game"),
            TextFont {
                font_size: 30.0,
                ..default()
            },
            TextColor(Color::from(SLATE_950))
        )],
    ));
}

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
                *color = BackgroundColor(
                    Color::WHITE.with_alpha(0.75),
                );
                next_state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(
                    Color::WHITE.with_alpha(0.85),
                );
            }
            Interaction::None => {
                *color = BackgroundColor(
                    Color::WHITE.with_alpha(0.95),
                );
            }
        }
    }
}
