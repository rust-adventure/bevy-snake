use bevy::prelude::*;
use bevy_easings::*;
use itertools::Itertools;
use iyes_loopless::prelude::*;
use kayak_ui::{
    bevy::BevyKayakUIPlugin,
    core::{bind, Binding, Bound, MutableBound},
};
use rand::{
    distributions::WeightedIndex,
    prelude::{Distribution, SliceRandom},
};

use std::{collections::VecDeque, time::Duration};

mod colors;
use colors::MATERIALS;
mod ui;
use ui::*;

// GameStates and FixedTimesteps can not be used
// together yes, instead use iyes crate https://canary.discord.com/channels/691052431525675048/956767127291965500/956770647911059477
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum RunState {
    Playing,
    GameOver,
    Menu,
}

#[derive(Default, Clone, PartialEq, Eq)]
struct Game {
    score: u32,
    score_best: u32,
}

#[derive(PartialEq, Eq, Debug)]
enum GameOverReason {
    HitWall,
    HitSnake,
    Win,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

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
        .add_plugin(BevyKayakUIPlugin)
        .add_event::<NewFoodEvent>()
        .insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(
            0.52, 0.73, 0.17,
        )))
        // .init_resource::<Game>()
        .insert_resource(bind(Game::default()))
        .init_resource::<SnakeBody>()
        .init_resource::<LastKeyPress>()
        .add_startup_system(setup)
        .add_startup_system(spawn_board)
        .add_state(RunState::Playing)
        .add_startup_system(ui)
        .add_system(snake_segments)
        .add_system_set(
            SystemSet::on_update(RunState::Playing)
                .with_system(render_snake)
                .with_system(user_input)
                .with_system(food_event_listener),
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
const TILE_SIZE: f32 = 30.0;
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
fn spawn_board(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let board = Board::new(20);

    let texture_handle = asset_server.load("grass.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16.0, 16.0),
        3,
        1,
    );
    let texture_atlas_handle =
        texture_atlases.add(texture_atlas);

    let mut rng = rand::thread_rng();
    // let die = Uniform::from(0..3);
    let weights = vec![3, 3, 1];
    let dist = WeightedIndex::new(weights).unwrap();

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
                builder.spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle
                        .clone(),
                    sprite: TextureAtlasSprite {
                        index: dist.sample(&mut rng),
                        custom_size: Some(Vec2::new(
                            TILE_SIZE, TILE_SIZE,
                        )),
                        ..TextureAtlasSprite::default()
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
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let board = query_board.single();
    let start_x = board.size / 2;
    let start_y = board.size / 2;
    let starting_tiles =
        VecDeque::from([(start_x, start_y)]);
    // let snake = Snake {
    //     head: Position { x: 1, y: 0 },
    //     body: VecDeque::from([Position { x: 0, y: 0
    // }]), };

    let texture_handle =
        asset_server.load("snake-sprites.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(136.0, 136.0),
        4,
        30,
    );
    let texture_atlas_handle =
        texture_atlases.add(texture_atlas);

    for position in snake.segments.iter() {
        // add new snake segment to board
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_xyz(
                    board.cell_position_to_physical(
                        position.x,
                    ),
                    board.cell_position_to_physical(
                        position.y,
                    ),
                    2.0,
                ),
                sprite: TextureAtlasSprite {
                    index: 116,
                    custom_size: Some(Vec2::new(
                        TILE_SIZE, TILE_SIZE,
                    )),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(*position);
    }

    for (x, y) in starting_tiles.iter() {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    // color: MATERIALS.food,
                    custom_size: Some(Vec2::new(
                        TILE_SIZE, TILE_SIZE,
                    )),
                    ..Sprite::default()
                },
                texture: asset_server.load("apple.png"),
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
    // for (entity, transform, pos) in
    // tiles.iter() {     let x =
    // board.cell_position_to_physical(pos.x);
    //     let y =
    // board.cell_position_to_physical(pos.y);
    //     commands.entity(entity).
    // insert(transform.ease_to(
    //         Transform::from_xyz(
    //             x,
    //             y,
    //             transform.translation.z,
    //         ),
    //         EaseFunction::QuadraticInOut,
    //         EasingType::Once {
    //             duration:
    // std::time::Duration::from_millis(
    //                 100,
    //             ),
    //         },
    //     ));
    // }
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
            (h, Some(n)) if (h.y - 1) == n.y => {}
            _ => {
                last_pressed.0 = KeyCode::Down;
            }
        }
    } else if input.pressed(KeyCode::Left) {
        match (head, neck) {
            (h, Some(n)) if (h.x - 1) == n.x => {}
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
    mut game: Res<Binding<Game>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
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
            run_state.set(RunState::GameOver).unwrap();
        }
        None => {
            snake.segments.push_front(new_segment);

            let texture_handle =
                asset_server.load("snake-sprites.png");
            let texture_atlas = TextureAtlas::from_grid(
                texture_handle,
                Vec2::new(136.0, 136.0),
                4,
                30,
            );
            let texture_atlas_handle =
                texture_atlases.add(texture_atlas);
            // add new snake segment to board
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    transform: Transform::from_xyz(
                        board.cell_position_to_physical(
                            new_segment.x,
                        ),
                        board.cell_position_to_physical(
                            new_segment.y,
                        ),
                        2.0,
                    ),
                    sprite: TextureAtlasSprite {
                        index: 116,
                        custom_size: Some(Vec2::new(
                            TILE_SIZE, TILE_SIZE,
                        )),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                // commands
                //     .spawn_bundle(SpriteBundle {
                //         sprite: Sprite {
                //             color: MATERIALS.tile,
                //             custom_size: Some(Vec2::new(
                //                 TILE_SIZE, TILE_SIZE,
                //             )),
                //             ..Sprite::default()
                //         },
                //         transform: Transform::from_xyz(
                //
                // board.cell_position_to_physical(
                //                 new_segment.x,
                //             ),
                //
                // board.cell_position_to_physical(
                //                 new_segment.y,
                //             ),
                //             2.0,
                //         ),
                //         ..Default::default()
                //     })
                .insert(snake.segments[0]);

            // remove old snake segment, unless snake just
            // ate food
            let is_food = query_food
                .iter()
                .find(|(_, pos)| &&new_segment == pos);
            match is_food {
                Some((entity, pos)) => {
                    game.set(Game {
                        score: game.get().score + 1,
                        score_best: 0,
                    });

                    // game.score += 1;
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

    let local_game = game.get();
    if local_game.score_best < local_game.score {
        game.set(Game {
            score: game.get().score,
            score_best: game.get().score,
        });

        // game.score_best = game.score;
    };
}

fn food_event_listener(
    mut commands: Commands,
    query_board: Query<&Board>,
    mut events: EventReader<NewFoodEvent>,
    snake: Res<SnakeBody>,
    asset_server: Res<AssetServer>,
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
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    // color: MATERIALS.food,
                    custom_size: Some(Vec2::new(
                        TILE_SIZE, TILE_SIZE,
                    )),
                    ..Sprite::default()
                },
                texture: asset_server.load("apple.png"),
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

// // snake_segments that do a size change
// fn snake_segments(
//     snake: Res<SnakeBody>,
//     mut positions: Query<(&Position, &mut
// Transform)>,     query_board: Query<&Board>,
// ) {
//     let board = query_board.single();

//     let growth_rate = 1.0 /
// snake.segments.len() as f32;

//     for (i, segment) in
// snake.segments.iter().rev().enumerate() {
//         let current_position = positions
//             .iter_mut()
//             .find(|pos| pos.0 == segment);

//         if let Some((pos, mut transform)) =
// current_position         {
//             let scale =
//                 0.5 + (growth_rate * 0.5 * (i +
// 1) as f32);             transform.scale =
// Vec3::new(scale, scale, 1.0)         }
//     }
// }

#[tracing::instrument(skip(
    commands,
    positions,
    last_pressed,
    query_board,
    asset_server
))]
fn snake_segments(
    mut commands: Commands,
    snake: Res<SnakeBody>,
    mut positions: Query<(
        &Position,
        &mut TextureAtlasSprite,
        &mut Transform,
    )>,
    last_pressed: Res<LastKeyPress>,
    query_board: Query<&Board>,
    asset_server: Res<AssetServer>,
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
// for (i, segment) in
// snake.segments.iter().rev().enumerate() {
//     let current_position = positions
//         .iter_mut()
//         .find(|pos| pos.0 == segment);

//     if let Some((pos, mut transform)) =
// current_position     {
//         let scale =
//             0.5 + (growth_rate * 0.5 * (i + 1)
// as f32);         transform.scale =
// Vec3::new(scale, scale, 1.0)     }
// }

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
