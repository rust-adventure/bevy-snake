use bevy::prelude::*;
use snake::{
    board::{spawn_board, Board},
    snake::{spawn_snake, Snake},
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
        .add_systems(
            Startup,
            (setup, spawn_board, spawn_snake),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
