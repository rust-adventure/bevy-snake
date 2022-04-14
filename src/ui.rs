use crate::{
    colors::{BUTTON_MATERIALS, MATERIALS},
    common::{Game, RunState},
};
use bevy::prelude::*;

// use bevy_ninepatch::{
//     NinePatchBuilder, NinePatchBundle,
// NinePatchData, };

mod kayak;
use kayak::*;
use kayak_ui::core::bind;

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
            .add_system(button_interaction_system)
            .add_system(button_text_system)
            .add_system(bind_gamestate)
            .add_system(bind_gamesettings);
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
    mut query_scores: QuerySet<(
        QueryState<&mut Text, With<ScoreDisplay>>,
        QueryState<&mut Text, With<BestScoreDisplay>>,
    )>,
) {
    let mut q0 = query_scores.q0();
    for mut text in q0.iter_mut() {
        text.sections[0].value = game.score.to_string();
    }

    let mut q1 = query_scores.q1();
    for mut text in q1.iter_mut() {
        text.sections[0].value =
            game.score_best.to_string();
    }
}

fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut Children),
        (Changed<Interaction>, With<Button>),
    >,
    // mut nine_patches: Query<(Entity, &mut Style)>,
    mut run_state: ResMut<State<RunState>>,
) {
    for (interaction, children) in
        interaction_query.iter_mut()
    {
        match interaction {
            Interaction::Clicked => {
                // *color = BUTTON_MATERIALS.pressed.into();

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
                // let mut patches =
                // children.iter();
                // if let (
                //     Some(first_nine_patch),
                //     Some(last_nine_patch),
                // ) = (patches.next(),
                // patches.next())
                // {
                //     if let Ok((_, mut style)) =
                // nine_patches
                //         .get_mut(*
                // first_nine_patch)
                //     {
                //         style.display =
                // Display::None;
                //     }
                //     if let Ok((_, mut style)) =
                // nine_patches
                //         .get_mut(*
                // last_nine_patch)
                //     {
                //         style.display =
                // Display::Flex;
                //     }
                // }
            }
            Interaction::None => {
                // let mut patches =
                // children.iter();
                // if let (
                //     Some(first_nine_patch),
                //     Some(last_nine_patch),
                // ) = (patches.next(),
                // patches.next())
                // {
                //     if let Ok((_, mut style)) =
                // nine_patches
                //         .get_mut(*
                // first_nine_patch)
                //     {
                //         style.display =
                // Display::Flex;
                //     }
                //     if let Ok((_, mut style)) =
                // nine_patches
                //         .get_mut(*
                // last_nine_patch)
                //     {
                //         style.display =
                // Display::None;
                //     }
                // }
            }
        }
    }
}

fn button_text_system(
    button_query: Query<&Children, With<Button>>,
    mut text_query: Query<&mut Text>,
    run_state: Res<State<RunState>>,
) {
    for children in button_query.iter() {
        match run_state.current() {
            RunState::Playing => {}
            RunState::GameOver => {}
            RunState::Menu => todo!(),
        }
    }
}

// fn new_game_ui(
//     mut commands: Commands,
//     font_spec: Res<FontSpec>,
//     asset_server: Res<AssetServer>,
//     mut nine_patches:
// ResMut<Assets<NinePatchBuilder<()>>>, ) {
//     // This entity will be placed in the center
// of the     // 9-Patch UI element
//     let exit_a = commands
//         .spawn_bundle(TextBundle {
//             transform: Transform::from_xyz(0.0,
// 0.0, 500.0),             style: Style {
//                 // size: Size::new(
//                 //     Val::Px(10.0),
//                 //     Val::Percent(50.0),
//                 // ),
//                 position: Rect {
//                     left: Val::Px(100.0),
//                     right: Val::Px(0.0),
//                     top: Val::Px(25.0),
//                     bottom: Val::Px(0.0),
//                 },
//                 // margin:
// Rect::all(Val::Px(10.0)),
// ..Default::default()             },
//             text: Text::with_section(
//                 "Exit",
//                 TextStyle {
//                     font:
// font_spec.family.clone(),
// font_size: 30.0,                     color:
// Color::rgb(0.9, 0.9, 0.9),                 },
//                 Default::default(),
//             ),
//             ..Default::default()
//         })
//         .id();
//     let exit_b = commands
//         .spawn_bundle(NodeBundle {
//             style: Style {
//                 size: Size::new(
//                     Val::Px(100.0),
//                     Val::Px(100.0),
//                 ),

