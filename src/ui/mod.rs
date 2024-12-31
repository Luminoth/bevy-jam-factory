mod button;
mod image;
mod label;
mod tween;
mod window;

use bevy::{ecs::system::EntityCommands, prelude::*};

pub use button::*;
pub use image::*;
pub use label::*;
pub use tween::*;
pub use window::*;

pub const FONT: &str = "fonts/FiraSans-Bold.ttf";
pub const FONT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

pub fn create_canvas<'a>(commands: &'a mut Commands, name: impl AsRef<str>) -> EntityCommands<'a> {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        Name::new(format!("Ui Canvas - {}", name.as_ref())),
        PickingBehavior::IGNORE,
    ))
}

pub fn create_column_container<'a>(parent: &'a mut ChildBuilder) -> EntityCommands<'a> {
    parent.spawn((
        Node {
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        Name::new("Column"),
        PickingBehavior::IGNORE,
    ))
}

pub fn create_row_container<'a>(parent: &'a mut ChildBuilder) -> EntityCommands<'a> {
    parent.spawn((
        Node {
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        Name::new("Row"),
        PickingBehavior::IGNORE,
    ))
}
