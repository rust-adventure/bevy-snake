use bevy::prelude::*;
use snake::{
    assets::AssetsPlugin,
    board::{spawn_board, Board},
    button_system,
    controls::ControlsPlugin,
    food::FoodPlugin,
    reset_game,
    snake::SnakePlugin,
    spawn_menu, tick, GameState,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(
            0.52, 0.73, 0.17,
        )))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake!".into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Board::new(20))
        .init_state::<GameState>()
        .insert_resource(Time::<Fixed>::from_seconds(0.1))
        .add_systems(
            FixedUpdate,
            tick.run_if(in_state(GameState::Playing)),
        )
        .add_plugins((
            ControlsPlugin,
            FoodPlugin,
            AssetsPlugin,
            SnakePlugin,
        ))
        .add_systems(
            OnEnter(GameState::Startup),
            (setup, spawn_board, start_menu).chain(),
        )
        .add_systems(
            OnEnter(GameState::Playing),
            reset_game,
        )
        .add_systems(OnEnter(GameState::Menu), spawn_menu)
        .add_systems(
            Update,
            button_system.run_if(in_state(GameState::Menu)),
        )
        .enable_state_scoped_entities::<GameState>()
        .run();
}

fn start_menu(
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::Menu);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
