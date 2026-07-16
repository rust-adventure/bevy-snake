use bevy::{color::palettes::tailwind::*, prelude::*};

use crate::{BOARD_SIZE, TILE_SIZE};

#[derive(Component, FromTemplate, Clone)]
#[relationship(relationship_target = BodyFor)]
pub struct HeadOf {
    #[relationship]
    pub body: Entity,
}

#[derive(Component)]
#[relationship_target(relationship = HeadOf, linked_spawn)]
pub struct BodyFor {
    #[relationship_target]
    head: Entity,
}

#[derive(SceneComponent, Default, Clone, Debug)]
#[scene(SnakeSegmentProps)]
pub struct SnakeSegment {
    pub position: UVec2,
}

#[derive(Default, Clone)]
pub struct SnakeSegmentProps {
    pub x: u32,
    pub y: u32,
}

impl SnakeSegment {
    fn scene(props: SnakeSegmentProps) -> impl Scene {
        let x = {
            let offset = props.x as i32 - BOARD_SIZE.as_ivec2().x / 2;
            offset as f32 * TILE_SIZE
        };
        let y = {
            let offset = props.y as i32 - BOARD_SIZE.as_ivec2().y / 2;
            offset as f32 * TILE_SIZE
        };
        bsn! {
            SnakeSegment {
                position: UVec2::new(props.x, props.y)
            }
            Sprite {
                color: LIME_400,
                custom_size: Vec2::splat(TILE_SIZE)
            }
            Transform {
                translation: Vec3 {
                    x: x,
                    y: y,
                }
            }
        }
    }
}
