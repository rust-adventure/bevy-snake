use std::collections::VecDeque;

use bevy::{
    math::Quat,
    prelude::{Entity, Query, Res, Resource, Transform},
    sprite::TextureAtlasSprite,
};
use itertools::Itertools;

use crate::{
    board::position::{Position, RelativePosition},
    settings::GameSettings,
};

#[derive(Debug, Default, Resource)]
pub struct Snake {
    pub segments: VecDeque<Entity>,
}

pub fn render_snake_segments(
    snake: Res<Snake>,
    mut positions: Query<(
        &Position,
        &mut TextureAtlasSprite,
        &mut Transform,
    )>,
    settings: Res<GameSettings>,
) {
    use RelativePosition::*;

    let snake_texture_index = settings.snake_index;

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
