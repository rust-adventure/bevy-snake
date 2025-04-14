use bevy::prelude::*;

#[derive(Debug)]
pub enum RelativePosition {
    North,
    South,
    East,
    West,
}
use RelativePosition::*;
use bevy_ecs_tilemap::tiles::TileFlip;

impl From<RelativePosition> for TileFlip {
    fn from(value: RelativePosition) -> Self {
        match value {
            North => TileFlip::default(),
            South => TileFlip {
                y: true,
                ..default()
            },
            East => TileFlip {
                d: true,
                x: true,
                ..default()
            },
            West => TileFlip {
                d: true,
                ..default()
            },
        }
    }
}
