use std::collections::VecDeque;

use bevy::{
    app::{Plugin, Update},
    math::{Quat, Vec2},
    prelude::{
        default, Commands, Entity, Event,
        IntoSystemConfigs, Query, Res, ResMut, Resource,
        Transform, Trigger,
    },
    sprite::{Sprite, SpriteBundle, TextureAtlas},
    state::condition::in_state,
};
use itertools::Itertools;

use crate::{
    assets::ImageAssets,
    board::{
        position::{Position, RelativePosition},
        Board, TILE_SIZE,
    },
    GameState,
};

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<SpawnSnakeSegmentEvent>()
            .init_resource::<Snake>()
            .add_systems(
                Update,
                render_snake_segments
                    .run_if(in_state(GameState::Playing)),
            )
            .observe(spawn_snake_segment);
    }
}

#[derive(Debug, Default, Resource)]
pub struct Snake {
    pub segments: VecDeque<Entity>,
}

pub fn render_snake_segments(
    snake: Res<Snake>,
    mut positions: Query<(
        &Position,
        &mut TextureAtlas,
        &mut Transform,
    )>,
) {
    use RelativePosition::*;

    let snake_texture_index = 0;

    // head
    if let Some((first, second)) =
        snake.segments.iter().tuple_windows().next()
    {
        let pos = positions.get(*first).unwrap().0;
        let pos_second = positions.get(*second).unwrap().0;
        let rotation =
            Quat::from(pos.detect_side(pos_second));
        let (_, mut sprite, mut transform) =
            positions.get_mut(*first).unwrap();
        sprite.index = snake_texture_index;
        transform.rotation = rotation;
    }

    // tail
    if let Some((second_to_last, last)) =
        snake.segments.iter().tuple_windows().last()
    {
        let pos = positions.get(*last).unwrap().0;
        let second_to_last_pos =
            positions.get(*second_to_last).unwrap().0;

        let rotation =
            Quat::from(pos.detect_side(second_to_last_pos));

        let (_, mut sprite, mut transform) =
            positions.get_mut(*last).unwrap();
        sprite.index = snake_texture_index + 3;
        transform.rotation = rotation;
    }

    for (front, origin, back) in
        snake.segments.iter().tuple_windows()
    {
        let front_pos = positions.get(*front).unwrap().0;
        let origin_pos = positions.get(*origin).unwrap().0;
        let back_pos = positions.get(*back).unwrap().0;

        let image = match (
            origin_pos.detect_side(front_pos),
            origin_pos.detect_side(back_pos),
        ) {
            // vertical
            (South, North) | (North, South) => (
                snake_texture_index + 1,
                Quat::from_rotation_z(0.0),
            ),
            // horizontal
            (East, West) | (West, East) => (
                snake_texture_index + 1,
                Quat::from_rotation_z(
                    std::f32::consts::FRAC_PI_2,
                ),
            ),
            // ⌞
            (North, East) | (East, North) => (
                snake_texture_index + 2,
                Quat::from_rotation_z(
                    std::f32::consts::FRAC_PI_2,
                ),
            ),
            // ⌜
            (East, South) | (South, East) => (
                snake_texture_index + 2,
                Quat::from_rotation_z(0.0),
            ),
            // ⌟
            (West, North) | (North, West) => (
                snake_texture_index + 2,
                Quat::from_rotation_z(std::f32::consts::PI),
            ),
            // ⌝
            (West, South) | (South, West) => (
                snake_texture_index + 2,
                Quat::from_rotation_z(
                    -std::f32::consts::FRAC_PI_2,
                ),
            ),
            _ => panic!("unhandled"),
        };

        let (_, mut sprite, mut transform) =
            positions.get_mut(*origin).unwrap();
        sprite.index = image.0;
        transform.rotation = image.1;
    }
}

#[derive(Event)]
pub struct SpawnSnakeSegmentEvent {
    pub position: Position,
}
fn spawn_snake_segment(
    trigger: Trigger<SpawnSnakeSegmentEvent>,
    mut commands: Commands,
    board: Res<Board>,
    image_assets: Res<ImageAssets>,
    mut snake: ResMut<Snake>,
) {
    let position = trigger.event().position;
    let x = board.cell_position_to_physical(position.x);
    let y = board.cell_position_to_physical(position.y);

    let entity = commands
        .spawn((
            SpriteBundle {
                texture: image_assets.snake.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(
                        TILE_SIZE,
                    )),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 2.0),
                ..default()
            },
            TextureAtlas {
                index: 8,
                layout: image_assets.snake_layout.clone(),
            },
            position,
        ))
        .id();

    snake.segments.push_front(entity);
}
