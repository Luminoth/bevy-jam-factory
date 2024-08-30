use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::ObjectInfo;
use crate::data::objects::ObjectData;
use crate::plugins::{
    game_ui::{log::LogEvent, object_info::ObjectInfoWindow},
    tiled::TiledMapObjectClickEvent,
};
use crate::ui::check_click_event;

/// Game Object data component
#[derive(Debug, Component, Deref)]
pub struct Object(pub ObjectData);

pub(super) fn object_click_event_handler(
    mut commands: Commands,
    mut events: EventReader<TiledMapObjectClickEvent>,
    mut log_events: EventWriter<LogEvent>,
    mut window_query: Query<&mut Visibility, With<ObjectInfoWindow>>,
) {
    if events.is_empty() {
        return;
    }

    for event in events.read() {
        if !check_click_event(
            event.listener,
            event.target,
            event.button,
            PointerButton::Secondary,
        ) {
            continue;
        }

        commands.insert_resource(ObjectInfo(event.target));
        *window_query.single_mut() = Visibility::Visible;
        log_events.send(LogEvent::new("Showing Object Info"));
    }
}
