use bevy::{
    prelude::{
        App as BevyApp, AssetServer, Commands, Handle, Res,
        ResMut, World,
    },
    window::WindowDescriptor,
    DefaultPlugins,
};
use kayak_ui::{
    bevy::ImageManager,
    core::{
        render, rsx,
        styles::{Style, StyleProp, Units},
        use_state, widget, Binding, Bound, EventType,
        Index, OnEvent,
    },
};
use kayak_ui::{
    bevy::{
        BevyContext, BevyKayakUIPlugin, FontMapping,
        UICameraBundle,
    },
    core::styles::PositionType,
};
use kayak_ui::{
    core::styles::Corner,
    widgets::{App, Button, Image, Text, Window},
};

use crate::Game;

// #[widget]
// fn Apply() {
//     let world =
// context.get_global_mut::<World>();
//     if world.is_err() {
//         return;
//     }

//     let mut world = world.unwrap();
//     let asset_server =
//         world.get_resource::<AssetServer>().
// unwrap();

//     let handle:
// Handle<bevy::render::texture::Image> =
//         asset_server.load("generic-rpg-vendor.
// png");

//     let mut image_manager =
//         world.get_resource_mut::
// <ImageManager>().unwrap();
//     let ui_image_handle =
// image_manager.get(&handle);

//     let image_styles = Style {
//         position_type: StyleProp::Value(
//             PositionType::SelfDirected,
//         ),
//         left:
// StyleProp::Value(Units::Pixels(10.0)),
//         top:
// StyleProp::Value(Units::Pixels(10.0)),
//         border_radius:
// StyleProp::Value(Corner::all(500.0)),
//         width:
// StyleProp::Value(Units::Pixels(200.0)),
//         height:
// StyleProp::Value(Units::Pixels(182.0)),
//         ..Style::default()
//     };

//     rsx! {
//         <App>
//             <Image styles={Some(image_styles)}
// handle={ui_image_handle} />         </App>
//     }
// }
#[widget]
fn Score() {
    let text_styles = Style {
        bottom: StyleProp::Value(Units::Stretch(1.0)),
        left: StyleProp::Value(Units::Stretch(0.1)),
        right: StyleProp::Value(Units::Stretch(0.1)),
        top: StyleProp::Value(Units::Stretch(1.0)),
        width: StyleProp::Value(Units::Stretch(1.0)),
        height: StyleProp::Value(Units::Pixels(28.0)),
        ..Default::default()
    };

    let button_text_styles = Style {
        left: StyleProp::Value(Units::Stretch(1.0)),
        right: StyleProp::Value(Units::Stretch(1.0)),
        ..Default::default()
    };

    // let (count, set_count, ..) = use_state!(0i32);
    let global_count = context
        .query_world::<Res<Binding<Game>>, _, _>(
            move |global_count| global_count.clone(),
        );

    context.bind(&global_count);

    let score = global_count.get().score;
    // let on_event =
    //     OnEvent::new(move |_, event| {
    //         match event.event_type {
    //             EventType::Click(..) => {
    //                 set_count(count + 1)
    //             }
    //             _ => {}
    //         }
    //     });

    rsx! {
        <>
            // <Window draggable={true} position={(50.0, 50.0)} size={(300.0, 300.0)} title={"Counter Example".to_string()}>
                <Text styles={Some(text_styles)} size={32.0} content={format!("Score: {}", score).to_string()}>{}</Text>
                // <Button>
                //     <Text styles={Some(button_text_styles)} line_height={Some(40.0)} size={24.0} content={"Count!".to_string()}>{}</Text>
                // </Button>
            // </Window>
        </>
    }
}

pub fn ui(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UICameraBundle::new());

    font_mapping.add(
        "Roboto",
        asset_server.load("roboto.kayak_font"),
    );

    let context = BevyContext::new(|context| {
        render! {
            <App>
                <Score />
            </App>
        }
    });

    commands.insert_resource(context);
}
