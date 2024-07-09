use bevy::prelude::*;
use rand::prelude::SliceRandom;

use crate::{
    assets::ImageAssets,
    board::{position::Position, Board, TILE_SIZE},
};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NewFoodEvent>()
            .add_event::<SpawnApple>()
            .observe(food_event_listener)
            .observe(spawn_apples);
    }
}
#[derive(Event)]
pub struct NewFoodEvent;

#[derive(Component)]
pub struct Food;

pub fn food_event_listener(
    _trigger: Trigger<NewFoodEvent>,
    mut commands: Commands,
    board: Res<Board>,
    positions: Query<&Position>,
) {
    let possible_food_locations = board
        .tiles()
        .filter(|tile| {
            !positions.iter().any(|pos| pos == tile)
        })
        .collect::<Vec<Position>>();

    let mut rng = rand::thread_rng();
    if let Some(pos) =
        possible_food_locations.choose(&mut rng)
    {
        commands.trigger(SpawnApple { position: *pos });
    } else {
        error!("can't find valid apple spawning space");
    }
}

#[derive(Event)]
struct SpawnApple {
    position: Position,
}
fn spawn_apples(
    trigger: Trigger<SpawnApple>,
    mut commands: Commands,
    board: Res<Board>,
    image_assets: Res<ImageAssets>,
) {
    let position = trigger.event().position;
    let x = board.cell_position_to_physical(position.x);
    let y = board.cell_position_to_physical(position.y);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..default()
            },
            texture: image_assets.apple.clone(),
            transform: Transform::from_xyz(x, y, 2.0),
            ..default()
        },
        position,
        Food,
    ));
}
