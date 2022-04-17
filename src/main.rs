use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
// use bevy_ninepatch::NinePatchPlugin;
use bevy_snake::{
    board::{spawn_board, Position},
    common::{Game, RunState},
    control::{user_input, LastKeyPress},
    food::{food_event_listener, Food, NewFoodEvent},
    scoring::SpeedrunPlugin,
    settings::GameSettings,
    snake::{
        new_game_spawns, render_snake_segments, SnakeBody,
        SnakeTextureSelection,
    },
    snake_movement,
    ui::GameUiPlugin,
};
use iyes_loopless::prelude::*;
use kayak_ui::bevy::BevyKayakUIPlugin;
use std::time::Duration;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(BevyKayakUIPlugin)
        .add_plugin(SpeedrunPlugin)
        .add_plugin(GameUiPlugin)
        .add_plugin(AudioPlugin)
        .add_event::<NewFoodEvent>()
        .insert_resource(ClearColor(Color::rgb(
            0.52, 0.73, 0.17,
        )))
        .init_resource::<Game>()
        .init_resource::<SnakeBody>()
        .init_resource::<SnakeTextureSelection>()
        .init_resource::<GameSettings>()
        .init_resource::<LastKeyPress>()
        .add_startup_system(setup)
        .add_startup_system(spawn_board)
        .add_state(RunState::Menu)
        .add_system_set(
            SystemSet::on_update(RunState::Playing)
                .with_system(user_input)
                .with_system(food_event_listener)
                .with_system(render_snake_segments),
        )
        .add_system_set(
            SystemSet::on_enter(RunState::Playing)
                .with_system(game_reset)
                .with_system(new_game_spawns),
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

fn game_reset(
    mut commands: Commands,
    mut snake: ResMut<SnakeBody>,
    mut positions: Query<(Entity, &Position)>,
    mut last_pressed: ResMut<LastKeyPress>,
    food_query: Query<Entity, With<Food>>,
) {
    for entity in food_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for position in &snake.segments {
        if let Some((entity, _)) = positions
            .iter_mut()
            .find(|pos| pos.1 == position)
        {
            commands.entity(entity).despawn_recursive();
        }
    }
    *snake = SnakeBody::default();
    *last_pressed = LastKeyPress(KeyCode::Right);
}
