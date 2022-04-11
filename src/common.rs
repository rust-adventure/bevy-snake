use bevy::{
    prelude::{AssetServer, FromWorld, Handle, World},
    text::Font,
};

pub struct FontSpec {
    pub family: Handle<Font>,
}

impl FromWorld for FontSpec {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world
            .get_resource_mut::<AssetServer>()
            .unwrap();
        FontSpec {
            family: asset_server
                .load("fonts/FiraSans-Bold.ttf"),
        }
    }
}

// GameStates and FixedTimesteps can not be used
// together yes, instead use iyes crate https://canary.discord.com/channels/691052431525675048/956767127291965500/956770647911059477
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RunState {
    Playing,
    GameOver,
    Menu,
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct Game {
    pub score: u32,
    pub score_best: u32,
}
