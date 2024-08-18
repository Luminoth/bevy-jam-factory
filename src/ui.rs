use bevy::prelude::*;
use bevy_egui::EguiContexts;

use crate::components::{game::OnInGame, ui::NoCaptureInput};

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const _HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const _PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

// TODO: combine these intersects checks into a single system that runs before the input systems
// https://www.reddit.com/r/bevy/comments/vbll6b/capturing_mouse_clicks_in_the_ui_before_they_get/
// https://github.com/mvlabat/bevy_egui/issues/47

pub fn cursor_intersects_ui(
    window: &Window,
    node_query: &Query<(&Node, &GlobalTransform, &Visibility), Without<NoCaptureInput>>,
) -> bool {
    // TODO: this one isn't working
    window
        .cursor_position()
        .map(|cursor_position| {
            node_query
                .iter()
                .filter(|(_, _, visibility)| *visibility == Visibility::Visible)
                .any(|(node, transform, _)| {
                    let node_position = transform.translation().xy();
                    let half_size = 0.5 * node.size();
                    let min = node_position - half_size;
                    let max = node_position + half_size;
                    println!("min: {}, max: {}, cursor: {}", min, max, cursor_position);
                    (min.x..max.x).contains(&cursor_position.x)
                        && (min.y..max.y).contains(&cursor_position.y)
                })
        })
        .unwrap_or_default()
}

pub fn cursor_intersects_egui(contexts: &mut EguiContexts) -> bool {
    contexts.ctx_mut().is_pointer_over_area()
}

pub fn create_inventory_ui(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                visibility: Visibility::Visible,
                ..default()
            },
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
                    background_color: NORMAL_BUTTON.into(),
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
