use bevy::{color::palettes::tailwind::*, prelude::*};
use bevy_rand::prelude::*;
use itertools::Itertools;
use rand::seq::IndexedRandom;

use crate::{BOARD_SIZE, SnakeSegment, TILE_SIZE, tile_to_world};

pub struct ApplePlugin;

impl Plugin for ApplePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(new_food_event_triggers);
    }
}
#[derive(Event)]
pub struct NewApple;

pub fn new_food_event_triggers(
    _: On<NewApple>,
    mut commands: Commands,
    occupied_positions: Query<&SnakeSegment>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
) {
    let possible_food_locations = (0..(BOARD_SIZE.y))
        .cartesian_product(0..(BOARD_SIZE.x))
        .map(|(y, x)| UVec2::new(x, y))
        .filter(|tile| {
            !occupied_positions
                .iter()
                .any(|segment| &segment.position == tile)
        })
        .collect::<Vec<UVec2>>();

    let Some(pos) = possible_food_locations.choose(&mut rng) else {
        error!("can't find valid apple spawning space");
        return;
    };

    commands.spawn_scene(bsn! {
        @Apple {
            @x: {pos.x},
            @y: {pos.y}
        }
    });
}
#[derive(SceneComponent, Clone, Default)]
#[scene(AppleProps)]
pub struct Apple {
    pub position: UVec2,
}

#[derive(Default, Clone)]
pub struct AppleProps {
    pub x: u32,
    pub y: u32,
}

impl Apple {
    fn scene(AppleProps { x, y }: AppleProps) -> impl Scene {
        let position = UVec2::new(x, y);
        let world_position = tile_to_world(position);
        bsn! {
            Apple {
                position
            }
            Sprite {
                color: RED_400,
                custom_size: Vec2::splat(TILE_SIZE)
            }
            Transform {
                translation: {world_position.extend(1.)}
            }
        }
    }
}
