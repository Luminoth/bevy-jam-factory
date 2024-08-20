use bevy::prelude::*;

use crate::components::{game::OnInGame, ui::*};

const FONT: &str = "fonts/FiraSans-Bold.ttf";
const FONT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

const WINDOW_BACKGROUND: Color = Color::srgba(0.15, 0.15, 0.15, 0.8);
const TITLE_HEIGHT: usize = 40;
const TITLE_BACKGROUND: Color = Color::srgb(0.1, 0.1, 0.1);
const TITLE_FONT_SIZE: usize = 40;

const BUTTON_WIDTH: usize = 150;
const BUTTON_HEIGHT: usize = 50;
pub const BUTTON_NORMAL: Color = Color::srgb(0.15, 0.15, 0.15);
pub const BUTTON_HOVER: Color = Color::srgb(0.25, 0.25, 0.25);
pub const BUTTON_PRESSED: Color = Color::srgb(0.35, 0.75, 0.35);
const BUTTON_FONT_SIZE: usize = 40;

fn create_ui_window(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    window: &Window,
    content_size: (usize, usize),
    name: impl Into<String>,
) -> Entity {
    let name = name.into();

    let window_half_width = window.width() / 2.0;
    let window_half_height = window.height() / 2.0;

    let ui_window_height = content_size.1 + TITLE_HEIGHT;
    let half_width = content_size.0 as f32 / 2.0;
    let half_height = ui_window_height as f32 / 2.0;

    let ui_window = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(content_size.0 as f32),
                    height: Val::Px((ui_window_height) as f32),
                    border: UiRect::all(Val::Px(5.0)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Center,
                    left: Val::Px(window_half_width - half_width),
                    top: Val::Px(window_half_height - half_height),
                    ..default()
                },
                background_color: WINDOW_BACKGROUND.into(),
                ..default()
            },
            Name::new(format!("UiWindow - {}", name)),
            UiWindow,
            OnInGame,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(content_size.0 as f32),
                            height: Val::Px(TITLE_HEIGHT as f32),
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Title Bar"),
                    UiWindowTitleBar,
                ))
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(content_size.0 as f32 - TITLE_HEIGHT as f32),
                                height: Val::Px(TITLE_HEIGHT as f32),
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            background_color: TITLE_BACKGROUND.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                name,
                                TextStyle {
                                    font: asset_server.load(FONT),
                                    font_size: TITLE_FONT_SIZE as f32,
                                    color: FONT_COLOR,
                                },
                            ));
                        });

                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(TITLE_HEIGHT as f32),
                                    height: Val::Px(TITLE_HEIGHT as f32),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: BUTTON_NORMAL.into(),
                                ..default()
                            },
                            Name::new("Close Button"),
                            UiWindowCloseButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "X",
                                TextStyle {
                                    font: asset_server.load(FONT),
                                    font_size: BUTTON_FONT_SIZE as f32,
                                    color: FONT_COLOR,
                                },
                            ));
                        });
                });
        })
        .id();

    let content = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(content_size.0 as f32),
                    height: Val::Px(content_size.1 as f32),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            Name::new("Content"),
            UiWindowContent,
        ))
        .id();

    commands.entity(ui_window).push_children(&[content]);

    content
}

pub fn create_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    content: impl Into<String>,
) {
    parent
        .spawn(ButtonBundle {
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
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                content.into(),
                TextStyle {
                    font: asset_server.load(FONT),
                    font_size: BUTTON_FONT_SIZE as f32,
                    color: FONT_COLOR,
                },
            ));
        });
}

pub fn create_inventory_ui(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    window: &Window,
) {
    let content_id = create_ui_window(commands, &asset_server, window, (400, 200), "Inventory");
    commands.entity(content_id).with_children(|parent| {
        create_button(parent, &asset_server, "Button");
    });
}
