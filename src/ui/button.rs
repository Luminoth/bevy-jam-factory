use bevy::prelude::*;

use super::{label::*, *};
use crate::plugins::ui::UiAssets;

const BUTTON_WIDTH: usize = 150;
const BUTTON_HEIGHT: usize = 50;
pub const BUTTON_NORMAL: Color = Color::srgb(0.15, 0.15, 0.15);
pub const BUTTON_HOVER: Color = Color::srgb(0.25, 0.25, 0.25);
pub const BUTTON_PRESSED: Color = Color::srgb(0.35, 0.75, 0.35);
const BUTTON_FONT_SIZE: usize = 32;

pub fn create_button<'a>(
    parent: &'a mut ChildBuilder,
    ui_assets: &UiAssets,
    content: impl Into<String>,
) -> EntityCommands<'a> {
    let mut commands = parent.spawn((
        Node {
            width: Val::Px(BUTTON_WIDTH as f32),
            height: Val::Px(BUTTON_HEIGHT as f32),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Button,
        BorderColor(Color::BLACK),
        BorderRadius::MAX,
        BackgroundColor(BUTTON_NORMAL),
    ));

    commands.with_children(|parent| {
        create_label(
            parent,
            ui_assets,
            content.into(),
            BUTTON_FONT_SIZE as f32,
            FONT_COLOR,
        );
    });

    commands
}
