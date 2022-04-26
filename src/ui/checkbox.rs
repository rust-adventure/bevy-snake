use bevy::prelude::{Res, World};
use kayak_ui::{
    bevy::ImageManager,
    core::{
        rsx,
        styles::{Edge, Style, StyleProp, Units},
        widget, OnEvent, WidgetProps,
    },
    widgets::NinePatch,
};

use crate::assets::ImageAssets;

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct CheckboxProps {
    pub checked: bool,
    #[prop_field(OnEvent)]
    pub on_event: Option<OnEvent>,
}
#[widget]
pub fn Checkbox(props: CheckboxProps) {
    let styles = Style {
        width: StyleProp::Value(Units::Pixels(25.0)),
        height: StyleProp::Value(Units::Pixels(25.0)),
        ..Default::default()
    };

    let (handle_empty_box, handle_checked_box) = context
        .query_world::<Res<ImageAssets>, _, _>(|assets| {
            (
                assets.box_unchecked.clone(),
                assets.box_checked.clone(),
            )
        });

    let (empty_box, checked_box) = context
        .get_global_mut::<World>()
        .map(|mut world| {
            let mut image_manager = world
                .get_resource_mut::<ImageManager>()
                .unwrap();
            (
                image_manager.get(&handle_empty_box),
                image_manager.get(&handle_checked_box),
            )
        })
        .unwrap();

    let on_event = props.on_event.clone();

    let image = if props.checked {
        checked_box
    } else {
        empty_box
    };
    rsx! {
        <NinePatch
            on_event={on_event}
            border={Edge::all(1.0)}
            styles={Some(styles)}
            handle={image}
        />
    }
}
