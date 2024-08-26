use bevy::{prelude::*, window::PrimaryWindow};
use bevy_simple_scroll_view::{ScrollView, ScrollableContent};

use crate::data::{items::ItemType, objects::ResourceType};
use crate::plugins::{Inventory, InventoryUpdatedEvent, UiAssets};
use crate::ui::*;

#[derive(Debug, Component)]
pub struct InventoryWindow;

#[derive(Debug, Component)]
pub struct InventoryContent;

pub(super) fn setup_window(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    let content_id = create_window(
        &mut commands,
        &ui_assets,
        window,
        (400, 200),
        "Inventory",
        false,
        InventoryWindow,
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
                parent
                    .spawn((
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
                        InventoryContent,
                    ))
                    .with_children(|_parent| {
                    });
            });
    });
}

pub(super) fn show_inventory(mut window_query: Query<&mut Visibility, With<InventoryWindow>>) {
    *window_query.single_mut() = Visibility::Visible;
}

pub(super) fn inventory_updated_handler(
    mut events: EventReader<InventoryUpdatedEvent>,
    inventory: Res<Inventory>,
    window_query: Query<&InventoryWindow>,
) {
    if events.is_empty() {
        return;
    }

    info!("Inventory updated");

    let _window = window_query.single();

    for (resource_type, amount) in &inventory.resources {
        // TODO: updat the UI
        match resource_type {
            ResourceType::Iron => info!("Iron: {}", amount),
        }
    }

    for (item_type, amount) in &inventory.items {
        // TODO: update the UI
        match item_type {
            ItemType::Harvester => info!("Harvesters: {}", amount),
        }
    }

    events.clear();
}
