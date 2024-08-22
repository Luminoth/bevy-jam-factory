use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::{label::*, *};

const BUTTON_WIDTH: usize = 150;
const BUTTON_HEIGHT: usize = 50;
pub const BUTTON_NORMAL: Color = Color::srgb(0.15, 0.15, 0.15);
pub const BUTTON_HOVER: Color = Color::srgb(0.25, 0.25, 0.25);
pub const BUTTON_PRESSED: Color = Color::srgb(0.35, 0.75, 0.35);
const BUTTON_FONT_SIZE: usize = 32;

pub fn create_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    content: impl Into<String>,
    on_click: On<Pointer<Click>>,
) -> Entity {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(BUTTON_WIDTH as f32),
                    height: Val::Px(BUTTON_HEIGHT as f32),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                border_radius: BorderRadius::MAX,
                background_color: BUTTON_NORMAL.into(),
                ..default()
            },
            on_click,
        ))
        .with_children(|parent| {
            create_label(
                parent,
                asset_server,
                content.into(),
                BUTTON_FONT_SIZE as f32,
                FONT_COLOR,
            );
        })
        .id()
}
