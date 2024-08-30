use bevy::prelude::*;

use crate::data::inventory::InventoryData;

/// Game Inventory data resource
#[derive(Debug, Default, Reflect, Resource, Deref)]
pub struct Inventory(pub InventoryData);

/// Emitted when the Inventory is updated
#[derive(Debug, Default, Event)]
pub struct InventoryUpdatedEvent;
