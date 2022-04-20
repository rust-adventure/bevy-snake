use bevy::{app::AppExit, prelude::*};
use kayak_ui::{
    bevy::{
        BevyContext, FontMapping, ImageManager,
        UICameraBundle,
    },
    core::{
        bind, constructor, render, rsx,
        styles::{
            Edge, LayoutType, PositionType, Style,
            StyleProp, Units,
        },
        use_state, widget, Binding, Bound, EventType,
        Index, MutableBound, OnEvent, VecTracker,
        WidgetProps,
    },
    widgets::{App, Element, If, NinePatch, Text},
};

use crate::{
    common::RunState, scoring::Speedruns,
    settings::GameSettings, snake::SnakeTextureSelection,
};

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

pub fn bind_speedruns(
    runs: Res<Speedruns>,
    binding: Res<Binding<Speedruns>>,
) {
    if runs.is_changed() {
        binding.set(runs.clone());
    }
}

// THIS ONLY RUNS ONCE. VERY IMPORTANT FACT.
pub fn new_game_ui_kayak(
    mut font_mapping: ResMut<FontMapping>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    runstate: Res<State<RunState>>,
    settings: Res<GameSettings>,
    runs: Res<Speedruns>,
) {
    commands.spawn_bundle(UICameraBundle::new());
    commands
        .insert_resource(bind(runstate.current().clone()));
    commands.insert_resource(bind(settings.clone()));
    commands.insert_resource(bind(runs.clone()));

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
    SpeedRuns,
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

    let container = {
        let mut world =
            context.get_global_mut::<World>().unwrap();
        let asset_server =
            world.get_resource::<AssetServer>().unwrap();
        let handle: Handle<bevy::render::texture::Image> =
            asset_server.load("green_panel.png");

        let mut image_manager = world
            .get_resource_mut::<ImageManager>()
            .unwrap();
        image_manager.get(&handle)
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

    let set_menu = set_menu_state.clone();
    let on_click_runs = OnEvent::new(move |_, event| {
        match event.event_type {
            EventType::Click(..) => {
                set_menu(Menu::SpeedRuns)
            }
            _ => {}
        }
    });

    let show_main = menu_state == Menu::Main;
    let show_settings = menu_state == Menu::Settings;
    let show_speedruns_page = menu_state == Menu::SpeedRuns;
    let show_speedruns = settings.speedrun_mode;

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
                    <Text line_height={Some(50.0)} size={20.0} content={"Settings".to_string()}/>
                </BlueButton>
                <If condition={show_speedruns}>
                    <BlueButton on_click={Some(on_click_runs)}>
                        <Text line_height={Some(50.0)} size={20.0} content={"Show Runs".to_string()}/>
                    </BlueButton>
                </If>
                <QuitButton/>
           </If>

           <If condition={show_settings}>
                <BlueButton on_click={Some(on_click_back)}>
                    <Text line_height={Some(50.0)} size={20.0} content={"Back".to_string()}/>
                </BlueButton>
                <Text line_height={Some(50.0)} size={20.0} content={"Speedrun!".to_string()}/>
                <Checkbox checked={settings.speedrun_mode} on_click={Some(on_click_checkbox)}/>
                <SnakeSelector/>
           </If>

           <If condition={show_speedruns_page}>
                <BlueButton on_click={Some(on_click_back)}>
                    <Text line_height={Some(50.0)} size={20.0} content={"Back".to_string()}/>
                </BlueButton>
                <Text line_height={Some(50.0)} size={20.0} content={"Best Runs".to_string()}/>
                <SpeedrunsDisplay/>
           </If>
       </NinePatch>

       </If>
    }
}

