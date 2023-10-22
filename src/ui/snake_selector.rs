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
                // every snake head in the textureatlas is
                // at 0,4,8,12,..
                // so we multiply by 4
                settings.snake_index = snake_head.0 * 4;
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
    images: Res<ImageAssets>,
    mut image_query: Query<
        &mut UiImage,
        With<CurrentSnake>,
    >,
) {
    for mut image in &mut image_query {
        *image = UiImage::new(
            images.snake_heads[settings.snake_index / 4]
                .clone(),
        );
    }
}

pub fn spawn_snake_selector(
    parent: &mut ChildBuilder,
    images: Res<ImageAssets>,
    asset_server: Res<AssetServer>,
    current_snake_index: usize,
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
                ImageBundle {
                    style: Style {
                        width: Val::Px(25.0),
                        height: Val::Px(25.0),
                        margin: UiRect::right(Val::Px(
                            10.0,
                        )),
                        ..default()
                    },
                    image: UiImage::new(
                        images.snake_heads
                            [current_snake_index]
                            .clone(),
                    ),
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
            for (i, snake_head) in
                images.snake_heads.iter().enumerate()
            {
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(50.0),
                            height: Val::Px(50.0),
                            margin: UiRect::all(Val::Px(
                                3.0,
                            )),
                            ..default()
                        },
                        background_color: Color::ALICE_BLUE
                            .into(),
                        image: UiImage::new(
                            snake_head.clone(),
                        ),
                        ..default()
                    },
                    SnakeHead(i),
                ));
            }
        });
}
