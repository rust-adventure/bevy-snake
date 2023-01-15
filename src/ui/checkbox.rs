use bevy::prelude::{
    Bundle, Commands, Component, Entity, In, Query, Res,
    World,
};
use kayak_ui::{
    prelude::{KayakWidgetContext, Widget, WidgetName, *},
    widgets::{KImage, KImageBundle},
};

use crate::assets::ImageAssets;

#[derive(Component, Default, PartialEq, Clone)]
pub struct CheckboxButton {
    pub checked: bool,
}

impl Widget for CheckboxButton {}

#[derive(Bundle)]
pub struct CheckboxBundle {
    pub button: CheckboxButton,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}

impl Default for CheckboxBundle {
    fn default() -> Self {
        Self {
            button: CheckboxButton::default(),
            on_event: OnEvent::default(),
            widget_name: CheckboxButton::default()
                .get_name(),
        }
    }
}
pub fn checkbox_button_render(
    In((widget_context, entity)): In<(
        KayakWidgetContext,
        Entity,
    )>,
    mut commands: Commands,
    images: Res<ImageAssets>,
    checkbox_query: Query<&CheckboxButton>,
) -> bool {
    let parent_id = Some(entity);

    let data = checkbox_query.get(entity).unwrap();

    let image = if data.checked {
        images.box_checked.clone()
    } else {
        images.box_unchecked.clone()
    };
    rsx! {
        <KImageBundle
        image={KImage(image)}
        styles={KStyle {
            width: StyleProp::Value(Units::Pixels(25.0)),
            height: StyleProp::Value(Units::Pixels(25.0)),
            ..Default::default()
        }}
    />
    };
    true
}
