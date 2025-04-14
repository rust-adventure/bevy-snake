use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_ecs_tilemap::{
    map::TilemapId,
    tiles::{
        TileBundle, TileFlip, TilePos, TileStorage,
        TileTextureIndex,
    },
};
use itertools::Itertools;

use crate::{GameState, board::SnakeLayer, tick};

mod position;
use position::*;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<SpawnSnakeSegmentEvent>()
            .insert_resource(SnakeHeadTextureIndex(8))
            .init_resource::<Snake>()
            .add_systems(
                Update,
                render_snake_segments
                    .after(tick)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_observer(spawn_snake_segment);
    }
}

#[derive(Resource)]
struct SnakeHeadTextureIndex(u32);

#[derive(Component, Debug)]
pub struct SnakeSegment;

#[derive(Debug, Default, Resource)]
pub struct Snake {
    pub segments: VecDeque<Entity>,
}

fn detect_side(
    first: &TilePos,
    other: &TilePos,
) -> RelativePosition {
    if other.y > first.y {
        RelativePosition::North
    } else if other.y < first.y {
        RelativePosition::South
    } else if other.x > first.x {
        RelativePosition::East
    } else if other.x < first.x {
        RelativePosition::West
    } else {
        panic!("should never happen");
    }
}

fn render_snake_segments(
    snake: Res<Snake>,
    mut positions: Query<
        (
            &TilePos,
            &mut TileTextureIndex,
            &mut TileFlip,
        ),
        With<SnakeSegment>,
    >,
    snake_texture_index: Res<SnakeHeadTextureIndex>,
) {
    use RelativePosition::*;

    // head
    if let Some((first, second)) =
        snake.segments.iter().tuple_windows().next()
    {
        let pos = positions.get(*first).unwrap().0;
        let pos_second = positions.get(*second).unwrap().0;
        let flip =
            TileFlip::from(detect_side(pos, pos_second));
        let (_, mut sprite, mut tile_flip) =
            positions.get_mut(*first).unwrap();
        sprite.0 = snake_texture_index.0;
        *tile_flip = flip;
    }

    // tail
    if let Some((second_to_last, last)) =
        snake.segments.iter().tuple_windows().last()
    {
        let pos = positions.get(*last).unwrap().0;
        let second_to_last_pos =
            positions.get(*second_to_last).unwrap().0;

        let flip = TileFlip::from(detect_side(
            pos,
            second_to_last_pos,
        ));

        let (_, mut sprite, mut tile_flip) =
            positions.get_mut(*last).unwrap();
        sprite.0 = snake_texture_index.0 + 3;
        *tile_flip = flip
    }

    for (front, origin, back) in
        snake.segments.iter().tuple_windows()
    {
        let front_pos = positions.get(*front).unwrap().0;
        let origin_pos = positions.get(*origin).unwrap().0;
        let back_pos = positions.get(*back).unwrap().0;

        let image = match (
            detect_side(origin_pos, front_pos),
            detect_side(origin_pos, back_pos),
        ) {
            // vertical
            (South, North) | (North, South) => (
                snake_texture_index.0 + 1,
                TileFlip::default(),
            ),
            // horizontal
            (East, West) | (West, East) => (
                snake_texture_index.0 + 1,
                TileFlip {
                    d: true,
                    ..default()
                },
            ),
            // ⌞
            (North, East) | (East, North) => (
                snake_texture_index.0 + 2,
                TileFlip {
                    d: true,
                    y: true,
                    ..default()
                },
            ),
            // ⌜
            (East, South) | (South, East) => (
                snake_texture_index.0 + 2,
                TileFlip {
                    d: true,
                    ..default()
                },
            ),
            // ⌟
            (West, North) | (North, West) => (
                snake_texture_index.0 + 2,
                TileFlip {
                    y: true,
                    x: true,
                    ..default()
                },
            ),
            // ⌝
            (West, South) | (South, West) => (
                snake_texture_index.0 + 2,
                TileFlip {
                    d: true,
                    x: true,
                    ..default()
                },
            ),
            _ => panic!("unhandled"),
        };

        let (_, mut sprite, mut tile_flip) =
            positions.get_mut(*origin).unwrap();
        sprite.0 = image.0;
        *tile_flip = image.1;
    }
}

#[derive(Event)]
pub struct SpawnSnakeSegmentEvent {
    pub position: TilePos,
}

fn spawn_snake_segment(
    trigger: Trigger<SpawnSnakeSegmentEvent>,
    mut commands: Commands,
    mut snake: ResMut<Snake>,
    tilemap: Single<
        (Entity, &mut TileStorage),
        With<SnakeLayer>,
    >,
    snake_texture_index: Res<SnakeHeadTextureIndex>,
) {
    let (tilemap_entity, mut tile_storage) =
        tilemap.into_inner();

    let tile_pos = trigger.event().position;

    let tile_entity = commands
        .spawn((
            TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(tilemap_entity),
                texture_index: TileTextureIndex(
                    snake_texture_index.0,
                ),
                ..Default::default()
            },
            SnakeSegment,
        ))
        .id();
    tile_storage.set(&tile_pos, tile_entity);

    snake.segments.push_front(tile_entity);
}
