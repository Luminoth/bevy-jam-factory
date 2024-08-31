//! Game world Items
// TODO: "Item" is a terrible name ...

pub mod conveyor;
pub mod crafter;
pub mod harvester;

use bevy::prelude::*;

use super::inventory::InventoryData;
use super::objects::{ObjectData, ObjectType};
use crate::plugins::game::inventory::InventoryUpdatedEvent;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, strum::EnumString, strum::Display, Reflect)]
pub enum ItemType {
    Harvester,
    Conveyor,
    Crafter,
}

impl ItemType {
    /// Checks to see if this Item can be dropped on the given Object
    pub fn can_drop_on_object(&self, object_type: ObjectType) -> bool {
        match self {
            Self::Harvester => object_type == ObjectType::Resources,
            Self::Conveyor | Self::Crafter => false,
        }
    }

    /// Creates an instance of this Item in the game at an Object
    ///
    /// Removes an instance of this Item from the Inventory
    /// Returns true if the Item replaces the Object
    ///
    /// # Panics
    ///
    /// This will panic if this Item is dropped on an invalid Object
    // TODO: pass in state to know where we were dropped
    pub fn on_drop_object(
        &self,
        inventory: &mut InventoryData,
        inventory_updated_events: &mut EventWriter<InventoryUpdatedEvent>,
        object: &ObjectData,
    ) -> bool {
        let replace = match self {
            Self::Harvester => {
                info!("TODO: create harvester");
                let _harvester = harvester::HarvesterData::from(object);

                // TODO: add the item in the world

                true
            }
            Self::Conveyor | Self::Crafter => unreachable!(),
        };

        inventory.remove_item(*self, inventory_updated_events);

        replace
    }

    // TODO: pass in state to determine if we can drop here
    pub fn can_drop_on_tile(&self) -> bool {
        match self {
            Self::Conveyor => true,
            Self::Crafter => true,
            Self::Harvester => false,
        }
    }

    /// Creates an instance of this Item in the game at a Tile
    ///
    /// Removes an instance of this Item from the Inventory
    /// Returns true if the Item replaces the Tile
    ///
    /// # Panics
    ///
    /// This will panic if this Item is dropped on an invalid Tile
    pub fn on_drop_tile(
        &self,
        inventory: &mut InventoryData,
        inventory_updated_events: &mut EventWriter<InventoryUpdatedEvent>,
    ) -> bool {
        let replace = match self {
            Self::Conveyor => {
                // TODO: create the item in the world
                info!("TODO: create conveyor");

                false
            }
            Self::Crafter => {
                // TODO: create the item in the world
                info!("TODO: create crafter");

                false
            }
            Self::Harvester => unreachable!(),
        };

        inventory.remove_item(*self, inventory_updated_events);

        replace
    }
}

pub trait ItemData {
    #[allow(dead_code)]
    fn get_type(&self) -> ItemType;
}
