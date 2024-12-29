use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::plugins::ui::UiAssets;

pub fn create_label<'a>(
    parent: &'a mut ChildBuilder,
    ui_assets: &UiAssets,
    content: impl Into<String>,
    size: f32,
    color: Color,
) -> EntityCommands<'a> {
    parent.spawn((
        Text::new(content.into()),
        TextFont::from_font(ui_assets.font.clone()).with_font_size(size),
        TextColor(color),
        Name::new("Label"),
    ))
}
