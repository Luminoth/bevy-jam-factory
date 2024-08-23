use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::*;

pub fn create_label<'a>(
    parent: &'a mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    content: impl Into<String>,
    size: f32,
    color: Color,
) -> EntityCommands<'a> {
    parent.spawn((
        TextBundle::from_section(
            content.into(),
            TextStyle {
                font: asset_server.load(FONT),
                font_size: size,
                color: color,
            },
        ),
        Name::new("Label"),
        Pickable::IGNORE,
    ))
}
