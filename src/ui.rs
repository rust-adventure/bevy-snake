use self::snake_selector::{
    snake_selector_interaction, update_current_snake,
};
use crate::{
    assets::{AudioAssets, FontAssets, ImageAssets},
    scoring::{HighScore, Score},
    settings::{AudioSettings, GameSettings},
    GameState,
};
use bevy::prelude::*;
mod button;
mod snake_selector;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(MenuPage::Main)
            .add_systems(
                PostStartup,
                (pause_ui, playing_ui),
            )
            .add_systems(
                OnEnter(GameState::Menu),
                show_menu,
            )
            .add_systems(OnExit(GameState::Menu), hide_menu)
            .add_systems(
                Update,
                (button::text_button_system, scoreboard),
            )
            .add_systems(
                Update,
                (
                    change_menu,
                    audio_state,
                    snake_selector_interaction,
                    update_current_snake,
                )
                    .run_if(in_state(GameState::Menu)),
            );
    }
}

#[derive(Resource, Component, Debug, PartialEq)]
pub enum MenuPage {
    Main,
    Settings,
}

#[derive(Component)]
struct MainMenu;

fn show_menu(
    mut menu: Query<&mut Visibility, With<MainMenu>>,
) {
    let mut menu = menu.single_mut();
    *menu = Visibility::Visible;
}
fn hide_menu(
    mut menu: Query<&mut Visibility, With<MainMenu>>,
) {
    let mut menu = menu.single_mut();
    *menu = Visibility::Hidden;
}
fn change_menu(
    menu: Res<MenuPage>,
    mut menu_pages: Query<(&MenuPage, &mut Visibility)>,
) {
    if menu.is_changed() {
        for (page, mut visibility) in menu_pages.iter_mut()
        {
            if page == &*menu {
                *visibility = Visibility::Inherited;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

fn audio_state(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut UiImage),
        (
            Changed<Interaction>,
            With<Button>,
            With<AudioSettingsCheckbox>,
        ),
    >,
    images: Res<ImageAssets>,
    mut settings: ResMut<GameSettings>,
    sounds: Res<AudioAssets>,
) {
    for (interaction, mut image) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if settings.audio == AudioSettings::ON {
                    commands.spawn(AudioBundle {
                        source: sounds.apple.clone(),
                        ..default()
                    });
                }
                settings.audio = match settings.audio {
                    AudioSettings::ON => AudioSettings::OFF,
                    AudioSettings::OFF => AudioSettings::ON,
                };
                *image =
                    UiImage::new(match settings.audio {
                        AudioSettings::ON => {
                            images.box_checked.clone()
                        }
                        AudioSettings::OFF => {
                            images.box_unchecked.clone()
                        }
                    });
            }
            _ => {}
        }
    }
}

#[derive(Component)]
struct AudioSettingsCheckbox;

pub fn pause_ui(
    mut commands: Commands,
    images: Res<ImageAssets>,
    atlases: Res<Assets<TextureAtlas>>,
    fonts: Res<FontAssets>,
) {
    commands
        .spawn((
            NodeBundle {
                background_color: BackgroundColor(
                    Color::Hsla {
                        hue: 0.0,
                        saturation: 0.0,
                        lightness: 100.0,
                        alpha: 0.2,
                    },
                ),

                style: Style {
                    height: Val::Percent(100.),
                    width: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        background_color: BackgroundColor(
                            Color::Hsla {
                                hue: 0.0,
                                saturation: 0.0,
                                lightness: 100.0,
                                alpha: 0.4,
                            },
                        ),
                        style: Style {
                            width:     Val::Px(360.0),
                            height:     Val::Px(500.0),
                            flex_direction:
                                FlexDirection::Column,
                            justify_content:
                                JustifyContent::SpaceEvenly,
                            position_type:
                                PositionType::Absolute,
                            align_self: AlignSelf::Center,
                            border: UiRect::all(Val::Px(
                                10.0,
                            )),
                            ..default()
                        },
                        ..default()
                    },
                    MenuPage::Main,
                ))
                .with_children(|parent| {
                    button::spawn_button(
                        parent,
                        &fonts,
                        "New Game",
                    );
                    button::spawn_button(
                        parent,
                        &fonts,
                        "Settings",
                    );
                    button::spawn_button(
                        parent,
                        &fonts,
                        "Exit",
                    );
                });
            parent
                .spawn((
                    NodeBundle {
                        visibility: Visibility::Hidden,
                        background_color: BackgroundColor(
                            Color::Hsla {
                                hue: 0.0,
                                saturation: 0.0,
                                lightness: 100.0,
                                alpha: 0.4,
                            },
                        ),
                        style: Style {
                            width:     Val::Px(360.0),
                            height:     Val::Px(500.0),
                            flex_direction:
                                FlexDirection::Column,
                            justify_content:
                                JustifyContent::SpaceBetween,
                            border: UiRect::all(Val::Px(
                                10.0,
                            )),
                            ..default()
                        },
                        ..default()
                    },
                    MenuPage::Settings,
                ))
                .with_children(|parent| {
                    button::spawn_button(
                        parent,
                        &fonts,
                        "Back",
                    );
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width:     Val::Auto,
                                height:     Val::Px(25.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                ButtonBundle {
                                    style: Style {
                                        width:     Val::Px(25.0),
                                        height:     Val::Px(25.0),
                                        margin:
                                            UiRect::right(
                                                Val::Px(
                                                    10.0,
                                                ),
                                            ),
                                        ..default()
                                    },
                                    image: UiImage::new(
                                        images
                                            .box_checked
                                            .clone(),
                                    ),
                                    ..default()
                                },
                                AudioSettingsCheckbox,
                            ));
                            parent.spawn(
                                TextBundle::from_section(
                                    "Play Audio",
                                    TextStyle {
                                        font:fonts.roboto.clone(),
                                        font_size: 25.0,
                                        color: Color::rgb(
                                            0.0, 0.0, 0.0,
                                        ),
                                    },
                                ),
                            );
                        });

                    snake_selector::spawn_snake_selector(
                        parent,
                        images,
                        0,
                        &atlases,
                        &fonts,
                    );
                });
        });
}

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct HighScoreDisplay;

fn scoreboard(
    score: Res<Score>,
    high_score: Res<HighScore>,
    mut query_scores: Query<
        &mut Text,
        (
            With<ScoreDisplay>,
            Without<HighScoreDisplay>,
        ),
    >,
    mut query_high_scores: Query<
        &mut Text,
        (
            With<HighScoreDisplay>,
            Without<ScoreDisplay>,
        ),
    >,
    timer: ResMut<crate::scoring::Timer>,
) {
    let mut text = query_scores.single_mut();
    text.sections[1].value = score.score.to_string();

    let elapsed = timer
        .runtime
        .map(|duration| duration.as_secs())
        .or(timer
            .start
            .map(|start| start.elapsed().as_secs()))
        .unwrap_or(0);
    text.sections[4].value = elapsed.to_string();

    let mut text = query_high_scores.single_mut();
    text.sections[1].value = high_score.score.to_string();
    text.sections[4].value =
        high_score.time.as_secs().to_string();
}

pub fn playing_ui(
    mut commands: Commands,
    score: Res<Score>,
    high_score: Res<HighScore>,
    fonts: Res<FontAssets>,
) {
    let alfa_style = TextStyle {
        font: fonts.alfa_slab_one_regular.clone(),
        font_size: 25.0,
        color: Color::BLACK,
    };
    let roboto_style = TextStyle {
        font: fonts.roboto.clone(),
        font_size: 30.0,
        color: Color::BLACK,
    };
}
