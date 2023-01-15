use std::default;

use bevy::prelude::{
    Bundle, Color, Commands, Component, Entity, In, Query,
    Res, ResMut, World,
};
use kayak_ui::{
    prelude::{Widget, WidgetName, *},
    widgets::{
        Element, ElementBundle, NinePatch, NinePatchBundle,
        TextProps, TextWidgetBundle,
    },
};

use super::{
    button,
    checkbox::CheckboxButton,
    // snake_selector::SnakeSelector,
};
use crate::{
    assets::ImageAssets,
    settings::{AudioSettings, GameSettings},
    ui::checkbox::CheckboxBundle,
};

#[derive(Component, Default, PartialEq, Clone)]
pub struct SettingsMenu {
    pub hidden: bool,
}

impl Widget for SettingsMenu {}

#[derive(Bundle)]
pub struct SettingsMenuBundle {
    pub menu: SettingsMenu,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}

impl Default for SettingsMenuBundle {
    fn default() -> Self {
        Self {
            menu: SettingsMenu::default(),
            on_event: OnEvent::default(),
            widget_name: SettingsMenu::default().get_name(),
        }
    }
}

pub fn settings_menu_render(
    // This is a bevy feature which allows custom
    // parameters to be passed into a system.
    // In this case Kayak UI gives the system a
    // `KayakWidgetContext` and an `Entity`.
    In((widget_context, entity)): In<(
        KayakWidgetContext,
        Entity,
    )>,
    // The rest of the parameters are just like those found
    // in a bevy system! In fact you can add whatever
    // you would like here including more queries or
    // lookups to resources within bevy's ECS.
    mut commands: Commands,
    images: Res<ImageAssets>,
    // In this case we really only care about our buttons
    // children! Let's query for them.
    settings: ResMut<GameSettings>,
    props: Query<&SettingsMenu>,
) -> bool {
    let props = props.get(entity).unwrap();
    dbg!(&props.hidden);
    let parent_id = Some(entity);

    let checkbox_styles = KStyle {
        layout_type: StyleProp::Value(LayoutType::Row),
        col_between: StyleProp::Value(Units::Pixels(20.0)),
        ..Default::default()
    };

    // let back = props.back.clone();

    let on_click_back = OnEvent::new(
        move |In((
            event_dispatcher_context,
            _,
            event,
            _entity,
        )): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>|
            //   mut s: ResMut<GameSettings>| {
                {
            match event.event_type {
                EventType::Click(..) => {
                    // back.call(());
                }
                _ => {}
            };
            (event_dispatcher_context, event)
        },
    );

    println!("ASFAJHSFASKJ");
    let on_click_audio = OnEvent::new(
        move |In((
            event_dispatcher_context,
            _,
            event,
            _entity,
        )): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut s: ResMut<GameSettings>| {
            match event.event_type {
                EventType::Click(..) => {
                    s.audio = match s.audio {
                        AudioSettings::ON => {
                            AudioSettings::OFF
                        }
                        AudioSettings::OFF => {
                            AudioSettings::ON
                        }
                    };
                }
                _ => {}
            };
            (event_dispatcher_context, event)
        },
    );

    let audio_checked = match settings.audio {
        AudioSettings::ON => true,
        AudioSettings::OFF => false,
    };

    rsx! {
        <ElementBundle>
            // <button::SnakeButton
            //     on_event={Some(on_click_back)}
            //     >
            //     <Text
            //         size={20.0}
            //         content={"Back".to_string()}
            //     />
            // </button::SnakeButton>
          {if props.hidden {
            constructor!{
              <ElementBundle styles={checkbox_styles}>
                <CheckboxBundle
                    button={CheckboxButton{
                        checked: audio_checked
                    }}
                    on_event={on_click_audio}
                />
                <TextWidgetBundle
                    text={TextProps{
                        size:{20.0},
                        content:{"Play Audio".to_string()},
                        ..Default::default()
                    }}
                />
            </ElementBundle>}
                }}
            // <SnakeSelector/>
        </ElementBundle>
    };
    true
}
