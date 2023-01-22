use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<ImageAssets>()
            .init_collection::<AudioAssets>();
    }
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "gameover.ogg")]
    pub gameover: Handle<bevy_kira_audio::AudioSource>,
    #[asset(path = "apple.ogg")]
    pub apple: Handle<bevy_kira_audio::AudioSource>,
    #[asset(path = "menu_click.ogg")]
    pub menu_click: Handle<bevy_kira_audio::AudioSource>,
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
    #[asset(
        paths(
            "snake_heads/snake_sprites_01.png",
            "snake_heads/snake_sprites_02.png",
            "snake_heads/snake_sprites_03.png",
            "snake_heads/snake_sprites_04.png",
            "snake_heads/snake_sprites_05.png",
            "snake_heads/snake_sprites_06.png",
            "snake_heads/snake_sprites_07.png",
            "snake_heads/snake_sprites_08.png",
            "snake_heads/snake_sprites_09.png",
            "snake_heads/snake_sprites_10.png",
            "snake_heads/snake_sprites_11.png",
            "snake_heads/snake_sprites_12.png",
            "snake_heads/snake_sprites_13.png",
            "snake_heads/snake_sprites_14.png",
            "snake_heads/snake_sprites_15.png",
            "snake_heads/snake_sprites_16.png",
            "snake_heads/snake_sprites_17.png",
            "snake_heads/snake_sprites_18.png",
            "snake_heads/snake_sprites_19.png",
            "snake_heads/snake_sprites_20.png",
            "snake_heads/snake_sprites_21.png",
            "snake_heads/snake_sprites_22.png",
            "snake_heads/snake_sprites_23.png",
            "snake_heads/snake_sprites_24.png",
            "snake_heads/snake_sprites_25.png",
            "snake_heads/snake_sprites_26.png",
            "snake_heads/snake_sprites_27.png",
            "snake_heads/snake_sprites_28.png",
            "snake_heads/snake_sprites_29.png",
            "snake_heads/snake_sprites_30.png"
        ),
        collection(typed)
    )]
    pub snake_heads: Vec<Handle<Image>>,
}
