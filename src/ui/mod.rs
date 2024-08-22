pub mod button;
pub mod label;
pub mod window;

use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_mod_picking::prelude::*;

use crate::components::ui::*;
use button::*;
use label::*;
use window::*;

pub const FONT: &str = "fonts/FiraSans-Bold.ttf";
pub const FONT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

pub fn create_canvas<'a>(
    commands: &'a mut Commands,
    name: impl Into<String>,
) -> EntityCommands<'a> {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },
        Name::new(name.into()),
        Pickable::IGNORE,
    ))
}

pub fn create_column_container<'a>(parent: &'a mut ChildBuilder) -> EntityCommands<'a> {
    parent.spawn((
        NodeBundle {
            style: Style {
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
        Pickable::IGNORE,
    ))
}

pub fn create_row_container<'a>(parent: &'a mut ChildBuilder) -> EntityCommands<'a> {
    parent.spawn((
        NodeBundle {
            style: Style {
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        },
        Pickable::IGNORE,
    ))
}

pub fn create_object_info_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    window: &Window,
) {
    let content_id = create_window(
        commands,
        asset_server,
        window,
        (400, 200),
        "Object Info",
        false,
        ObjectInfoWindow,
    );
    commands.entity(content_id).with_children(|parent| {
        create_row_container(parent).with_children(|parent| {
            create_label(parent, asset_server, "Object Type:", 14.0, FONT_COLOR);
            create_label(parent, asset_server, "N/A", 14.0, FONT_COLOR);
        });

        // Resources
        // TODO: this needs to start hidden
        create_column_container(parent)
            .insert((Name::new("Resources"), ObjectInfoResources))
            .with_children(|parent| {
                create_row_container(parent).with_children(|parent| {
                    create_label(parent, asset_server, "Resource Type:", 14.0, FONT_COLOR);
                    create_label(parent, asset_server, "N/A", 14.0, FONT_COLOR);
                });

                create_row_container(parent).with_children(|parent| {
                    create_label(parent, asset_server, "Amount:", 14.0, FONT_COLOR);
                    create_label(parent, asset_server, "N/A", 14.0, FONT_COLOR);
                });
            });
    });
}

pub fn create_inventory_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    window: &Window,
) {
    let content_id = create_window(
        commands,
        asset_server,
        window,
        (400, 200),
        "Inventory",
        false,
        InventoryWindow,
    );
    commands.entity(content_id).with_children(|parent| {
        create_button(
            parent,
            asset_server,
            "Button",
            On::<Pointer<Click>>::run(move || info!("Button clicked!")),
        );
    });
}
