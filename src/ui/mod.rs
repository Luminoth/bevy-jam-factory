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
pub fn check_click_event(event: &Listener<Pointer<Click>>, button: PointerButton) -> bool {
    if event.target != event.listener() {
        return false;
    }

    if event.button != button {
        return false;
    }

    true
}

#[inline]
pub fn check_drag_start_event(event: &Listener<Pointer<DragStart>>, button: PointerButton) -> bool {
    if event.target != event.listener() {
        return false;
    }

    if event.button != button {
        return false;
    }

    true
}

#[inline]
pub fn check_drag_event(event: &Listener<Pointer<Drag>>, button: PointerButton) -> bool {
    if event.target != event.listener() {
        return false;
    }

    if event.button != button {
        return false;
    }

    true
}

#[inline]
pub fn check_drag_end_event(event: &Listener<Pointer<DragEnd>>, button: PointerButton) -> bool {
    if event.target != event.listener() {
        return false;
    }

    if event.button != button {
        return false;
    }

    true
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
