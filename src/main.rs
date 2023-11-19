use bevy::prelude::*;
use snake::{
    board::{spawn_board, Board},
    snake::{spawn_snake, Snake},
    tick,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
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
        .init_resource::<Snake>()
        .insert_resource(Time::<Fixed>::from_seconds(0.1))
        .add_systems(
            Startup,
            (setup, spawn_board, spawn_snake),
        )
        .add_systems(FixedUpdate, tick)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
