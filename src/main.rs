use bevy::prelude::*;
use bevy_easings::*;
use itertools::Itertools;
use iyes_loopless::prelude::*;
use rand::prelude::SliceRandom;
use std::{collections::{VecDeque, HashMap}, time::Duration};
use bevy_kira_audio::{Audio, AudioPlugin, AudioChannel};

mod colors;
use colors::MATERIALS;
mod ui;
use ui::*;
mod common;
use common::*;
mod board;
use board::*;

#[derive(PartialEq, Eq, Debug)]
enum GameOverReason {
    HitWall,
    HitSnake,
    Win,
}

#[derive(Debug)]
pub struct SnakeBody {
    segments: VecDeque<Position>,
}

impl Default for SnakeBody {
    fn default() -> Self {
        Self {
            segments: VecDeque::from([
                Position { x: 4, y: 4 },
                Position { x: 3, y: 4 },
            ]),
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
        .add_plugin(GameUiPlugin)
        .add_plugin(AudioPlugin)
        .add_event::<NewFoodEvent>()
        .insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(
            0.52, 0.73, 0.17,
        )))
        .init_resource::<Game>()
        .init_resource::<SnakeBody>()
        .init_resource::<LastKeyPress>()
        .init_resource::<FontSpec>()
        .add_startup_system(setup)
        .add_startup_system(spawn_board)
        // .add_startup_system(load_audio)
        .add_state(RunState::Playing)
        .add_system_set(
            SystemSet::on_update(RunState::Playing)
                .with_system(user_input)
                .with_system(food_event_listener)
                .with_system(snake_segments),
        )
        .add_system_set(
            SystemSet::on_enter(RunState::Playing)
                // .with_system(game_reset.system())
                .with_system(spawn_snake),
        )
        .add_stage_before(
            CoreStage::Update,
            "snake_tick",
            FixedTimestepStage::new(Duration::from_millis(
                100,
            ))
            .with_stage(
                SystemStage::parallel().with_system(
                    snake_movement.run_in_bevy_state(
                        RunState::Playing,
                    ),
                ),
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_snake(
    mut commands: Commands,
    query_board: Query<&Board>,
    snake: Res<SnakeBody>,
) {
    let board = query_board.single();

    for position in snake.segments.iter() {
        commands.add(SpawnSnakeSegment {
            position: *position,
        });
    }

    commands.add(SpawnApple {
        position: Position {
            x: board.size / 2,
            y: board.size / 2,
        },
    });
}

fn user_input(
    input: Res<Input<KeyCode>>,
    snake: Res<SnakeBody>,
    mut last_pressed: ResMut<LastKeyPress>,
) {
    let head = snake.segments[0];
    let neck = snake.segments.get(1);

    if input.pressed(KeyCode::Up) {
        match (head, neck) {
            (h, Some(n)) if (h.y + 1) == n.y => {}
            _ => {
                last_pressed.0 = KeyCode::Up;
            }
        }
    } else if input.pressed(KeyCode::Down) {
        match (head, neck) {
            (h, Some(n))
                if h.y != 0 && (h.y - 1) == n.y => {}
            _ => {
                last_pressed.0 = KeyCode::Down;
            }
        }
    } else if input.pressed(KeyCode::Left) {
        match (head, neck) {
            (h, Some(n))
                if h.x != 0 && (h.x - 1) == n.x => {}
            _ => {
                last_pressed.0 = KeyCode::Left;
            }
        }
    } else if input.pressed(KeyCode::Right) {
        match (head, neck) {
            (h, Some(n)) if (h.x + 1) == n.x => {}
            _ => {
                last_pressed.0 = KeyCode::Right;
            }
        }
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
    mut game: ResMut<Game>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>
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
            audio.play(asset_server.load("gameover.ogg"));
            run_state.set(RunState::GameOver).unwrap();
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

                    audio.play(asset_server.load("apple.ogg"));
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
                        })
                        .unwrap();
                    commands
                        .entity(position_to_remove.0)
                        .despawn_recursive();
                }
            }
        }
    };

    if game.score_best < game.score {
        game.score_best = game.score;
    };
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
        // dbg!(pos);
        commands.add(SpawnApple { position: *pos });
    }
}

