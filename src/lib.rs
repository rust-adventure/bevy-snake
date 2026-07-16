use bevy::{
    color::palettes::tailwind::*,
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::{Activate, Button},
};
use itertools::Itertools;

use apple::{Apple, NewApple};
use controls::Direction;
use snake::{BodyFor, HeadOf, SnakeSegment};

pub mod apple;
pub mod controls;
pub mod snake;

const BOARD_SIZE: UVec2 = UVec2::splat(20);
const TILE_SIZE: f32 = 136.;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Startup,
    Menu,
    Playing,
}

pub fn tick(
    mut commands: Commands,
    snake_heads: Single<(Entity, &SnakeSegment), (With<HeadOf>, Without<BodyFor>)>,
    snake_tails: Single<Entity, (With<BodyFor>, Without<HeadOf>)>,
    snake_segments: Query<&SnakeSegment>,
    apples: Query<(Entity, &Apple)>,
    input: Res<Direction>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if snake_segments.iter().count() == BOARD_SIZE.element_product() as usize {
        // win condition!
        next_state.set(GameState::Menu);
    }

    let (entity, segment) = snake_heads.into_inner();
    let next_pos = (segment.position.as_ivec2() + input.as_ivec2()).as_uvec2();

    // If the snake is going to hit a wall on the next tick
    if !(0..BOARD_SIZE.x).contains(&next_pos.x) || !(0..BOARD_SIZE.y).contains(&next_pos.y) {
        next_state.set(GameState::Menu);
        return;
    };

    // If the snake is going to hit itself on the next tick
    if snake_segments
        .iter()
        .any(|segment| segment.position == next_pos)
    {
        next_state.set(GameState::Menu);
        return;
    }

    commands.spawn_scene(bsn! {
        @SnakeSegment {
            @x: {next_pos.x},
            @y: {next_pos.y}
        }
        HeadOf {
            body: entity
        }
    });

    // if the snake is going to hit an apple on the next tick, remove the apple
    if let Some((apple, _)) = apples.iter().find(|(_, apple)| apple.position == next_pos) {
        commands.entity(apple).despawn();
        commands.trigger(NewApple);
    } else {
        // otherwise remove the tail, moving the snake forward
        let tail = snake_tails.into_inner();
        commands.entity(tail).remove::<BodyFor>().despawn();
    };
}

pub fn reset_game(
    mut commands: Commands,
    snake_tail: Query<Entity, (With<BodyFor>, Without<HeadOf>)>,
    mut input: ResMut<Direction>,
    apples: Query<Entity, With<Apple>>,
) {
    *input = Direction::default();
    for tail in &snake_tail {
        commands.entity(tail).despawn();
    }
    for apple in &apples {
        commands.entity(apple).despawn();
    }
}

pub fn new_game_scene() -> impl SceneList {
    bsn_list! [
        #Tail
        @SnakeSegment {
            @x: 1,
            @y: 2
        }
        BodyFor [
            @SnakeSegment {
                @x: 2,
                @y: 2
            }
        ],

        @Apple {
            @x: 8,
            @y: 8,
        }
    ]
}

pub fn menu_scene() -> impl Scene {
    bsn! {
        Button
        Hovered::default()
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(65.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        BackgroundColor(SLATE_600)
        DespawnOnExit::<GameState>(GameState::Menu)
        Children [
            Text::new("New Game")
            TextFont {
                font_size: FontSize::Px(30.0)
            },
            TextColor(Color::from(SLATE_950))
        ]
        on(on_new_game)
    }
}

fn on_new_game(_activate: On<Activate>, mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Playing);
}

pub fn board_scene() -> impl SceneList {
    let tiles: Vec<_> = (0..BOARD_SIZE.y)
        .cartesian_product(0..BOARD_SIZE.x)
        .map(|(y, x)| UVec2::new(x, y))
        .map(|tile_pos| {
            let color = if tile_pos.element_sum().is_multiple_of(2) {
                LIME_600
            } else {
                LIME_700
            };
            let world_pos = tile_to_world(tile_pos);
            bsn! {
                Sprite {
                    color,
                    custom_size: Vec2::splat(TILE_SIZE)
                }
                Transform {
                    translation: {world_pos.extend(0.)}
                }
            }
        })
        .collect();

    bsn_list![{ tiles }]
}

pub fn tile_to_world(tile: UVec2) -> Vec2 {
    let centered_position = tile.as_ivec2() - BOARD_SIZE.as_ivec2() / 2;
    centered_position.as_vec2() * TILE_SIZE
}
