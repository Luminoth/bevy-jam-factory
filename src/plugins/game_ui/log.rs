use bevy::{prelude::*, window::PrimaryWindow};
use bevy_mod_picking::prelude::*;

use crate::plugins::UiAssets;
use crate::ui::*;

// TODO: finish up being able to log stuff
// (log when music is toggled as an example)
// also need to be able to interact below the log window
// (eg. fully ignore picking with it)

#[derive(Debug, Component)]
pub struct LogWindow;

#[derive(Debug, Component)]
pub struct LogText;

#[derive(Debug, Default, Reflect, Resource)]
pub struct LogTextContent(pub String);

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
