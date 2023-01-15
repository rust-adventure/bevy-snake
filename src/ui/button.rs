//! This example illustrates how to create a button that changes color and text based on its
//! interaction state.

use bevy::{app::AppExit, prelude::*};
use iyes_loopless::state::NextState;

use crate::GameState;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn text_button_system(
    mut commands: Commands,
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
) {
    for (interaction, mut color, children) in
        &mut interaction_query
    {
        let text = text_query.get(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                match text.sections[0].value.as_str() {
                    "New Game" => {
                        commands.insert_resource(
                            NextState(GameState::Playing),
                        );
                    }
                    "Settings" => {
                        // Show Settings Page
                    }
                    "Exit" => {
                        exit.send(AppExit);
                    }
                    _ => {
                        unimplemented!(
                            "Button goes nowhere"
                        );
                    }
                }
            }
            Interaction::Hovered => {
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
                size: Size::new(
                    Val::Percent(100.0),
                    Val::Px(65.0),
                ),
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
                    font: asset_server.load("roboto.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}

// images.blue_button10.clone()
// } else {
// images.blue_button09.clone()
