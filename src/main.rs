use bevy::prelude::*;
use bevy::{
    core::{FixedTimestep, FixedTimesteps},
    prelude::*,
};
use bevy_easings::*;
use itertools::Itertools;
use rand::prelude::SliceRandom;
use std::collections::VecDeque;

mod colors;
use colors::MATERIALS;

// GameStates and FixedTimesteps can not be used together yes, instead use iyes crate
// https://canary.discord.com/channels/691052431525675048/956767127291965500/956770647911059477
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum RunState {
    Playing,
    GameOver,
}

#[derive(PartialEq, Eq, Debug)]
enum GameOverReason {
    HitWall,
    HitSnake,
    Win,
}

const LABEL: &str = "my_fixed_timestep";

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

#[derive(Debug)]
pub struct SnakeBody {
    segments: VecDeque<Position>,
}

impl Default for SnakeBody {
    fn default() -> Self {
        Self {
            segments: VecDeque::from([Position {
                x: 0,
                y: 0,
            }]),
        }
    }
}

struct LastKeyPress(KeyCode);
impl Default for LastKeyPress {
    fn default() -> Self {
        Self(KeyCode::Right)
    }
}

struct NewFoodEvent;

#[derive(Component)]
struct Food;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EasingsPlugin)
        .add_event::<NewFoodEvent>()
        .insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(
            0.04, 0.04, 0.1,
        )))
        .init_resource::<SnakeBody>()
        .init_resource::<LastKeyPress>()
        .add_startup_system(setup)
        .add_startup_system(spawn_board)
        // .add_startup_system(spawn_snake.after("board"))
        .add_state(RunState::Playing)
        .add_system_set(
            SystemSet::on_update(RunState::Playing)
            .with_system(render_snake)
            .with_system(user_input)
            .with_system(food_event_listener)
        //         .with_system(render_tile_points)
        //         .with_system(board_shift)
        //         .with_system(render_tiles)
        //         .with_system(new_tile_handler)
        //         .with_system(end_game),
        )
        .add_system_set(
            SystemSet::on_enter(RunState::Playing)
                // .with_system(game_reset.system())
                .with_system(spawn_snake)
        )
        .add_stage_after(
            CoreStage::Update,
            FixedUpdateStage,
            SystemStage::parallel()
                // .with_system_set(SystemSet::on_update(RunState::Playing))
                .with_run_criteria(
                    FixedTimestep::step(0.1)
                        // labels are optional. they provide a way to access the current
                        // FixedTimestep state from within a system
                        .with_label(LABEL),
                )
                .with_system(snake_movement),
        )
        .run();
}
const TILE_SIZE: f32 = 20.0;
const TILE_SPACER: f32 = 0.0;

#[derive(
    Debug, PartialEq, Copy, Clone, Eq, Hash, Component,
)]
struct Position {
    x: u8,
    y: u8,
}

#[derive(Component)]
struct Board {
    size: u8,
    physical_size: f32,
}

impl Board {
    fn new(size: u8) -> Self {
        let physical_size = f32::from(size) * TILE_SIZE
            + f32::from(size + 1) * TILE_SPACER;
        Board {
            size,
            physical_size,
        }
    }
    fn cell_position_to_physical(&self, pos: u8) -> f32 {
        let offset =
            -self.physical_size / 2.0 + 0.5 * TILE_SIZE;

        offset
            + f32::from(pos) * TILE_SIZE
            + f32::from(pos + 1) * TILE_SPACER
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d());
}
fn spawn_board(mut commands: Commands) {
    let board = Board::new(20);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: MATERIALS.board,
                custom_size: Some(Vec2::new(
                    board.physical_size,
                    board.physical_size,
                )),
                ..Sprite::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            for tile in (0..board.size)
                .cartesian_product(0..board.size)
            {
                builder.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: if (tile.0 + tile.1) % 2 == 0
                        {
                            MATERIALS.tile_placeholder
                        } else {
                            MATERIALS.tile_placeholder_dark
                        },
                        custom_size: Some(Vec2::new(
                            TILE_SIZE, TILE_SIZE,
                        )),
                        ..Sprite::default()
                    },
                    transform: Transform::from_xyz(
                        board.cell_position_to_physical(
                            tile.0,
                        ),
                        board.cell_position_to_physical(
                            tile.1,
                        ),
                        1.0,
                    ),
                    ..Default::default()
                });
            }
        })
        .insert(board);
}

fn spawn_snake(
    mut commands: Commands,
    query_board: Query<&Board>,
    snake: Res<SnakeBody>,
) {
    let board = query_board.single();
    let start_x = board.size / 2;
    let start_y = board.size / 2;
    let starting_tiles =
        VecDeque::from([(start_x, start_y)]);
    // let snake = Snake {
    //     head: Position { x: 1, y: 0 },
    //     body: VecDeque::from([Position { x: 0, y: 0 }]),
    // };

    let x = (&snake).segments[0].x.clone();
    let y = (&snake).segments[0].y.clone();
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: MATERIALS.tile,
                custom_size: Some(Vec2::new(
                    TILE_SIZE, TILE_SIZE,
                )),
                ..Sprite::default()
            },
            transform: Transform::from_xyz(
                board.cell_position_to_physical(x),
                board.cell_position_to_physical(y),
                2.0,
            ),
            ..Default::default()
        })
        .insert(snake.segments[0]);

    for (x, y) in starting_tiles.iter() {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: MATERIALS.food,
                    custom_size: Some(Vec2::new(
                        TILE_SIZE, TILE_SIZE,
                    )),
                    ..Sprite::default()
                },
                transform: Transform::from_xyz(
                    board.cell_position_to_physical(*x),
                    board.cell_position_to_physical(*y),
                    2.0,
                ),
                ..Default::default()
            })
            .insert(Position { x: *x, y: *y })
            .insert(Food);
    }
}

