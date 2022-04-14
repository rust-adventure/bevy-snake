use bevy::{app::AppExit, prelude::*};
use kayak_ui::{
    bevy::{
        BevyContext, BevyKayakUIPlugin, FontMapping,
        ImageManager, UICameraBundle,
    },
    core::{bind, context, KayakContextRef},
};
use kayak_ui::{
    core::Binding,
    widgets::{App, NinePatch, Text},
};
use kayak_ui::{
    core::{
        constructor, render, rsx,
        styles::{
            Edge, LayoutType, Style, StyleProp, Units,
        },
        use_state, widget, Bound, EventType, Handler,
        Index, MutableBound, OnEvent, VecTracker,
        WidgetProps,
    },
    widgets::If,
};

use crate::{common::RunState, settings::GameSettings};

pub fn bind_gamestate(
    state: Res<State<RunState>>,
    binding: Res<Binding<RunState>>,
) {
    // bevy change detection for states is accurate,
    // but feels buggy. It is always true for State<>.
    // https://github.com/bevyengine/bevy/issues/2343
    // if state.is_changed() {
    binding.set(state.current().clone());
    // }
}

pub fn bind_gamesettings(
    settings: Res<GameSettings>,
    binding: Res<Binding<GameSettings>>,
) {
    if settings.is_changed() {
        dbg!(&settings);
        binding.set(settings.clone());
    }
}

// THIS ONLY RUNS ONCE. VERY IMPORTANT FACT.
pub fn new_game_ui_kayak(
    mut font_mapping: ResMut<FontMapping>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    runstate: Res<State<RunState>>,
    settings: Res<GameSettings>,
) {
    commands.spawn_bundle(UICameraBundle::new());
    commands
        .insert_resource(bind(runstate.current().clone()));
    commands.insert_resource(bind(settings.clone()));

    let roboto_font =
        asset_server.load("roboto.kayak_font");
    font_mapping.add("Roboto", roboto_font.clone());

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
    let (menu_state, set_menu_state, ..) =
        use_state!(Menu::Main);

    let show_menus = {
        let runstate = context
            .query_world::<Res<Binding<RunState>>, _, _>(
                move |state| state.clone(),
            );

        context.bind(&runstate);
        match runstate.get() {
            RunState::Menu => true,
            _ => false,
        }
    };

    let settings = {
        let settings = context
        .query_world::<Res<Binding<GameSettings>>, _, _>(
            move |settings| settings.clone(),
        );

        context.bind(&settings);
        settings.get()
    };

    let (container, roboto) = {
        let world = context.get_global_mut::<World>();
        if world.is_err() {
            return;
        }

        let mut world = world.unwrap();
        let (roboto_font, handle) = {
            let asset_server = world
                .get_resource::<AssetServer>()
                .unwrap();
            let roboto_font =
                asset_server.load("roboto.kayak_font");
            let handle: Handle<
                bevy::render::texture::Image,
            > = asset_server.load("green_panel.png");
            (roboto_font, handle)
        };

        let container = {
            let mut image_manager = world
                .get_resource_mut::<ImageManager>()
                .unwrap();
            image_manager.get(&handle)
        };

        let font_mapping =
            world.get_resource::<FontMapping>().unwrap();
        let roboto = font_mapping.get(&roboto_font);

        (container, roboto)
    };

    let nine_patch_styles = Style {
        width: StyleProp::Value(Units::Pixels(360.0)),
        height: StyleProp::Value(Units::Pixels(500.0)),
        layout_type: StyleProp::Value(LayoutType::Column),
        left: StyleProp::Value(Units::Stretch(1.0)),
        right: StyleProp::Value(Units::Stretch(1.0)),
        top: StyleProp::Value(Units::Stretch(1.0)),
        bottom: StyleProp::Value(Units::Stretch(1.0)),
        padding: StyleProp::Value(Edge::all(
            Units::Stretch(1.0),
        )),
        row_between: StyleProp::Value(Units::Pixels(20.0)),
        ..Style::default()
    };

    let set_menu = set_menu_state.clone();
    let on_click =
        OnEvent::new(move |_, event| {
            match event.event_type {
                EventType::Click(..) => {
                    set_menu(Menu::Settings)
                }
                _ => {}
            }
        });

    let set_menu = set_menu_state.clone();
    let on_click_back = OnEvent::new(move |_, event| {
        match event.event_type {
            EventType::Click(..) => set_menu(Menu::Main),
            _ => {}
        }
    });

    let on_click_checkbox = OnEvent::new(
        move |context, event| match event.event_type {
            EventType::Click(..) => {
                context.query_world::<ResMut<GameSettings>, _, _>(
            |mut settings| {
                settings.speedrun_mode = !settings.speedrun_mode;
            },
        );
            }
            _ => {}
        },
    );

    let show_main = menu_state == Menu::Main;
    let show_settings = menu_state == Menu::Settings;

    rsx! {
       <If condition={show_menus}>
       <NinePatch
               styles={Some(nine_patch_styles)}
               border={Edge::all(50.0)}
               handle={container}
           >

           <If condition={show_main}>
                <NewGameButton/>
                <BlueButton on_click={Some(on_click)}>
                    <Text line_height={Some(50.0)} size={20.0} content={"Settings".to_string()} font={roboto} />
                </BlueButton>
                <QuitButton/>
           </If>

           <If condition={show_settings}>
                <BlueButton on_click={Some(on_click_back)}>
                    <Text line_height={Some(50.0)} size={20.0} content={"Back".to_string()} font={roboto} />
                </BlueButton>
                <Text line_height={Some(50.0)} size={20.0} content={"Speedrun Mode!".to_string()} font={roboto} />
                <Checkbox checked={settings.speedrun_mode} on_click={Some(on_click_checkbox)}/>
           </If>
       </NinePatch>

       </If>
    }
}

