use crate::{
    assets::{AudioAssets, ImageAssets},
    settings::{AudioSettings, GameSettings},
};
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};

#[derive(Component)]
pub struct SnakeHead(usize);

pub fn snake_selector_interaction(
    mut settings: ResMut<GameSettings>,
    mut interaction_query: Query<
        (&Interaction, &SnakeHead),
        (Changed<Interaction>, With<Button>),
    >,
    audio: Res<Audio>,
    sounds: Res<AudioAssets>,
) {
    for (interaction, snake_head) in &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                if settings.audio == AudioSettings::ON {
                    audio.play(sounds.apple.clone());
                }
                settings.snake_index = snake_head.0;
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

#[derive(Component)]
pub struct CurrentSnake;

pub fn update_current_snake(
    settings: ResMut<GameSettings>,
    mut image_query: Query<
        &mut UiTextureAtlasImage,
        With<CurrentSnake>,
    >,
) {
    for mut image in &mut image_query {
        image.index = settings.snake_index;
    }
}

pub fn spawn_snake_selector(
    parent: &mut ChildBuilder,
    images: Res<ImageAssets>,
    asset_server: Res<AssetServer>,
    current_snake_index: usize,
    atlases: &Res<Assets<TextureAtlas>>,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Auto,
                height: Val::Px(25.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                AtlasImageBundle {
                    style: Style {
                        width: Val::Px(25.0),
                        height: Val::Px(25.0),
                        margin: UiRect::right(Val::Px(
                            10.0,
                        )),
                        ..default()
                    },
                    texture_atlas: images.snake.clone(),
                    texture_atlas_image:
                        UiTextureAtlasImage {
                            index: current_snake_index,
                            ..default()
                        },
                    ..default()
                },
                CurrentSnake,
            ));
            parent.spawn(TextBundle::from_section(
                "Current Snake",
                TextStyle {
                    font: asset_server.load("roboto.ttf"),
                    font_size: 25.0,
                    color: Color::rgb(0.0, 0.0, 0.0),
                },
            ));
        });

    parent
        .spawn(NodeBundle {
            style: Style {
                flex_wrap: FlexWrap::Wrap,
                width: Val::Percent(100.0),
                height: Val::Auto,
                justify_content:
                    JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            let atlas = atlases
                .get(&images.snake)
                .expect("snake textureatlas to be loaded");

            for (i, _rect) in
                atlas.textures.iter().enumerate().step_by(4)
            {
                parent
                    .spawn((
                        ButtonBundle::default(),
                        SnakeHead(i),
                    ))
                    .with_children(|parent| {
                        parent.spawn(AtlasImageBundle {
                            style: Style {
                                width: Val::Px(50.0),
                                height: Val::Px(50.0),
                                margin: UiRect::all(
                                    Val::Px(3.0),
                                ),
                                ..default()
                            },
                            texture_atlas: images
                                .snake
                                .clone(),
                            texture_atlas_image:
                                UiTextureAtlasImage {
                                    index: i,
                                    ..default()
                                },
                            ..default()
                        });
                    });
            }
        });
}