fn render_snake(
    mut commands: Commands,
    segments: Query<(Entity, &Position)>,
    query_board: Query<&Board>,
    snake: Res<SnakeBody>,
) {
    let board = query_board.single();

    // dbg!(snake.single());
    // for (entity, transform, pos) in tiles.iter() {
    //     let x = board.cell_position_to_physical(pos.x);
    //     let y = board.cell_position_to_physical(pos.y);
    //     commands.entity(entity).insert(transform.ease_to(
    //         Transform::from_xyz(
    //             x,
    //             y,
    //             transform.translation.z,
    //         ),
    //         EaseFunction::QuadraticInOut,
    //         EasingType::Once {
    //             duration: std::time::Duration::from_millis(
    //                 100,
    //             ),
    //         },
    //     ));
    // }
}

fn user_input(
    input: Res<Input<KeyCode>>,
    mut last_pressed: ResMut<LastKeyPress>,
) {
    if input.pressed(KeyCode::Up) {
        last_pressed.0 = KeyCode::Up;
    } else if input.pressed(KeyCode::Down) {
        last_pressed.0 = KeyCode::Down;
    } else if input.pressed(KeyCode::Left) {
        last_pressed.0 = KeyCode::Left;
    } else if input.pressed(KeyCode::Right) {
        last_pressed.0 = KeyCode::Right;
    }
}
fn snake_movement(
    mut commands: Commands,
    query_board: Query<&Board>,
    mut snake: ResMut<SnakeBody>,
    positions: Query<(Entity, &Position)>,
    last_pressed: Res<LastKeyPress>,
    query_food: Query<(Entity, &Position), With<Food>>,
    mut food_events: EventWriter<NewFoodEvent>,
    mut run_state: ResMut<State<RunState>>,
) {
    let board = query_board.single();

    let mut new_segment = snake.segments[0].clone();

    // did the snake hit the wall?
    let hit_wall = match last_pressed.0 {
            KeyCode::Up => {
                if new_segment.y == board.size-1 {
                    Some(GameOverReason::HitWall)
                } else {
                    new_segment.y += 1;
                    None
                }
            }
            KeyCode::Down => {
                if new_segment.y == 0 {
                    Some(GameOverReason::HitWall)
                } else {
                    new_segment.y -= 1;
                    None
                }                
            }
            KeyCode::Left => {
                if new_segment.x == 0 {
                    Some(GameOverReason::HitWall)
                } else {
                    new_segment.x -= 1;
                    None
                }
            }
            KeyCode::Right => {
                if new_segment.x == board.size-1 {
                    Some(GameOverReason::HitWall)
                } else {
                    new_segment.x += 1;
                    None
                }
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

    let has_won = if snake.segments.len() == board.size as usize * board.size as usize {
        Some(GameOverReason::Win)
    } else {
        None
    };

    match hit_wall.or(hit_self).or(has_won) {
        Some(GameOverReason::HitWall)
        | Some(GameOverReason::HitSnake) | Some(GameOverReason::Win) => {
            // send game over event
            run_state.set(RunState::GameOver).unwrap();

        }
        None => {
            snake.segments.push_front(new_segment);

            // add new snake segment to board
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: MATERIALS.tile,
                        custom_size: Some(Vec2::new(
                            TILE_SIZE, TILE_SIZE,
                        )),
                        ..Sprite::default()
                    },
                    transform: Transform::from_xyz(
                        board.cell_position_to_physical(
                            new_segment.x,
                        ),
                        board.cell_position_to_physical(
                            new_segment.y,
                        ),
                        2.0,
                    ),
                    ..Default::default()
                })
                .insert(snake.segments[0]);

            // remove old snake segment, unless snake just ate food
            let is_food = query_food
                .iter()
                .find(|(_, pos)| &&new_segment == pos);
            match is_food {
                Some((entity, pos)) => {
                    //TODO: add points for food consumption
                    commands
                        .entity(entity)
                        .despawn_recursive();
                    dbg!("send food event");
                    food_events.send(NewFoodEvent);
                }
                None => {
                    let segment =
                        snake.segments.pop_back().unwrap();

                    let position_to_remove = positions
                        .iter()
                        .find(|(_, position)| {
                            position == &&segment
                        })
                        .unwrap();
                    commands
                        .entity(position_to_remove.0)
                        .despawn_recursive();
                }
            }
        }
    }
}

fn food_event_listener(
    mut commands: Commands,
    query_board: Query<&Board>,
    mut events: EventReader<NewFoodEvent>,
    snake: Res<SnakeBody>,
) {
    let board = query_board.single();
    let mut rng = rand::thread_rng();

    let possible_food_locations = (0..board.size)
        .cartesian_product(0..board.size)
        .map(|point| Position {
            x: point.0,
            y: point.1,
        })
        .filter(|pos| !snake.segments.contains(pos))
        .collect::<Vec<Position>>();

    let mut num_food = 0;
    for _ in events.iter() {
        num_food += 1;
    }
    for pos in possible_food_locations
        .choose_multiple(&mut rng, num_food)
    {
        dbg!(pos);
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: MATERIALS.food,
                    custom_size: Some(Vec2::new(
                        TILE_SIZE, TILE_SIZE,
                    )),
                    ..Sprite::default()
                },
                transform: Transform::from_xyz(
                    board.cell_position_to_physical(pos.x),
                    board.cell_position_to_physical(pos.y),
                    2.0,
                ),
                ..Default::default()
            })
            .insert(*pos)
            .insert(Food);
    }
}
