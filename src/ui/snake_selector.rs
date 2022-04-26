use bevy::prelude::{AssetServer, Handle, ResMut, World};
use kayak_ui::{
    bevy::ImageManager,
    core::{
        rsx,
        styles::{
            Edge, LayoutType, Style, StyleProp, Units,
        },
        widget, EventType, OnEvent, WidgetProps,
    },
    widgets::{Element, NinePatch},
};

use crate::settings::GameSettings;

#[widget]
pub fn SnakeSelector() {
    let container: Vec<(usize, u16)> = (1..31)
        .into_iter()
        .map(|num| {
            let mut world =
                context.get_global_mut::<World>().unwrap();

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
        width: StyleProp::Value(Units::Pixels(
            6.0 * 30.0 + 5.0 * 5.0,
        )),
        height: StyleProp::Value(Units::Pixels(
            5.0 * 30.0 + 4.0 * 15.0,
        )),
        row_between: StyleProp::Value(Units::Pixels(15.0)),
        layout_type: StyleProp::Value(LayoutType::Column),
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
                ctx.query_world::<ResMut<GameSettings>, _, _>(
                    |mut settings| {
                       settings.snake_index = props.handle.0 * 4;
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
