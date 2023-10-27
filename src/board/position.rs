use bevy::prelude::*;

#[derive(
    Debug,
    PartialEq,
    Deref,
    DerefMut,
    Copy,
    Clone,
    Eq,
    Hash,
    Component,
)]
pub struct Position(pub IVec2);

#[derive(Debug)]
pub enum RelativePosition {
    North,
    South,
    East,
    West,
}
use RelativePosition::*;

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self(IVec2::new(x, y))
    }
    pub fn detect_side(
        self: &Position,
        other: &Position,
    ) -> RelativePosition {
        if other.y > self.y {
            North
        } else if other.y < self.y {
            South
        } else if other.x > self.x {
            East
        } else if other.x < self.x {
            West
        } else {
            panic!("should never happen");
        }
    }
}

impl From<RelativePosition> for Quat {
    fn from(value: RelativePosition) -> Self {
        match value {
            North => Quat::from_rotation_z(0.0),
            South => {
                Quat::from_rotation_z(std::f32::consts::PI)
            }
            East => Quat::from_rotation_z(
                -std::f32::consts::FRAC_PI_2,
            ),
            West => Quat::from_rotation_z(
                std::f32::consts::FRAC_PI_2,
            ),
        }
    }
}
