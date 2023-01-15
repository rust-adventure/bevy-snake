use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use iyes_loopless::prelude::*;
use snake::{
    assets::AssetsPlugin,
    board::spawn_board,
    controls::ControlsPlugin,
    food::FoodPlugin,
    reset_game,
    scoring::ScorePlugin,
    settings::SettingsPlugin,
    snake::{render_snake_segments, Snake},
    tick,
    ui::UiPlugin,
    GameState, STARTING_GAME_STATE,
};
use std::time::Duration;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0.52, 0.73, 0.17,
        )))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Snake!".to_string(),
                ..default()
            },
            ..default()
        }))
        .add_loopless_state(STARTING_GAME_STATE)
        .add_fixed_timestep(
            Duration::from_millis(100),
            "snake_tick",
        )
        // .add_fixed_timestep_child_stage("snake_tick")
        .add_fixed_timestep_system(
            "snake_tick",
            0,
            tick.run_in_state(GameState::Playing),
        )
        .add_plugin(AudioPlugin)
        .add_plugin(SettingsPlugin)
        .add_plugin(ControlsPlugin)
        .add_plugin(FoodPlugin)
        .add_plugin(AssetsPlugin)
        .add_plugin(UiPlugin)
        .init_resource::<Snake>()
        .add_plugin(ScorePlugin)
        .add_startup_system(setup)
        .add_startup_system(spawn_board)
        .add_system(
            render_snake_segments
                .run_in_state(GameState::Playing),
        )
        .add_enter_system(GameState::Playing, reset_game)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
