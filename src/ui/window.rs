use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use super::{button::*, label::*, *};
use crate::plugins::*;

const WINDOW_BACKGROUND: Color = Color::srgba(0.15, 0.15, 0.15, 0.8);
const TITLE_HEIGHT: usize = 40;
const TITLE_BACKGROUND: Color = Color::srgb(0.1, 0.1, 0.1);
const TITLE_FONT_SIZE: usize = 40;

fn drag_window(
    event: Listener<Pointer<Drag>>,
    mut window_query: Query<&mut Style, With<UiWindow>>,
    titlebar_query: Query<&UiWindowTitleBar>,
) {
    if !check_drag_event(&event, PointerButton::Primary) {
        return;
    }

    let titlebar = titlebar_query.get(event.target).unwrap();
    let mut window_style = window_query.get_mut(titlebar.0).unwrap();

    if let Val::Px(left) = &mut window_style.left {
        *left += event.delta.x;
    }

    if let Val::Px(top) = &mut window_style.top {
        *top += event.delta.y;
    }
}

fn close_window(
    event: Listener<Pointer<Click>>,
    mut window_query: Query<&mut Visibility, With<UiWindow>>,
    close_button_query: Query<&UiWindowCloseButton>,
) {
    if !check_click_event(&event, PointerButton::Primary) {
        return;
    }

    let close_button = close_button_query.get(event.target).unwrap();
    let mut window_visibility = window_query.get_mut(close_button.0).unwrap();
    *window_visibility = Visibility::Hidden;
}

pub fn create_window<C>(
    commands: &mut Commands,
    ui_assets: &Res<UiAssets>,
    window: &Window,
    content_size: (usize, usize),
    name: impl Into<String>,
    visible: bool,
    tag: C,
) -> Entity
where
    C: Component,
{
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
                    position_type: PositionType::Absolute,
                    left: Val::Px(window_half_width - half_width),
                    top: Val::Px(window_half_height - half_height),
                    ..default()
                },
                background_color: WINDOW_BACKGROUND.into(),
                visibility: if visible {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                },
                ..default()
            },
            Name::new(format!("UiWindow - {}", name)),
            Pickable::IGNORE,
            UiWindow,
            tag,
        ))
        .id();

    commands.entity(ui_window).with_children(|parent| {
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
                Pickable::IGNORE,
            ))
            .with_children(|parent| {
                parent
                    .spawn((
                        NodeBundle {
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
                        },
                        Name::new("Title"),
                        On::<Pointer<Drag>>::run(drag_window),
                        UiWindowTitleBar(ui_window),
                    ))
                    .with_children(|parent| {
                        create_label(parent, ui_assets, name, TITLE_FONT_SIZE as f32, FONT_COLOR);
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
                            // TODO: we can't make this any other color
                            // because the button::update system forces it back
                            // and that's probably not the best thing to be doing
                            background_color: BUTTON_NORMAL.into(),
                            ..default()
                        },
                        Name::new("Close Button"),
                        On::<Pointer<Click>>::run(close_window),
                        UiWindowCloseButton(ui_window),
                    ))
                    .with_children(|parent| {
                        create_label(parent, ui_assets, "X", 24.0, FONT_COLOR);
                    });
            });
    });

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
            Pickable::IGNORE,
        ))
        .id();

    commands.entity(ui_window).push_children(&[content]);

    content
}

pub fn create_fixed_window<C>(
    commands: &mut Commands,
    position: (usize, usize),
    content_size: (usize, usize),
    name: impl Into<String>,
    visible: bool,
    tag: C,
) -> Entity
where
    C: Component,
{
    let name = name.into();

    let ui_window = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(content_size.0 as f32),
                    height: Val::Px((content_size.1) as f32),
                    border: UiRect::all(Val::Px(5.0)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    left: Val::Px(position.0 as f32),
                    top: Val::Px(position.1 as f32),
                    ..default()
                },
                background_color: WINDOW_BACKGROUND.into(),
                visibility: if visible {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                },
                ..default()
            },
            Name::new(format!("UiWindow - {}", name)),
            Pickable::IGNORE,
            UiWindow,
            tag,
        ))
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
            Pickable::IGNORE,
        ))
        .id();

    commands.entity(ui_window).push_children(&[content]);

    content
}
