use bevy::prelude::*;

use crate::components::{game::OnInGame, ui::NoCaptureInput};

const WINDOW_BACKGROUND: Color = Color::srgba(0.15, 0.15, 0.15, 0.8);
const BUTTON_NORMAL: Color = Color::srgb(0.15, 0.15, 0.15);
const _BUTTON_HOVER: Color = Color::srgb(0.25, 0.25, 0.25);
const _BUTTON_PRESSED: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn create_inventory_ui(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    window: &Window,
) {
    // TODO: we need to create a base "canvas" for all UI to live on that is the full size of the window

    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(400.0),
                    height: Val::Px(200.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    left: Val::Px(half_width - 200.0),
                    top: Val::Px(half_height - 100.0),
                    ..default()
                },
                background_color: WINDOW_BACKGROUND.into(),
                ..default()
            },
            Name::new("Inventory UI"),
            NoCaptureInput,
            OnInGame,
        ))
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    border_radius: BorderRadius::MAX,
                    background_color: BUTTON_NORMAL.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Button",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::srgb(0.9, 0.9, 0.9),
                            },
                        ),
                        NoCaptureInput,
                    ));
                });
        });
}
