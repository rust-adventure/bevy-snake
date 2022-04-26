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
        .insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(SettingsPlugin)
        .add_plugin(ControlsPlugin)
        .add_plugin(FoodPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(AssetsPlugin)
        .insert_resource(ClearColor(Color::rgb(
            0.52, 0.73, 0.17,
        )))
        .init_resource::<Snake>()
        .add_loopless_state(STARTING_GAME_STATE)
        .add_plugin(ScorePlugin)
        .add_startup_system(setup)
        .add_startup_system(spawn_board)
        .add_system(
            render_snake_segments
                .run_in_state(GameState::Playing),
        )
        .add_enter_system(GameState::Playing, reset_game)
        .add_stage_before(
            CoreStage::Update,
            "snake_tick",
            FixedTimestepStage::new(Duration::from_millis(
                100,
            ))
            .with_stage(
                SystemStage::parallel().with_system(
                    tick.run_in_state(GameState::Playing),
                ),
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d());
}
