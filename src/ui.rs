use crate::colors::{BUTTON_MATERIALS, MATERIALS};
use crate::common::{FontSpec, Game, RunState};
use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct BestScoreDisplay;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui)
            .add_system(scoreboard)
            .add_system(button_interaction_system)
            .add_system(button_text_system);
    }
}

fn setup_ui(
    mut commands: Commands,
    font_spec: Res<FontSpec>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(30.0*20.0), Val::Px(50.0)),
                align_items: AlignItems::FlexStart,
                margin: Rect { left: Val::Auto, right: Val::Auto, top: Val::Px(10.0), bottom: Val::Auto },
                ..Default::default()
            },
            color: UiColor(MATERIALS.none),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Snake",
                    TextStyle {
                        font: font_spec.family.clone(),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                    TextAlignment::default(),
                ),
                ..Default::default()
            });

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        size: Size::new(Val::Percent(100.0), Val::Auto),
                        ..Default::default()
                    },
                    color: UiColor(MATERIALS.none),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // scorebox
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::ColumnReverse,
                                align_items: AlignItems::Center,
                                margin: Rect {
                                    left: Val::Px(20.0),
                                    right: Val::Px(20.0),
                                    top: Val::Px(0.0),
                                    bottom: Val::Px(0.0),
                                },
                                padding: Rect::all(Val::Px(10.0)),
                                ..Default::default()
                            },
                            color: UiColor(MATERIALS.none),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    "Score",
                                    TextStyle {
                                        font: font_spec.family.clone(),
                                        font_size: 15.0,
                                        color: Color::WHITE,
                                    },
                                    TextAlignment {
                                        vertical: VerticalAlign::Center,
                                        horizontal: HorizontalAlign::Center,
                                    },
                                ),
                                ..Default::default()
                            });
                            parent
                                .spawn_bundle(TextBundle {
                                    text: Text::with_section(
                                        "<score>",
                                        TextStyle {
                                            font: font_spec.family.clone(),
                                            font_size: 20.0,
                                            color: Color::WHITE,
                                        },
                                        TextAlignment {
                                            vertical: VerticalAlign::Center,
                                            horizontal: HorizontalAlign::Center,
                                        },
                                    ),
                                    ..Default::default()
                                })
                                .insert(ScoreDisplay);
                        });
                    // end scorebox
                    // best scorebox
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::ColumnReverse,
                                align_items: AlignItems::Center,
                                padding: Rect::all(Val::Px(10.0)),
                                ..Default::default()
                            },
                            color: UiColor(MATERIALS.none),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    "Best",
                                    TextStyle {
                                        font: font_spec.family.clone(),
                                        font_size: 15.0,
                                        color: Color::WHITE,
                                    },
                                    TextAlignment {
                                        vertical: VerticalAlign::Center,
                                        horizontal: HorizontalAlign::Center,
                                    },
                                ),
                                ..Default::default()
                            });
                            parent
                                .spawn_bundle(TextBundle {
                                    text: Text::with_section(
                                        "<score>",
                                        TextStyle {
                                            font: font_spec.family.clone(),
                                            font_size: 20.0,
                                            color: Color::WHITE,
                                        },
                                        TextAlignment {
                                            vertical: VerticalAlign::Center,
                                            horizontal: HorizontalAlign::Center,
                                        },
                                    ),
                                    ..Default::default()
                                })
                                .insert(BestScoreDisplay);
                        });
                    // end best scorebox
                });
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(100.0), Val::Px(30.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Button",
                            TextStyle {
                                font: font_spec.family.clone(),
                                font_size: 20.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
        });
}

fn scoreboard(
    game: Res<Game>,
    mut query_scores: QuerySet<(
        QueryState<&mut Text, With<ScoreDisplay>>,
        QueryState<&mut Text, With<BestScoreDisplay>>,
    )>,
) {
    let mut q0 = query_scores.q0();
    let mut text = q0.single_mut();
    text.sections[0].value = game.score.to_string();

    let mut q1 = query_scores.q1();
    let mut text = q1.single_mut();
    text.sections[0].value = game.score_best.to_string();
}

fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut run_state: ResMut<State<RunState>>,
) {
    for (interaction, mut color) in
        interaction_query.iter_mut()
    {
        match interaction {
            Interaction::Clicked => {
                *color = BUTTON_MATERIALS.pressed.into();

                match run_state.current() {
                    RunState::Playing => {
                        run_state
                            .set(RunState::GameOver)
                            .unwrap();
                    }
                    RunState::GameOver => {
                        run_state
                            .set(RunState::Playing)
                            .unwrap();
                    }
                    RunState::Menu => todo!(),
                }
            }
            Interaction::Hovered => {
                *color = BUTTON_MATERIALS.hovered.into();
            }
            Interaction::None => {
                *color = BUTTON_MATERIALS.none.into();
            }
        }
    }
}

fn button_text_system(
    button_query: Query<&Children, With<Button>>,
    mut text_query: Query<&mut Text>,
    run_state: Res<State<RunState>>,
) {
    let children = button_query.single();
    let mut text =
        text_query
            .get_mut(*children.first().expect(
                "expect button to have a first child",
            ))
            .unwrap();
    match run_state.current() {
        RunState::Playing => {
            text.sections[0].value = "".to_string();
        }
        RunState::GameOver => {
            text.sections[0].value = "New Game".to_string();
        }
        RunState::Menu => todo!(),
    }
}
