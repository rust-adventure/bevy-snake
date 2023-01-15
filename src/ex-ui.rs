use std::time::Duration;

use bevy::{
    app::AppExit,
    prelude::{
        AssetServer, Commands, Component, EventWriter,
        Plugin, Query, Res, ResMut, Without, World,
    },
};
use iyes_loopless::state::{CurrentState, NextState};
use kayak_ui::prelude::{widgets::*, *};

use crate::{
    assets::ImageAssets,
    scoring::{HighScore, Score, Timer},
    settings::GameSettings,
    ui::{
        button::{menu_button_render, MenuButton},
        checkbox::{
            checkbox_button_render, CheckboxButton,
        },
        settings::{settings_menu_render, SettingsMenu},
    },
    GameState, STARTING_GAME_STATE,
};

mod button;
mod checkbox;
mod mainmenu;
mod settings;
// mod snake_selector;
use mainmenu::*;
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(KayakContextPlugin)
            .add_plugin(KayakWidgets)
            .add_startup_system(game_ui)
            .add_system(on_game_state_change);
    }
}

// THIS ONLY RUNS ONCE. VERY IMPORTANT FACT.
pub fn game_ui(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
    settings: Res<GameSettings>,
    score: Res<Score>,
    high_score: Res<HighScore>,
) {
    let mut widget_context = KayakRootContext::new();
    widget_context.add_plugin(KayakWidgetsContextPlugin);

    font_mapping.set_default(
        // asset_server.load("roboto.kayak_font"),
        asset_server.load("roboto.kttf"),
    );

    let parent_id = None;

    // We need to register the prop and state types.
    // State is empty so you can use the `EmptyState`
    // component!
    widget_context.add_widget_data::<GameMenuProps, Menu>();

    // Next we need to add the systems
    widget_context.add_widget_system(
        // We are registering these systems with a specific
        // WidgetName.
        GameMenuProps::default().get_name(),
        // widget_update auto diffs props and state.
        // Optionally if you have context you can use:
        // widget_update_with_context otherwise you
        // will need to create your own widget update
        // system!
        widget_update::<GameMenuProps, Menu>,
        // Add our render system!
        game_menu_render,
    );

    widget_context
        .add_widget_data::<MenuButton, ButtonState>();
    widget_context.add_widget_system(
        MenuButton::default().get_name(),
        widget_update::<MenuButton, ButtonState>,
        menu_button_render,
    );

    widget_context
        .add_widget_data::<CheckboxButton, EmptyState>();
    widget_context.add_widget_system(
        CheckboxButton::default().get_name(),
        widget_update::<CheckboxButton, EmptyState>,
        checkbox_button_render,
    );

    widget_context
        .add_widget_data::<SettingsMenu, EmptyState>();
    widget_context.add_widget_system(
        SettingsMenu::default().get_name(),
        widget_update::<SettingsMenu, EmptyState>,
        settings_menu_render,
    );

    rsx! {
        <KayakAppBundle>
            <GameMenuBundle/>
        </KayakAppBundle>
    };

    commands.spawn(UICameraBundle::new(widget_context));
}
