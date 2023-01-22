use crate::{
    assets::{AudioAssets, ImageAssets},
    scoring::{HighScore, Score},
    settings::{AudioSettings, GameSettings},
    GameState,
};
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use iyes_loopless::prelude::{
    AppLooplessStateExt, IntoConditionalSystem,
};

use self::snake_selector::{
    snake_selector_interaction, update_current_snake,
};

mod button;
mod snake_selector;
// mod checkbox;
// mod mainmenu;
// mod settings;
// mod snake_selector;
// use mainmenu::*;
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system_to_stage(
            StartupStage::PostStartup,
            game_ui,
        )
        .add_system(button::text_button_system)
        .insert_resource(MenuPage::Main)
        .add_enter_system(GameState::Menu, show_menu)
        .add_enter_system(GameState::Playing, hide_menu)
        .add_system(
            change_menu.run_in_state(GameState::Menu),
        )
        .add_system(
            audio_state.run_in_state(GameState::Menu),
        )
        .add_system(
            snake_selector_interaction
                .run_in_state(GameState::Menu),
        )
        .add_system(
            update_current_snake
                .run_in_state(GameState::Menu),
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
    *menu = Visibility::VISIBLE;
}
fn hide_menu(
    mut menu: Query<&mut Visibility, With<MainMenu>>,
) {
    let mut menu = menu.single_mut();
    *menu = Visibility::INVISIBLE;
}
fn change_menu(
    menu: Res<MenuPage>,
    mut menu_pages: Query<(&MenuPage, &mut Visibility)>,
) {
    if menu.is_changed() {
        for (page, mut visibility) in menu_pages.iter_mut()
        {
            if page == &*menu {
                *visibility = Visibility::VISIBLE;
            } else {
                *visibility = Visibility::INVISIBLE;
            }
        }
    }
}

fn audio_state(
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
    audio: Res<Audio>,
    sounds: Res<AudioAssets>,
) {
    for (interaction, mut image) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if settings.audio == AudioSettings::ON {
                    audio.play(sounds.apple.clone());
                }
                settings.audio = match settings.audio {
                    AudioSettings::ON => AudioSettings::OFF,
                    AudioSettings::OFF => AudioSettings::ON,
                };
                *image = UiImage(match settings.audio {
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

pub fn game_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<GameSettings>,
    score: Res<Score>,
    high_score: Res<HighScore>,
    images: Res<ImageAssets>,
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
                    size: Size::new(
                        Val::Percent(100.0),
                        Val::Percent(100.0),
                    ),
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
                            size: Size::new(
                                Val::Px(360.0),
                                Val::Px(500.0),
                            ),
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
                        &asset_server,
                        "New Game",
                    );
                    button::spawn_button(
                        parent,
                        &asset_server,
                        "Settings",
                    );
                    button::spawn_button(
                        parent,
                        &asset_server,
                        "Exit",
                    );
                });
            parent
                .spawn((
                    NodeBundle {
                        visibility: Visibility::INVISIBLE,
                        background_color: BackgroundColor(
                            Color::Hsla {
                                hue: 0.0,
                                saturation: 0.0,
                                lightness: 100.0,
                                alpha: 0.4,
                            },
                        ),
                        style: Style {
                            size: Size::new(
                                Val::Px(360.0),
                                Val::Px(500.0),
                            ),
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
                        &asset_server,
                        "Back",
                    );
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(
                                    Val::Auto,
                                    Val::Px(25.0),
                                ),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                ButtonBundle {
                                    style: Style {
                                        size: Size::new(
                                            Val::Px(25.0),
                                            Val::Px(25.0),
                                        ),
                                        margin:
                                            UiRect::right(
                                                Val::Px(
                                                    10.0,
                                                ),
                                            ),
                                        ..default()
                                    },
                                    image: UiImage(
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
                                        font: asset_server
                                            .load(
                                            "roboto.ttf",
                                        ),
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
                        asset_server,
                        0,
                    );
                });
        });
}
