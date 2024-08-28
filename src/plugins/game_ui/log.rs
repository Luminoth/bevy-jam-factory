use std::time::Instant;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_mod_picking::prelude::*;

use crate::plugins::UiAssets;
use crate::ui::*;

// TODO: we need to be able to interact below the log window
// (eg. update_pointer_capture needs to ignore it)

#[derive(Debug, Component)]
pub struct LogWindow;

#[derive(Debug, Component)]
pub struct LogText;

#[derive(Debug, Default, Reflect, Resource)]
pub struct LogTextContent(pub String);

#[derive(Debug, Event)]
pub struct LogEvent {
    timestamp: Instant,
    message: String,
}

impl LogEvent {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            timestamp: Instant::now(),
            message: message.into(),
        }
    }
}

pub(super) fn setup_window(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    let log_id = create_fixed_window(
        &mut commands,
        ((window.width() - 400.0) as usize, 0),
        (400, 200),
        "Log",
        true,
        LogWindow,
    );
    commands.entity(log_id).with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "",
                TextStyle {
                    font: ui_assets.font.clone(),
                    font_size: 12.0,
                    color: FONT_COLOR,
                },
            ),
            Name::new("Log"),
            Pickable::IGNORE,
            LogText,
        ));
    });
}

pub(super) fn log_event_handler(
    mut events: EventReader<LogEvent>,
    mut log_content: ResMut<LogTextContent>,
    mut log_text_query: Query<&mut Text, With<LogText>>,
) {
    for event in events.read() {
        info!("{:?}: {}", event.timestamp, event.message);
        log_content.0.push_str(&event.message);
        log_content.0.push('\n');
    }

    let mut log_text = log_text_query.single_mut();
    log_text
        .sections
        .get_mut(0)
        .unwrap()
        .value
        .clone_from(&log_content.0);
}
