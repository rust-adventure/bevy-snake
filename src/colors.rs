use bevy::{
    color::{palettes, Srgba},
    prelude::Color,
};

pub const BOARD: Color = Color::srgb(0.42, 0.63, 0.07);
pub const TILE_PLACEHOLDER: Color =
    Color::srgb(0.62, 0.83, 0.27);
pub const TILE_PLACEHOLDER_DARK: Color =
    Color::srgb(0.57, 0.78, 0.22);
pub const SNAKE: Color = Color::WHITE;
pub const FOOD: Srgba = palettes::tailwind::RED_400;
pub const TEXT: Color = Color::BLACK;
