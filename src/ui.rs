use bevy::{
    app::AppExit,
    prelude::{
        AssetServer, Commands, EventWriter, Plugin, Res,
        ResMut, World,
    },
};
use iyes_loopless::state::{CurrentState, NextState};
use kayak_ui::{
    bevy::{
        BevyContext, BevyKayakUIPlugin, FontMapping,
        ImageManager, UICameraBundle,
    },
    core::{
        bind, render, rsx,
        styles::{
            Corner, Edge, LayoutType, Style, StyleProp,
            Units,
        },
        use_state, widget, Binding, Bound, Color,
        EventType, Handler, Index, MutableBound, OnEvent,
    },
    widgets::{App, If, NinePatch, Text},
};

use crate::{
    assets::ImageAssets, GameState, STARTING_GAME_STATE,
};

mod button;
mod settings;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(BevyKayakUIPlugin)
            .insert_resource(bind(STARTING_GAME_STATE))
            .add_startup_system(game_ui)
            .add_system(bind_gamestate);
    }
}

pub fn bind_gamestate(
    state: Res<CurrentState<GameState>>,
    binding: Res<Binding<GameState>>,
) {
    if state.is_changed() {
        binding.set(state.0);
    }
}

// THIS ONLY RUNS ONCE. VERY IMPORTANT FACT.
fn game_ui(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UICameraBundle::new());
    font_mapping.set_default(
        asset_server.load("roboto.kayak_font"),
    );

    let context = BevyContext::new(|context| {
        render! {
            <App>
                <GameMenu/>
            </App>
        }
    });

    commands.insert_resource(context);
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Menu {
    Main,
    Settings,
}

#[widget]
fn GameMenu() {
    let container_styles = Style {
        border_radius: StyleProp::Value(Corner::all(15.0)),
        background_color: StyleProp::Value(Color::WHITE),
        bottom: StyleProp::Value(Units::Stretch(1.0)),
        height: StyleProp::Value(Units::Pixels(500.0)),
        layout_type: StyleProp::Value(LayoutType::Column),
        left: StyleProp::Value(Units::Stretch(1.0)),
        padding: StyleProp::Value(Edge::all(
            Units::Stretch(1.0),
        )),
        right: StyleProp::Value(Units::Stretch(1.0)),
        row_between: StyleProp::Value(Units::Pixels(20.0)),
        top: StyleProp::Value(Units::Stretch(1.0)),
        width: StyleProp::Value(Units::Pixels(360.0)),
        ..Default::default()
    };

    let (menu_state, set_menu_state, ..) =
        use_state!(Menu::Main);

    let set_menu = set_menu_state.clone();
    let set_menu_to_main = Handler::new(move |_| {
        set_menu(Menu::Main);
    });

    let show_menus = {
        let gamestate = context
            .query_world::<Res<Binding<GameState>>, _, _>(
                |state| state.clone(),
            );

        context.bind(&gamestate);
        gamestate.get() == GameState::Menu
    };

    let green_panel = context
        .query_world::<Res<ImageAssets>, _, _>(|assets| {
            assets.green_panel.clone()
        });

    let container = context
        .get_global_mut::<World>()
        .map(|mut world| {
            world
                .get_resource_mut::<ImageManager>()
                .unwrap()
                .get(&green_panel)
        })
        .unwrap();

    let on_click_new_game =
        OnEvent::new(|ctx, event| match event.event_type {
            EventType::Click(..) => {
                let mut world =
                    ctx.get_global_mut::<World>().unwrap();
                world.insert_resource(NextState(
                    GameState::Playing,
                ));
            }
            _ => {}
        });

    let set_menu = set_menu_state.clone();
    let on_click_settings =
        OnEvent::new(move |_, event| {
            match event.event_type {
                EventType::Click(..) => {
                    set_menu(Menu::Settings);
                }
                _ => {}
            }
        });

    let on_click_exit =
        OnEvent::new(|ctx, event| match event.event_type {
            EventType::Click(..) => {
                ctx
                .query_world::<EventWriter<AppExit>, _, _>(
                    |mut exit| {
                        exit.send(AppExit);
                    },
                );
            }
            _ => {}
        });

    let show_main_menu = menu_state == Menu::Main;
    let show_settings_menu = menu_state == Menu::Settings;

    rsx! {
    <If condition={show_menus}>
       <If condition={show_main_menu}>
       <NinePatch
         styles={Some(container_styles)}
         border={Edge::all(10.0)}
         handle={container}
       >
           <button::SnakeButton
             on_event={Some(on_click_new_game)}
            >
               <Text
                   size={20.0}
                   content={"New Game".to_string()}
               />
           </button::SnakeButton>
           <button::SnakeButton
             on_event={Some(on_click_settings)}
            >
               <Text
                   size={20.0}
                   content={"Settings".to_string()}
               />
           </button::SnakeButton>
           <button::SnakeButton
             on_event={Some(on_click_exit)}
            >
               <Text
                   size={20.0}
                   content={"Exit".to_string()}
               />
           </button::SnakeButton>
       </NinePatch>
       </If>
       <If condition={show_settings_menu}>
         <settings::SettingsMenu
             back={set_menu_to_main}
         />
       </If>
    </If>
    }
}
