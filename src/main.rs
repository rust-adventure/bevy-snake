use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
// use bevy_ninepatch::NinePatchPlugin;
use bevy_snake::{
    board::spawn_board,
    common::{Game, RunState},
    control::{user_input, LastKeyPress},
    food::{food_event_listener, NewFoodEvent},
    reset_game,
    scoring::SpeedrunPlugin,
    settings::GameSettings,
    snake::{
        render_snake_segments, SnakeBody,
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
        .init_resource::<Keep>()
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
                .with_system(reset_game),
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

struct Keep(Handle<bevy::render::texture::Image>);

impl FromWorld for Keep {
    fn from_world(world: &mut World) -> Self {
        let asset_server =
            world.get_resource::<AssetServer>().unwrap();
        let texture_handle: Handle<
            bevy::render::texture::Image,
        > = asset_server.load("snake_sprites.png");
        Keep(texture_handle)
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d());
}
