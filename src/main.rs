use bevy::{ecs::system::SystemId, prelude::*};
use snake::{
    assets::AssetsPlugin,
    board::{spawn_board, Board},
    controls::ControlsPlugin,
    food::FoodPlugin,
    reset_game,
    scoring::ScorePlugin,
    settings::SettingsPlugin,
    snake::{render_snake_segments, Snake},
    tick,
    ui::UiPlugin,
    GameState,
};

#[derive(Resource)]
struct Tick(SystemId);

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::rgb(
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
    .add_state::<GameState>()
    .insert_resource(Time::<Fixed>::from_seconds(0.1))
    // .add_systems(
    //     FixedUpdate,
    //     tick.run_if(in_state(GameState::Playing)),
    // )
    .add_plugins((
        SettingsPlugin,
        ControlsPlugin,
        FoodPlugin,
        AssetsPlugin,
        UiPlugin,
    ))
    .init_resource::<Snake>()
    .add_plugins(ScorePlugin)
    .add_systems(Startup, (setup, spawn_board))
    .add_systems(
        Update,
        (
            keyinput,
            render_snake_segments
                .run_if(in_state(GameState::Playing)),
        ),
    )
    .add_systems(OnEnter(GameState::Playing), reset_game);
    let id = app.world.register_system(tick);
    app.insert_resource(Tick(id));
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn keyinput(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    sys: Res<Tick>,
) {
    if keys.just_pressed(KeyCode::Space) {
        // Space was pressed
        commands.run_system(sys.0);
    }
}