#[tracing::instrument(skip(positions,))]
fn snake_segments(
    snake: Res<SnakeBody>,
    mut positions: Query<(
        &Position,
        &mut TextureAtlasSprite,
        &mut Transform,
    )>,
) {
    if snake.segments.len() > 1 {
        let current_position = positions
            .iter_mut()
            .find(|pos| pos.0 == &snake.segments[0]);

        match current_position {
            Some((pos, mut sprite, mut transform)) => {
                let rotation = match detect_side(
                    pos,
                    &snake.segments[1],
                ) {
                    Direction::Up => {
                        Quat::from_rotation_z(0.0)
                    }
                    Direction::Down => {
                        Quat::from_rotation_z(
                            std::f32::consts::PI,
                        )
                    }
                    Direction::Left => {
                        Quat::from_rotation_z(
                            std::f32::consts::FRAC_PI_2,
                        )
                    }
                    Direction::Right => {
                        Quat::from_rotation_z(
                            -std::f32::consts::FRAC_PI_2,
                        )
                    }
                };
                sprite.index = 116;
                transform.rotation = rotation;
            }
            None => {}
        }
    }

    if snake.segments.len() > 1 {
        let current_position =
            positions.iter_mut().find(|pos| {
                pos.0
                    == &snake.segments
                        [snake.segments.len() - 1]
            });

        match current_position {
            Some((pos, mut sprite, mut transform)) => {
                let rotation = match detect_side(
                    pos,
                    &snake.segments
                        [snake.segments.len() - 2],
                ) {
                    Direction::Up => {
                        Quat::from_rotation_z(0.0)
                    }
                    Direction::Down => {
                        Quat::from_rotation_z(
                            std::f32::consts::PI,
                        )
                    }
                    Direction::Left => {
                        Quat::from_rotation_z(
                            std::f32::consts::FRAC_PI_2,
                        )
                    }
                    Direction::Right => {
                        Quat::from_rotation_z(
                            -std::f32::consts::FRAC_PI_2,
                        )
                    }
                };
                sprite.index = 119;
                transform.rotation = rotation;
            }
            None => {}
        }
    }

    for (front, origin, back) in
        snake.segments.iter().tuple_windows()
    {
        let a = detect_side(origin, front);
        let b = detect_side(origin, back);

        let image = match (a, b) {
            // vertical
            (Direction::Down, Direction::Up) => {
                (117, Quat::from_rotation_z(0.0))
            }
            (Direction::Up, Direction::Down) => {
                (117, Quat::from_rotation_z(0.0))
            }
            // horizontal
            (Direction::Right, Direction::Left) => (
                117,
                Quat::from_rotation_z(
                    std::f32::consts::FRAC_PI_2,
                ),
            ),
            (Direction::Left, Direction::Right) => (
                117,
                Quat::from_rotation_z(
                    std::f32::consts::FRAC_PI_2,
                ),
            ),
            // ⌞
            (Direction::Up, Direction::Right) => (
                118,
                Quat::from_rotation_z(
                    std::f32::consts::FRAC_PI_2,
                ),
            ),
            (Direction::Right, Direction::Up) => (
                118,
                Quat::from_rotation_z(
                    std::f32::consts::FRAC_PI_2,
                ),
            ),
            // ⌜
            (Direction::Right, Direction::Down) => {
                (118, Quat::from_rotation_z(0.0))
            }
            (Direction::Down, Direction::Right) => {
                (118, Quat::from_rotation_z(0.0))
            }
            // ⌟
            (Direction::Left, Direction::Up) => (
                118,
                Quat::from_rotation_z(std::f32::consts::PI),
            ),
            (Direction::Up, Direction::Left) => (
                118,
                Quat::from_rotation_z(std::f32::consts::PI),
            ),
            // ⌝
            (Direction::Left, Direction::Down) => (
                118,
                Quat::from_rotation_z(
                    -std::f32::consts::FRAC_PI_2,
                ),
            ),
            (Direction::Down, Direction::Left) => (
                118,
                Quat::from_rotation_z(
                    -std::f32::consts::FRAC_PI_2,
                ),
            ),
            _ => panic!("unhandled"),
        };
        let current_position = positions
            .iter_mut()
            .find(|pos| pos.0 == origin);

        match current_position {
            Some((_, mut sprite, mut transform)) => {
                sprite.index = image.0;
                transform.rotation = image.1;
            }
            None => {}
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[tracing::instrument]
fn detect_side(
    origin: &Position,
    other: &Position,
) -> Direction {
    // dbg!(origin, other);
    if other.y > origin.y {
        Direction::Up
    } else if other.y < origin.y {
        Direction::Down
    } else if other.x > origin.x {
        Direction::Right
    } else if other.x < origin.x {
        Direction::Left
    } else {
        info!(?origin, ?other);
        panic!("should never happen");
    }
}
