use bevy::prelude::*;

use super::ObjectInfo;
use crate::data::objects::ObjectData;
use crate::plugins::{
    game_ui::{log::LogEvent, object_info::ObjectInfoWindow},
    tiled::TiledMapObjectClickEvent,
};

/// Game Object data component
#[derive(Debug, Component, Deref)]
pub struct Object(pub ObjectData);

pub(super) fn object_click_event_handler(
    mut commands: Commands,
    mut events: EventReader<TiledMapObjectClickEvent>,
    mut log_events: EventWriter<LogEvent>,
    mut window_query: Query<&mut Visibility, With<ObjectInfoWindow>>,
) {
    for event in events.read() {
        if event.button == PointerButton::Secondary {
            commands.insert_resource(ObjectInfo(event.target));
            *window_query.single_mut() = Visibility::Visible;
            log_events.send(LogEvent::new("Showing Object Info"));
        }
    }
}
