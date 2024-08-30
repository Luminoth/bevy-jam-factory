//! Game world Items
// TODO: "Item" is a terrible name ...

use bevy::prelude::*;

use super::inventory::InventoryData;
use super::objects::ObjectType;
use crate::plugins::InventoryUpdatedEvent;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, strum::EnumString, strum::Display, Reflect)]
pub enum ItemType {
    Harvester,
}

impl ItemType {
    /// Checks to see if this Item can be dropped on the given Object
    pub fn can_drop_on_object(&self, object_type: ObjectType) -> bool {
        match self {
            Self::Harvester => object_type == ObjectType::Resources,
        }
    }

    /// Creates an instance of this Item in the game at an Object
    ///
    /// Removes an instance of this Item from the Inventory
    ///
    /// # Panics
    ///
    /// This will panic if this Item is dropped on an invalid Object
    // TODO: pass in state to know where we were dropped
    pub fn on_drop_object(
        &self,
        inventory: &mut InventoryData,
        inventory_updated_events: &mut EventWriter<InventoryUpdatedEvent>,
    ) {
        match self {
            // TODO: create the item in the world
            Self::Harvester => (),
        }

        inventory.remove_item(*self, inventory_updated_events);
    }

    // TODO: pass in state to determine if we can drop here
    pub fn can_drop_on_tile(&self) -> bool {
        match self {
            Self::Harvester => false,
        }
    }

    /// Creates an instance of this Item in the game at a Tile
    ///
    /// Removes an instance of this Item from the Inventory
    ///
    /// # Panics
    ///
    /// This will panic if this Item is dropped on an invalid Tile
    // TODO: pass in state to know where we were dropped
    #[allow(unreachable_code)]
    pub fn on_drop_tile(
        &self,
        _inventory: &mut InventoryData,
        _inventory_updated_events: &mut EventWriter<InventoryUpdatedEvent>,
    ) {
        match self {
            Self::Harvester => unreachable!(),
        }

        _inventory.remove_item(*self, _inventory_updated_events);
    }
}

// TODO: this is not the move
/*#[derive(Debug, Clone, PartialEq, Eq, strum::Display)]
pub enum ItemData {
    // TODO: this also needs access to the underlying Resources
    // because it's going to modify its data
    Harvester { current: u32 },
}

impl ItemData {
    #[inline]
    pub fn get_type(&self) -> ItemType {
        match self {
            Self::Harvester { .. } => ItemType::Harvester,
        }
    }

    #[allow(dead_code)]
    pub fn update(&mut self) {
        match self {
            Self::Harvester { .. } => (),
        }
    }
}*/
