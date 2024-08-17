use bevy::prelude::*;

use crate::components::game::OnInGame;

pub fn create_inventory_ui(commands: &mut Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                visibility: Visibility::Visible,
                ..default()
            },
            OnInGame,
        ))
        .with_children(|_parent| {});
}
