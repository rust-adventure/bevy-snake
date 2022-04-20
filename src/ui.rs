use crate::{colors::MATERIALS, common::Game};
use bevy::prelude::*;

mod kayak;
use kayak::*;

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct BestScoreDisplay;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FontSpec>()
            .add_startup_system(setup_ui)
            .add_startup_system(new_game_ui_kayak)
            .add_system(scoreboard)
            .add_system(bind_gamestate)
            .add_system(bind_gamesettings)
            .add_system(bind_speedruns);
    }
}

pub struct FontSpec {
    pub family: Handle<Font>,
}

impl FromWorld for FontSpec {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world
            .get_resource_mut::<AssetServer>()
            .unwrap();
        FontSpec {
            family: asset_server
                .load("fonts/FiraSans-Bold.ttf"),
        }
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
        });
}

fn scoreboard(
    game: Res<Game>,
    mut query_score: Query<
        &mut Text,
        (
            With<ScoreDisplay>,
            Without<BestScoreDisplay>,
        ),
    >,
    mut query_high_score: Query<
        &mut Text,
        (
            With<BestScoreDisplay>,
            Without<ScoreDisplay>,
        ),
    >,
) {
    for mut text in query_score.iter_mut() {
        text.sections[0].value = game.score.to_string();
    }

    for mut text in query_high_score.iter_mut() {
        text.sections[0].value =
            game.score_best.to_string();
    }
}
