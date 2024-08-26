use bevy::prelude::*;

use super::super::{Inventory, InventoryUpdatedEvent};

#[derive(Debug, Component)]
pub struct InventoryWindow;

#[derive(Debug, Component)]
pub struct InventoryContent;

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

    for (_resource_type, _amount) in &inventory.resources {
        // TODO: updat the UI
    }

    for (_item_type, _amount) in &inventory.items {
        // TODO: update the UI
    }

    events.clear();
}
