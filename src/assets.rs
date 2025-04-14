use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<ImageAssets>();
    }
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "apple.png")]
    pub apple: Handle<Image>,
    #[asset(texture_atlas(
        tile_size_x = 136,
        tile_size_y = 136,
        columns = 3,
        rows = 1,
        padding_x = 0,
        padding_y = 0
    ))]
    pub grass_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "grass.png")]
    pub grass: Handle<Image>,
    #[asset(texture_atlas_layout(
        tile_size_x = 136,
        tile_size_y = 136,
        columns = 4,
        rows = 30,
        padding_x = 0,
        padding_y = 0
    ))]
    pub snake_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "snake_sprites.png")]
    pub snake: Handle<Image>,
}
