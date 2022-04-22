use bevy::prelude::*;
use iyes_loopless::prelude::*;
use snake::{
    board::spawn_board, controls::ControlsPlugin,
    snake::Snake, tick,
};
use std::time::Duration;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ControlsPlugin)
        .insert_resource(ClearColor(Color::rgb(
            0.52, 0.73, 0.17,
        )))
        .init_resource::<Snake>()
        .add_startup_system(setup)
        .add_startup_system(spawn_board)
        .add_stage_before(
            CoreStage::Update,
            "snake_tick",
            FixedTimestepStage::new(Duration::from_millis(
                100,
            ))
            .with_stage(
                SystemStage::parallel().with_system(tick),
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d());
}