#[widget]
fn NewGameButton() {
    let on_click = OnEvent::new(|context, event| {
        if let EventType::Click(..) = event.event_type {
            context
            .query_world::<ResMut<State<RunState>>, _, _>(
                |mut state| {
                    state.set(RunState::Playing).unwrap();
                },
            );
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
        if let EventType::Click(..) = event.event_type {
            context
                .query_world::<EventWriter<AppExit>, _, _>(
                    |mut exit| {
                        exit.send(AppExit);
                    },
                );
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
    let container = {
        let world = context.get_global_mut::<World>();
        if world.is_err() {
            return;
        }
        let mut world = world.unwrap();

        let asset_server =
            world.get_resource::<AssetServer>().unwrap();
        let handle: Handle<bevy::render::texture::Image> =
            asset_server.load("green_panel.png");

        let mut image_manager = world
            .get_resource_mut::<ImageManager>()
            .unwrap();
        let container = image_manager.get(&handle);

        container
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

                <Text line_height={Some(50.0)} size={20.0} content={"Score".to_string()} />
                <Text line_height={Some(50.0)} size={20.0} content={"Time".to_string()} />
        </NinePatch>

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
        let mut world =
            context.get_global_mut::<World>().unwrap();
        let asset_server =
            world.get_resource::<AssetServer>().unwrap();
        let handle1 = asset_server.load("grey_box.png");
        let handle2 =
            asset_server.load("green_boxCheckmark.png");

        let mut image_manager = world
            .get_resource_mut::<ImageManager>()
            .unwrap();
        let empty_box = image_manager.get(&handle1);
        let checked_box = image_manager.get(&handle2);

        (empty_box, checked_box)
    };

    let button_styles = Style {
        position_type: StyleProp::Value(
            PositionType::SelfDirected,
        ),
        width: StyleProp::Value(Units::Pixels(25.0)),
        height: StyleProp::Value(Units::Pixels(25.0)),
        ..Default::default()
    };

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
    let image = if props.checked {
        checked_box
    } else {
        empty_box
    };
    rsx! {
        <NinePatch
            on_event={Some(on_event)}
            border={Edge::all(1.0)}
            styles={Some(button_styles)}
            handle={image}
        />
    }
}

#[widget]
fn SnakeSelector() {
    let container: Vec<(usize, u16)> = (1..31)
        .into_iter()
        .map(|num| {
            let world = context.get_global_mut::<World>();
            if world.is_err() {
                panic!("whoops");
            }

            let mut world = world.unwrap();

            let asset_server = world
                .get_resource::<AssetServer>()
                .unwrap();

            let handle: Handle<
                bevy::render::texture::Image,
            > = asset_server.load(&format!(
                "snake_heads/snake_sprites_{:02}.png",
                num
            ));

            let mut image_manager = world
                .get_resource_mut::<ImageManager>()
                .unwrap();
            let image = image_manager.get(&handle);
            image
        })
        .enumerate()
        .collect();

    let row_styles = Style {
        col_between: StyleProp::Value(Units::Pixels(5.0)),
        layout_type: StyleProp::Value(LayoutType::Row),
        ..Default::default()
    };
    let snake_container_styles = Style {
        left: StyleProp::Value(Units::Pixels(5.0)),
        row_between: StyleProp::Value(Units::Pixels(35.0)),
        layout_type: StyleProp::Value(LayoutType::Column),
        border: StyleProp::Value(Edge::all(10.0)),
        padding_left: StyleProp::Value(Units::Stretch(1.0)),
        padding_right: StyleProp::Value(Units::Stretch(
            1.0,
        )),
        ..Default::default()
    };

    let one = container[0].clone();
    let two = container[1].clone();
    let three = container[2].clone();
    let four = container[3].clone();
    let five = container[4].clone();

    let six = container[5].clone();
    let seven = container[6].clone();
    let eight = container[7].clone();
    let nine = container[8].clone();
    let ten = container[9].clone();

    let eleven = container[10].clone();
    let twelve = container[11].clone();
    let thirteen = container[12].clone();
    let fourteen = container[13].clone();
    let fifteen = container[14].clone();

    let sixteen = container[15].clone();
    let seventeen = container[16].clone();
    let eighteen = container[17].clone();
    let nineteen = container[18].clone();
    let twenty = container[19].clone();

    let twentyone = container[20].clone();
    let twentytwo = container[21].clone();
    let twentythree = container[22].clone();
    let twentyfour = container[23].clone();
    let twentyfive = container[24].clone();

    let twentysix = container[25].clone();
    let twentyseven = container[26].clone();
    let twentyeight = container[27].clone();
    let twentynine = container[28].clone();
    let thirty = container[29].clone();
    rsx! {
    <Element styles={Some(snake_container_styles)}>
        <Element styles={Some(row_styles)}>
            <SnakeHead handle={one}/>
            <SnakeHead handle={two}/>
            <SnakeHead handle={three}/>
            <SnakeHead handle={four}/>
            <SnakeHead handle={five}/>
            <SnakeHead handle={six}/>
        </Element>
        <Element styles={Some(row_styles)}>
            <SnakeHead handle={seven}/>
            <SnakeHead handle={eight}/>
            <SnakeHead handle={nine}/>
            <SnakeHead handle={ten}/>
            <SnakeHead handle={eleven}/>
            <SnakeHead handle={twelve}/>
        </Element>
        <Element styles={Some(row_styles)}>
            <SnakeHead handle={thirteen}/>
            <SnakeHead handle={fourteen}/>
            <SnakeHead handle={fifteen}/>
            <SnakeHead handle={sixteen}/>
            <SnakeHead handle={seventeen}/>
            <SnakeHead handle={eighteen}/>
        </Element>
        <Element styles={Some(row_styles)}>
            <SnakeHead handle={nineteen}/>
            <SnakeHead handle={twenty}/>
            <SnakeHead handle={twentyone}/>
            <SnakeHead handle={twentytwo}/>
            <SnakeHead handle={twentythree}/>
            <SnakeHead handle={twentyfour}/>
        </Element>
        <Element styles={Some(row_styles)}>
            <SnakeHead handle={twentyfive}/>
            <SnakeHead handle={twentysix}/>
            <SnakeHead handle={twentyseven}/>
            <SnakeHead handle={twentyeight}/>
            <SnakeHead handle={twentynine}/>
            <SnakeHead handle={thirty}/>
        </Element>
    </Element>
    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
struct SnakeHeadProps {
    #[prop_field(OnEvent)]
    on_event: Option<OnEvent>,
    handle: (usize, u16),
}

#[widget]
fn SnakeHead(props: SnakeHeadProps) {
    let image_styles = Style {
        width: StyleProp::Value(Units::Pixels(30.0)),
        height: StyleProp::Value(Units::Pixels(30.0)),
        ..Default::default()
    };

    let on_event = OnEvent::new(move |ctx, event| {
        match event.event_type {
            EventType::Click(..) => {
                ctx.query_world::<ResMut<SnakeTextureSelection>, _, _>(
                    |mut selection| {
                       selection.0 = props.handle.0 * 4;
                    },
                );
            }
            _ => (),
        }
    });
    rsx! {
        <NinePatch
            on_event={Some(on_event)}
            styles={Some(image_styles.clone())}
            border={Edge::all(1.0)}
            handle={props.handle.1}
        />
    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
struct SpeedrunsDisplayProps {
    #[prop_field(Styles)]
    styles: Option<Style>,
    #[prop_field(OnEvent)]
    on_click: Option<OnEvent>,
}
#[widget]
fn SpeedrunsDisplay(props: SpeedrunsDisplayProps) {
    let speedruns = {
        let speedruns = context
            .query_world::<Res<Binding<Speedruns>>, _, _>(
                move |speedruns| speedruns.clone(),
            );

        context.bind(&speedruns);
        speedruns.get()
    };

    let button_styles = Style {
        position_type: StyleProp::Value(
            PositionType::SelfDirected,
        ),
        width: StyleProp::Value(Units::Pixels(25.0)),
        height: StyleProp::Value(Units::Pixels(25.0)),
        ..Default::default()
    };

    let container_styles = Style {
        left: StyleProp::Value(Units::Pixels(5.0)),
        row_between: StyleProp::Value(Units::Pixels(35.0)),
        layout_type: StyleProp::Value(LayoutType::Column),
        border: StyleProp::Value(Edge::all(10.0)),
        padding_left: StyleProp::Value(Units::Stretch(1.0)),
        padding_right: StyleProp::Value(Units::Stretch(
            1.0,
        )),
        ..Default::default()
    };

    rsx! {
        <Element styles={Some(container_styles)}>
        {VecTracker::from(
            speedruns.sorted_by_run()
                .into_iter()
                .enumerate()
                .map(|(index, run)| constructor! {
                    <Text line_height={Some(30.0)} size={20.0} content={format!("#{}: {} apples in {}s",index,run.score, run.time.as_secs().to_string())}/>
                }),
        )}
        </Element>
    }
}
