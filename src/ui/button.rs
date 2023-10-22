use super::MenuPage;
use crate::{
    assets::AudioAssets,
    settings::{AudioSettings, GameSettings},
    GameState,
};
use bevy::{app::AppExit, prelude::*};
use bevy_kira_audio::{Audio, AudioControl};

const NORMAL_BUTTON: Color = Color::Hsla {
    hue: 0.0,
    saturation: 0.0,
    lightness: 0.0,
    alpha: 1.0,
};
const HOVERED_BUTTON: Color = Color::Hsla {
    hue: 0.0,
    saturation: 0.0,
    lightness: 0.25,
    alpha: 1.0,
};
const PRESSED_BUTTON: Color = Color::Hsla {
    hue: 0.0,
    saturation: 0.0,
    lightness: 0.0,
    alpha: 1.0,
};

pub fn text_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    text_query: Query<&Text>,
    mut exit: EventWriter<AppExit>,
    mut menu_page: ResMut<MenuPage>,
    settings: Res<GameSettings>,
    audio: Res<Audio>,
    sounds: Res<AudioAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, children) in
        &mut interaction_query
    {
        let text = text_query.get(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                if settings.audio == AudioSettings::ON {
                    audio.play(sounds.apple.clone());
                }
                *color = PRESSED_BUTTON.into();
                match text.sections[0].value.as_str() {
                    "New Game" => {
                        next_state.set(GameState::Playing);
                    }
                    "Settings" => {
                        *menu_page = MenuPage::Settings;
                        // Show Settings Page
                    }
                    "Exit" => {
                        exit.send(AppExit);
                    }
                    "Back" => {
                        *menu_page = MenuPage::Main;
                        // Show Main Menu Page
                    }
                    _ => {
                        unimplemented!(
                            "Button goes nowhere"
                        );
                    }
                }
            }
            Interaction::Hovered => {
                if settings.audio == AudioSettings::ON {
                    audio.play(sounds.menu_click.clone());
                }
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn spawn_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    text: &str,
) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(65.0),

                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font: asset_server
                        .load("AlfaSlabOne-Regular.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}
