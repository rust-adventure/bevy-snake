use bevy::prelude::{Res, World};
use kayak_ui::{
    bevy::ImageManager,
    core::{
        rsx,
        styles::{
            Corner, Edge, LayoutType, Style, StyleProp,
            Units,
        },
        widget, Color, EventType, Handler, OnEvent,
        WidgetProps,
    },
    widgets::{NinePatch, Text},
};

use super::button;
use crate::assets::ImageAssets;

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct SettingsMenuProps {
    pub back: Handler<()>,
}

#[widget]
pub fn SettingsMenu(props: SettingsMenuProps) {
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

    let back = props.back.clone();
    let on_click_back = OnEvent::new(move |_, event| {
        match event.event_type {
            EventType::Click(..) => {
                back.call(());
            }
            _ => {}
        }
    });

    rsx! {
        <NinePatch
            styles={Some(container_styles)}
            border={Edge::all(10.0)}
            handle={container}
        >
            <button::SnakeButton
                on_event={Some(on_click_back)}
                >
                <Text
                    size={20.0}
                    content={"Back".to_string()}
                />
            </button::SnakeButton>
        </NinePatch>
    }
}
