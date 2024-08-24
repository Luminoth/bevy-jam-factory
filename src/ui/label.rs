use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::*;
use crate::plugins::UiAssets;

pub fn create_label<'a>(
    parent: &'a mut ChildBuilder,
    ui_assets: &Res<UiAssets>,
    content: impl Into<String>,
    size: f32,
    color: Color,
) -> EntityCommands<'a> {
    parent.spawn((
        TextBundle::from_section(
            content.into(),
            TextStyle {
                font: ui_assets.font.clone(),
                font_size: size,
                color,
            },
        ),
        Name::new("Label"),
        Pickable::IGNORE,
    ))
}