//                 align_items:
// AlignItems::Center,
// justify_content: JustifyContent::Center,
//                 ..Default::default()
//             },
//             color:
// UiColor(MATERIALS.tile_placeholder_dark),
//             ..Default::default()
//         })
//         .with_children(|parent| {
//             parent.spawn_bundle(TextBundle {
//                 // node: Node {
//                 //     size: Vec2::new(100.0,
// 100.0),                 // },
//                 style: Style {
//                     size: Size::new(
//                         Val::Auto,
//                         Val::Percent(100.0),
//                     ),
//                     ..Default::default()
//                 },
//                 text: Text::with_section(
//                     "Exit",
//                     TextStyle {
//                         font:
// font_spec.family.clone(),
// font_size: 30.0,                         color:
// Color::rgb(0.9, 0.9, 0.9),
// },                     Default::default(),
//                 ),
//                 ..Default::default()
//             });
//         })
//         .id();

//     commands
//         .spawn_bundle(NodeBundle {
//             style: Style {
//                 size: Size::new(
//                     Val::Percent(100.0),
//                     Val::Percent(100.0),
//                 ),
//                 position_type:
// PositionType::Absolute,
// align_items: AlignItems::Center,
// justify_content: JustifyContent::Center,
//                 ..Default::default()
//             },
//             color: UiColor(MATERIALS.screen),
//             ..Default::default()
//         })
//         .with_children(|parent| {
//             parent
//                 .spawn_bundle(NodeBundle {
//                     style: Style {
//                         flex_direction:
//
// FlexDirection::ColumnReverse,
// align_items: AlignItems::Center,
// justify_content:
// JustifyContent::Center,
// padding: Rect {
// left: Val::Px(20.0),
// right: Val::Px(20.0),
// top: Val::Px(20.0),
// bottom: Val::Px(20.0),
// },                         size: Size::new(
//                             Val::Px(
//                                 30.0 * 20.0 /
// 5.0 * 3.0,                             ),
//                             Val::Px(30.0 * 20.0
// - 40.0),                         ),

//                         ..Default::default()
//                     },
//                     color: UiColor(
//
// MATERIALS.tile_placeholder_dark,
// ),                     ..Default::default()
//                 })
//                 .with_children(|parent| {
//
// parent.spawn_bundle(TextBundle {
// text: Text::with_section(
// "High Score",
// TextStyle {
// font: font_spec
// .family
// .clone(),
// font_size: 30.0,
// color: Color::WHITE,
// },
// TextAlignment::default(),
// ),                         ..Default::default()
//                     });
//                     parent
//                 .spawn_bundle(TextBundle {
//                     text: Text::with_section(
//                         "<score>",
//                         TextStyle {
//                             font:
// font_spec.family.clone(),
// font_size: 30.0,
// color: Color::WHITE,                         },
//                         TextAlignment {
//                             vertical:
// VerticalAlign::Center,
// horizontal:
// HorizontalAlign::Center,
// },                     ),
//                     ..Default::default()
//                 })
//                 .insert(BestScoreDisplay);

//
// parent.spawn_bundle(TextBundle {
// text: Text::with_section(
// "Last Score",
// TextStyle {
// font: font_spec
// .family
// .clone(),
// font_size: 30.0,
// color: Color::WHITE,
// },
// TextAlignment::default(),
// ),                         ..Default::default()
//                     });
//                     parent
//                 .spawn_bundle(TextBundle {
//                     text: Text::with_section(
//                         "<score>",
//                         TextStyle {
//                             font:
// font_spec.family.clone(),
// font_size: 30.0,
// color: Color::WHITE,                         },
//                         TextAlignment {
//                             vertical:
// VerticalAlign::Center,
// horizontal:
// HorizontalAlign::Center,
// },                     ),
//                     ..Default::default()
//                 })
//                 .insert(ScoreDisplay);

