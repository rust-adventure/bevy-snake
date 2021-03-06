use bevy::prelude::Color;

pub struct Materials {
    pub board: Color,
    pub tile_placeholder: Color,
    pub tile_placeholder_dark: Color,
    pub tile: Color,
    pub food: Color,
    pub none: Color,
    pub screen: Color,
}
pub const MATERIALS: Materials = Materials {
    board: Color::rgb(0.7, 0.7, 0.8),
    tile_placeholder: Color::rgb(0.75, 0.75, 0.9),
    tile_placeholder_dark: Color::rgb(0.70, 0.70, 0.85),
    tile: Color::rgb(0.9, 0.9, 1.0),
    food: Color::rgb(0.9, 0.1, 0.1),
    none: Color::NONE,
    screen: Color::rgba(0.0, 0.0, 0.0, 0.2),
};

pub struct ButtonMaterials {
    pub none: Color,
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
}

pub const BUTTON_MATERIALS: ButtonMaterials =
    ButtonMaterials {
        none: Color::NONE,
        normal: Color::rgb(0.75, 0.75, 0.9),
        hovered: Color::rgb(0.7, 0.7, 0.9),
        pressed: Color::rgb(0.6, 0.6, 1.0),
    };
