use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<ImageAssets>()
            .init_collection::<AudioAssets>()
            .init_collection::<FontAssets>();
    }
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "AlfaSlabOne-Regular.ttf")]
    pub alfa_slab_one_regular: Handle<Font>,
    #[asset(path = "roboto.ttf")]
    pub roboto: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "gameover.ogg")]
    pub gameover: Handle<AudioSource>,
    #[asset(path = "apple.ogg")]
    pub apple: Handle<AudioSource>,
    #[asset(path = "menu_click.ogg")]
    pub menu_click: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "apple.png")]
    pub apple: Handle<Image>,
    #[asset(path = "green_panel.png")]
    pub green_panel: Handle<Image>,
    #[asset(path = "blue_button09.png")]
    pub blue_button09: Handle<Image>,
    #[asset(path = "blue_button10.png")]
    pub blue_button10: Handle<Image>,
    #[asset(path = "grey_box.png")]
    pub box_unchecked: Handle<Image>,
    #[asset(path = "green_boxCheckmark.png")]
    pub box_checked: Handle<Image>,
    #[asset(texture_atlas(
        tile_size_x = 16.,
        tile_size_y = 16.,
        columns = 3,
        rows = 1,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "grass.png")]
    pub grass: Handle<TextureAtlas>,
    #[asset(texture_atlas(
        tile_size_x = 136.,
        tile_size_y = 136.,
        columns = 4,
        rows = 30,
        padding_x = 0.,
        padding_y = 0.
    ))]
    #[asset(path = "snake_sprites.png")]
    pub snake: Handle<TextureAtlas>,
}