//                     // parent
//                     //
// .spawn_bundle(ButtonBundle {
// //         style: Style {
// //             size: Size::new(
// //                 Val::Percent(100.0),
//                     //
// Val::Px(30.0),                     //
// ),                     //
// justify_content:                     //
// JustifyContent::Center,                     //
// align_items:                     //
// AlignItems::Center,                     //
// ..Default::default()                     //
// },                     //
// ..Default::default()                     //
// })                     //
// .with_children(|parent| {
// //         parent                     //
// .spawn_bundle(TextBundle {
// //             text: Text::with_section(
//                     //                 "New
// Game",                     //
// TextStyle {                     //
// font: font_spec                     //
// .family                     //
// .clone(),                     //
// font_size: 20.0,                     //
// color: Color::rgb(                     //
// 0.9, 0.9, 0.9,                     //
// ),                     //                 },
//                     //
// Default::default(),                     //
// ),                     //
// ..Default::default()                     //
// });                     //     });
//                     // parent
//                     //
// .spawn_bundle(ButtonBundle {
// //         style: Style {
// //             size: Size::new(
// //                 Val::Px(100.0),
// //                 Val::Px(30.0),
// //             ),                     //
// justify_content:                     //
// JustifyContent::Center,                     //
// align_items:                     //
// AlignItems::Center,                     //
// ..Default::default()                     //
// },                     //
// ..Default::default()                     //
// })                     //
// .with_children(|parent| {
// //         parent                     //
// .spawn_bundle(TextBundle {
// //             text: Text::with_section(
//                     //
// "Settings",                     //
// TextStyle {                     //
// font: font_spec                     //
// .family                     //
// .clone(),                     //
// font_size: 20.0,                     //
// color: Color::rgb(                     //
// 0.9, 0.9, 0.9,                     //
// ),                     //                 },
//                     //
// Default::default(),                     //
// ),                     //
// ..Default::default()                     //
// });                     //     });
//                     //button
//                     button(
//                         parent,
//                         font_spec,
//                         asset_server,
//                         nine_patches,
//                         exit_a,
//                         exit_b,
//                     );
//                 });
//         });
// }

// fn button(
//     parent: &mut ChildBuilder,
//     // commands: &mut Commands,
//     font_spec: Res<FontSpec>,
//     asset_server: Res<AssetServer>,
//     mut nine_patches:
// ResMut<Assets<NinePatchBuilder<()>>>,
//     child_a: Entity,
//     child_b: Entity,
// ) {
//     // Texture for the base image
//     let panel_texture_handle =
//         asset_server.load("button.png");
//     let button_green =
//         asset_server.load("button_green.png");

//     // Create a basic 9-Patch UI element with
// margins     // of 20 pixels
//     let handle_one = nine_patches.add(
//         NinePatchBuilder::by_margins(36, 36,
// 36, 36),     );
//     let handle_two = nine_patches.add(
//         NinePatchBuilder::by_margins(36, 36,
// 36, 36),     );

//     parent
//         .spawn_bundle(ButtonBundle {
//             style: Style {
//                 size: Size::new(
//                     Val::Percent(100.0),
//                     Val::Px(72.0),
//                 ),
//                 justify_content:
// JustifyContent::Center,
// align_items: AlignItems::Center,
// ..Default::default()             },
//             color: UiColor(MATERIALS.none),
//             ..Default::default()
//         })
//         .with_children(|parent| {
//             parent.spawn_bundle(NinePatchBundle
// {                 style: Style {
//                     margin:
// Rect::all(Val::Auto),
// justify_content: JustifyContent::Center,
//                     align_items:
// AlignItems::Center,                     size:
// Size::new(
// Val::Percent(100.),
// Val::Percent(100.),                     ),
//                     ..Default::default()
//                 },
//                 nine_patch_data:
//
// NinePatchData::with_single_content(
// panel_texture_handle,
// handle_one.clone(),
// child_a,                     ),
//                 ..Default::default()
//             });
//             parent.spawn_bundle(NinePatchBundle
// {                 style: Style {
//                     margin:
// Rect::all(Val::Auto),
// justify_content: JustifyContent::Center,
//                     align_items:
// AlignItems::Center,                     size:
// Size::new(
// Val::Percent(100.),
// Val::Px(30.),                     ),
//                     display: Display::None,
//                     ..Default::default()
//                 },
//                 nine_patch_data:
//
// NinePatchData::with_single_content(
// button_green,
// handle_two,                         child_b,
//                     ),
//                 ..Default::default()
//             });
//         });
// }