#[widget]
fn NewGameButton() {
    // let roboto_font =
    //     asset_server.load("roboto.kayak_font");
    // let roboto = font_mapping.get(&roboto_font);

    let on_click = OnEvent::new(move |context, event| {
        match event.event_type {
            EventType::Click(..) => {
                context
            .query_world::<ResMut<State<RunState>>, _, _>(
                |mut state| {
                    state.set(RunState::Playing).unwrap();
                },
            );
            }
            _ => {}
        }
    });
    rsx! {
    <BlueButton on_click={Some(on_click)}>
        <Text line_height={Some(50.0)} size={20.0} content={"Play".to_string()} />
    </BlueButton>
    }
}

#[widget]
fn QuitButton() {
    let on_click = OnEvent::new(move |context, event| {
        match event.event_type {
            EventType::Click(..) => {
                context.query_world::<EventWriter<AppExit>, _, _>(
            |mut exit| {
                exit.send(AppExit);
            },
        );
            }
            _ => {}
        }
    });
    rsx! {
    <BlueButton on_click={Some(on_click)}>
        <Text line_height={Some(50.0)} size={20.0} content={"Exit".to_string()} />
    </BlueButton>
    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
struct BlueButtonProps {
    #[prop_field(Styles)]
    styles: Option<Style>,
    #[prop_field(OnEvent)]
    on_click: Option<OnEvent>,
    #[prop_field(Children)]
    children: Option<kayak_ui::core::Children>,
}
#[widget]
fn BlueButton(props: BlueButtonProps) {
    let (blue_button_handle, blue_button_hover_handle) = {
        let world = context.get_global_mut::<World>();
        if world.is_err() {
            return;
        }

        let mut world = world.unwrap();

        let (handle1, handle2) = {
            let asset_server = world
                .get_resource::<AssetServer>()
                .unwrap();
            let handle1: Handle<
                bevy::render::texture::Image,
            > = asset_server.load("blue_button09.png");
            let handle2: Handle<
                bevy::render::texture::Image,
            > = asset_server.load("blue_button10.png");

            (handle1, handle2)
        };

        let mut image_manager = world
            .get_resource_mut::<ImageManager>()
            .unwrap();
        let blue_button_handle =
            image_manager.get(&handle1);
        let blue_button_hover_handle =
            image_manager.get(&handle2);

        (
            blue_button_handle,
            blue_button_hover_handle,
        )
    };

    let current_button_handle = context
        .create_state::<u16>(blue_button_handle)
        .unwrap();

    let button_styles = Style {
        width: StyleProp::Value(Units::Pixels(200.0)),
        height: StyleProp::Value(Units::Pixels(50.0)),
        padding: StyleProp::Value(Edge::all(
            Units::Stretch(1.0),
        )),
        ..props.styles.clone().unwrap_or_default()
    };

    let cloned_current_button_handle =
        current_button_handle.clone();
    let on_click = props.on_click.clone();
    let on_event = OnEvent::new(move |ctx, event| {
        match event.event_type {
            EventType::MouseDown(..) => {
                cloned_current_button_handle
                    .set(blue_button_hover_handle);
            }
            EventType::MouseUp(..) => {
                cloned_current_button_handle
                    .set(blue_button_handle);
            }
            EventType::Click(..) => {
                match &on_click {
                    Some(v) => v.try_call(ctx, event),
                    None => todo!(),
                };
            }
            _ => (),
        }
    });

    let children = props.get_children();
    rsx! {
        <NinePatch
            border={Edge::all(24.0)}
            handle={current_button_handle.get()}
            styles={Some(button_styles)}
            on_event={Some(on_event)}
        >
            {children}
        </NinePatch>
    }
}

#[widget]
fn Speedrun() {
    let (roboto, container) = {
        let world = context.get_global_mut::<World>();
        if world.is_err() {
            return;
        }
        let mut world = world.unwrap();

        let asset_server =
            world.get_resource::<AssetServer>().unwrap();
        let handle: Handle<bevy::render::texture::Image> =
            asset_server.load("green_panel.png");

        let font_mapping =
            world.get_resource::<FontMapping>().unwrap();
        let roboto_font =
            asset_server.load("roboto.kayak_font");
        let roboto = font_mapping.get(&roboto_font);

        let mut image_manager = world
            .get_resource_mut::<ImageManager>()
            .unwrap();
        let container = image_manager.get(&handle);

        (roboto, container)
    };

    let nine_patch_styles = Style {
        width: StyleProp::Value(Units::Pixels(100.0)),
        height: StyleProp::Value(Units::Pixels(
            500.0 / 10.0,
        )),
        layout_type: StyleProp::Value(LayoutType::Column),
        left: StyleProp::Value(Units::Stretch(1.0)),
        right: StyleProp::Value(Units::Stretch(1.0)),
        top: StyleProp::Value(Units::Stretch(1.0)),
        bottom: StyleProp::Value(Units::Stretch(1.0)),
        padding: StyleProp::Value(Edge::all(
            Units::Stretch(1.0),
        )),
        row_between: StyleProp::Value(Units::Pixels(20.0)),
        ..Style::default()
    };

    rsx! {
        <NinePatch
                styles={Some(nine_patch_styles)}
                border={Edge::all(50.0)}
                handle={container}
            >

                <Text line_height={Some(50.0)} size={20.0} content={"Score".to_string()} font={roboto} />
                <Text line_height={Some(50.0)} size={20.0} content={"Time".to_string()} font={roboto} />
        </NinePatch>

    }
}

#[widget]
fn Speedruns() {
    let data = vec![
        "Text 1", "Text 2", "Text 3", "Text 4", "Text 5",
        "Text 6", "Text 7", "Text 8", "Text 9", "Text 10",
    ];
    rsx! {
        <>
            {VecTracker::from(data.iter().map(|data| {
                constructor! {
                    <Text content={data.clone().to_string()} size={16.0} />
                }
            }))}
        </>
    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
struct CheckboxProps {
    checked: bool,
    #[prop_field(Styles)]
    styles: Option<Style>,
    #[prop_field(OnEvent)]
    on_click: Option<OnEvent>,
}
#[widget]
fn Checkbox(props: CheckboxProps) {
    let (empty_box, checked_box) = {
        let world = context.get_global_mut::<World>();
        if world.is_err() {
            return;
        }

        let mut world = world.unwrap();

        let (handle1, handle2) = {
            let asset_server = world
                .get_resource::<AssetServer>()
                .unwrap();
            let handle1: Handle<
                bevy::render::texture::Image,
            > = asset_server.load("grey_box.png");
            let handle2: Handle<
                bevy::render::texture::Image,
            > = asset_server.load("green_boxCheckmark.png");

            (handle1, handle2)
        };

        let mut image_manager = world
            .get_resource_mut::<ImageManager>()
            .unwrap();
        let empty_box = image_manager.get(&handle1);
        let checked_box = image_manager.get(&handle2);

        (empty_box, checked_box)
    };

    let current_button_handle =
        context.create_state::<u16>(empty_box).unwrap();

    let button_styles = Style {
        width: StyleProp::Value(Units::Pixels(50.0)),
        height: StyleProp::Value(Units::Pixels(50.0)),
        padding: StyleProp::Value(Edge::all(
            Units::Stretch(1.0),
        )),
        ..props.styles.clone().unwrap_or_default()
    };

    let cloned_current_button_handle =
        current_button_handle.clone();
    let on_click = props.on_click.clone();
    let on_event = OnEvent::new(move |ctx, event| {
        match event.event_type {
            EventType::Click(..) => {
                match &on_click {
                    Some(v) => {
                        v.try_call(ctx, event);
                    }
                    None => {}
                };
            }
            _ => (),
        }
    });
    if props.checked {
        cloned_current_button_handle.set(checked_box);
    } else {
        cloned_current_button_handle.set(empty_box);
    }

    rsx! {
        <NinePatch
            border={Edge::all(15.0)}
            handle={current_button_handle.get()}
            styles={Some(button_styles)}
            on_event={Some(on_event)}
        />
    }
}
