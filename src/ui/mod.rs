mod button;
mod image;
mod label;
mod window;

use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_mod_picking::prelude::*;

pub use button::*;
pub use image::*;
pub use label::*;
pub use window::*;

pub const FONT: &str = "fonts/FiraSans-Bold.ttf";
pub const FONT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

#[inline]
pub fn check_click_event(
    listener: Entity,
    target: Entity,
    event_button: PointerButton,
    expected_button: PointerButton,
) -> bool {
    target == listener && event_button == expected_button
}

#[inline]
pub fn check_drag_start_event(
    listener: Entity,
    target: Entity,
    event_button: PointerButton,
    expected_button: PointerButton,
) -> bool {
    target == listener && event_button == expected_button
}

#[inline]
pub fn check_drag_event(
    listener: Entity,
    target: Entity,
    event_button: PointerButton,
    expected_button: PointerButton,
) -> bool {
    target == listener && event_button == expected_button
}

#[inline]
pub fn check_drag_end_event(
    listener: Entity,
    target: Entity,
    event_button: PointerButton,
    expected_button: PointerButton,
) -> bool {
    target == listener && event_button == expected_button
}

pub fn create_canvas<'a>(commands: &'a mut Commands, name: impl AsRef<str>) -> EntityCommands<'a> {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },
        Name::new(format!("Ui Canvas - {}", name.as_ref())),
        Pickable::IGNORE,
    ))
}

pub fn create_column_container<'a>(parent: &'a mut ChildBuilder) -> EntityCommands<'a> {
    parent.spawn((
        NodeBundle {
            style: Style {
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
        Name::new("Column"),
        Pickable::IGNORE,
    ))
}

pub fn create_row_container<'a>(parent: &'a mut ChildBuilder) -> EntityCommands<'a> {
    parent.spawn((
        NodeBundle {
            style: Style {
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        },
        Name::new("Row"),
        Pickable::IGNORE,
    ))
}
