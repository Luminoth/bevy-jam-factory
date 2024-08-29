use bevy::prelude::*;

use super::inventory::InventoryData;
use crate::plugins::InventoryUpdatedEvent;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, strum::EnumString, strum::Display, Reflect)]
pub enum ItemType {
    Harvester,
}

impl ItemType {
    pub fn can_drop_on_object(&self) -> bool {
        true
    }

    // TODO: move inventory mutations into the InventoryData
    // and have it send the events

    pub fn drop_object(
        &self,
        inventory: &mut InventoryData,
        inventory_updated_events: &mut EventWriter<InventoryUpdatedEvent>,
    ) {
        let amount = inventory.items.get_mut(self).unwrap();
        *amount = amount.saturating_sub(1);

        inventory_updated_events.send_default();
    }

    pub fn can_drop_on_tile(&self) -> bool {
        false
    }

    pub fn drop_tile(
        &self,
        inventory: &mut InventoryData,
        inventory_updated_events: &mut EventWriter<InventoryUpdatedEvent>,
    ) {
        let amount = inventory.items.get_mut(self).unwrap();
        *amount = amount.saturating_sub(1);

        inventory_updated_events.send_default();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, strum::Display)]
pub enum ItemData {}
