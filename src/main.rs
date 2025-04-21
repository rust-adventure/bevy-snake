use std::time::Duration;

use bevy::{
    color::palettes::tailwind::*, prelude::*,
    render::camera::ScalingMode,
    time::common_conditions::on_timer,
};
use bevy_ecs_tilemap::TilemapPlugin;
use snake::{
    GameState, assets::AssetsPlugin, board::spawn_board,
    button_system, controls::ControlsPlugin,
    food::FoodPlugin, reset_game, snake::SnakePlugin,
    spawn_menu, tick,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::from(LIME_800)))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Snake!".into(),
                    ..default()
                }),
                ..default()
            }),
            TilemapPlugin,
        ))
        .init_state::<GameState>()
        .add_systems(
            Update,
            tick.run_if(in_state(GameState::Playing))
                .run_if(on_timer(Duration::from_secs_f32(
                    0.1,
                ))),
        )
        .add_plugins((
            ControlsPlugin,
            FoodPlugin,
            AssetsPlugin,
            SnakePlugin,
        ))
        .add_systems(
            OnEnter(GameState::Startup),
            (
                setup,
                spawn_board,
                transition_to_start_menu,
            )
                .chain(),
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

fn transition_to_start_menu(
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::Menu);
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: (20. + 4.) * 136.,
                min_height: (20. + 4.) * 136.,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}
