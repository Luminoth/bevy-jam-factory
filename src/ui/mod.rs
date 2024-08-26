mod button;
mod label;
mod window;

use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_mod_picking::prelude::*;
use bevy_simple_scroll_view::{ScrollView, ScrollableContent};

use crate::plugins::UiAssets;
use crate::plugins::{inventory, object_info};
pub use button::*;
pub use label::*;
pub use window::*;

pub const FONT: &str = "fonts/FiraSans-Bold.ttf";
pub const FONT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

pub fn create_canvas<'a>(commands: &'a mut Commands, name: impl AsRef<str>) -> EntityCommands<'a> {
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
        Name::new(format!("Ui Canvas - {}", name.as_ref())),
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
        Name::new("Column"),
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
        Name::new("Row"),
        Pickable::IGNORE,
    ))
}

pub fn create_object_info_ui(commands: &mut Commands, ui_assets: &Res<UiAssets>, window: &Window) {
    let content_id = create_window(
        commands,
        ui_assets,
        window,
        (400, 200),
        "Object Info",
        false,
        object_info::ObjectInfoWindow,
    );
    commands.entity(content_id).with_children(|parent| {
        create_row_container(parent).with_children(|parent| {
            create_label(parent, ui_assets, "Object ID:", 14.0, FONT_COLOR);
            create_label(parent, ui_assets, "N/A", 14.0, FONT_COLOR).insert(
                object_info::ObjectInfoDataUI(object_info::ObjectInfoData::ObjectId),
            );
        });

        create_row_container(parent).with_children(|parent| {
            create_label(parent, ui_assets, "Object Type:", 14.0, FONT_COLOR);
            create_label(parent, ui_assets, "N/A", 14.0, FONT_COLOR).insert(
                object_info::ObjectInfoDataUI(object_info::ObjectInfoData::ObjectType),
            );
        });

        // Resources
        create_column_container(parent)
            .insert((
                Visibility::Hidden,
                Name::new("Resources"),
                object_info::ObjectInfoResources,
            ))
            .with_children(|parent| {
                create_row_container(parent).with_children(|parent| {
                    create_label(parent, ui_assets, "Resource Type:", 14.0, FONT_COLOR);
                    create_label(parent, ui_assets, "N/A", 14.0, FONT_COLOR).insert(
                        object_info::ObjectInfoResourcesDataUI(
                            object_info::ObjectInfoResourcesData::ResourceType,
                        ),
                    );
                });

                create_row_container(parent).with_children(|parent| {
                    create_label(parent, ui_assets, "Amount:", 14.0, FONT_COLOR);
                    create_label(parent, ui_assets, "N/A", 14.0, FONT_COLOR).insert(
                        object_info::ObjectInfoResourcesDataUI(
                            object_info::ObjectInfoResourcesData::Amount,
                        ),
                    );
                });
            });
    });
}

pub fn create_inventory_ui(commands: &mut Commands, ui_assets: &Res<UiAssets>, window: &Window) {
    let content_id = create_window(
        commands,
        ui_assets,
        window,
        (400, 200),
        "Inventory",
        false,
        inventory::InventoryWindow,
    );
    commands.entity(content_id).with_children(|parent| {
        parent
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(80.0),
                        margin: UiRect::all(Val::Px(15.0)),
                        ..default()
                    },
                    ..default()
                },
                Name::new("Scroll View"),
                ScrollView::default(),
            ))
            .with_children(|parent| {
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: bevy::ui::FlexDirection::Column,
                            width: Val::Percent(100.0),
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Scroll Content"),
                    ScrollableContent::default(),
                    inventory::InventoryContent,
                ));
            });
    });
}
