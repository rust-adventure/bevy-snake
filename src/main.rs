use std::time::Duration;

use bevy::{
    camera::ScalingMode, color::palettes::tailwind::*, prelude::*,
    time::common_conditions::on_timer,
};
use bevy_rand::prelude::*;
use snake::{
    GameState, apple::ApplePlugin, board_scene, controls::ControlsPlugin, menu_scene,
    new_game_scene, reset_game, tick,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::from(LIME_800)))
        .add_plugins(DefaultPlugins)
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins((ControlsPlugin, ApplePlugin))
        .init_state::<GameState>()
        .add_systems(
            Update,
            tick.run_if(
                in_state(GameState::Playing).and_then(on_timer(Duration::from_secs_f32(0.1))),
            ),
        )
        .add_systems(
            OnEnter(GameState::Startup),
            (setup.spawn(), board_scene.spawn(), transition_to_start_menu).chain(),
        )
        .add_systems(
            OnEnter(GameState::Playing),
            (reset_game, new_game_scene.spawn()).chain(),
        )
        .add_systems(OnEnter(GameState::Menu), menu_scene.spawn())
        .run();
}

fn transition_to_start_menu(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Menu);
}

fn setup() -> impl Scene {
    bsn! {
        Camera2d
        template_value(Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: (20. + 4.) * 136.,
                min_height: (20. + 4.) * 136.,
            },
            ..OrthographicProjection::default_2d()
        }))
    }
}
