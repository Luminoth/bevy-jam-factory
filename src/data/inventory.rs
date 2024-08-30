//! Inventory game data

use std::collections::HashMap;

use bevy::prelude::*;

use super::items::ItemType;
use super::resources::ResourceType;
use crate::plugins::InventoryUpdatedEvent;

#[derive(Debug, Default, Reflect)]
pub struct InventoryData {
    resources: HashMap<ResourceType, u32>,
    items: HashMap<ItemType, u32>,
}

impl InventoryData {
    // TODO: temp hack to get some stuff in the inventory for testing
    pub fn new_test() -> Self {
        Self {
            resources: HashMap::from([(ResourceType::Iron, 100)]),
            items: HashMap::from([(ItemType::Harvester, 1)]),
        }
    }

    #[inline]
    pub fn get_resources(&self) -> &HashMap<ResourceType, u32> {
        &self.resources
    }

    #[inline]
    pub fn get_items(&self) -> &HashMap<ItemType, u32> {
        &self.items
    }

    /// Adds an Item to the Inventory
    ///
    /// Sends an InventoryUpdatedEvent event after adding the Item
    #[allow(dead_code)]
    pub fn add_item(
        &mut self,
        item_type: ItemType,
        inventory_updated_events: &mut EventWriter<InventoryUpdatedEvent>,
    ) {
        let amount = self.items.entry(item_type).or_default();
        *amount += 1;
        inventory_updated_events.send_default();
    }

    /// Removes an Item from the Inventory
    ///
    /// Sends an InventoryUpdatedEvent event after removing the Item
    pub fn remove_item(
        &mut self,
        item_type: ItemType,
        inventory_updated_events: &mut EventWriter<InventoryUpdatedEvent>,
    ) {
        let amount = self.items.entry(item_type).or_default();
        *amount = amount.saturating_sub(1);
        inventory_updated_events.send_default();
    }
}
