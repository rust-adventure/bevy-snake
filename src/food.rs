use bevy::prelude::*;
use bevy_ecs_tilemap::{
    map::{TilemapId, TilemapSize},
    tiles::{
        TileBundle, TilePos, TileStorage, TileTextureIndex,
    },
};
use itertools::Itertools;
use rand::prelude::SliceRandom;

use crate::{board::SnakeLayer, snake::SnakeSegment};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NewFoodEvent>()
            .add_event::<SpawnAppleEvent>()
            .add_observer(new_food_event_triggers)
            .add_observer(spawn_apple_event_triggers);
    }
}
#[derive(Event)]
pub struct NewFoodEvent;

#[derive(Component)]
pub struct Food;

pub fn new_food_event_triggers(
    _trigger: Trigger<NewFoodEvent>,
    mut commands: Commands,
    positions: Query<
        &TilePos,
        Or<(With<SnakeSegment>, With<Food>)>,
    >,

    map: Single<&TilemapSize, With<SnakeLayer>>,
) {
    let possible_food_locations = (0..(map.x as i32))
        .cartesian_product(0..(map.y as i32))
        .filter(|tile| {
            !positions.iter().any(|pos| {
                (pos.x, pos.y)
                    == (tile.0 as u32, tile.1 as u32)
            })
        })
        .collect::<Vec<(i32, i32)>>();

    let mut rng = rand::thread_rng();
    let Some(pos) =
        possible_food_locations.choose(&mut rng)
    else {
        error!("can't find valid apple spawning space");
        return;
    };

    commands.trigger(SpawnAppleEvent {
        position: TilePos::new(pos.0 as u32, pos.1 as u32),
    });
}

#[derive(Event)]
struct SpawnAppleEvent {
    position: TilePos,
}

fn spawn_apple_event_triggers(
    trigger: Trigger<SpawnAppleEvent>,
    mut commands: Commands,
    tilemap: Single<
        (Entity, &mut TileStorage),
        With<SnakeLayer>,
    >,
) {
    let (tilemap_entity, mut tile_storage) =
        tilemap.into_inner();

    let tile_pos = trigger.event().position;

    let tile_entity = commands
        .spawn((
            TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(tilemap_entity),
                texture_index: TileTextureIndex(116),
                ..Default::default()
            },
            Food,
        ))
        .id();
    tile_storage.set(&tile_pos, tile_entity);
}
