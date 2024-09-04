//! Game world Items
// TODO: "Item" is a terrible name ...

pub mod conveyor;
pub mod crafter;
pub mod harvester;

use bevy::prelude::*;

use super::inventory::InventoryData;
use super::objects::{ObjectData, ObjectType};
use crate::plugins::game::{inventory::InventoryUpdatedEvent, items::CreateItemEvent};

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
        create_item_events: &mut EventWriter<CreateItemEvent>,
    ) -> bool {
        let replace = match self {
            Self::Harvester => {
                let harvester_data = harvester::HarvesterData::from(object);
                create_item_events.send(CreateItemEvent::Harvester(harvester_data));

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
        _commands: &mut Commands,
        inventory: &mut InventoryData,
        inventory_updated_events: &mut EventWriter<InventoryUpdatedEvent>,
        create_item_events: &mut EventWriter<CreateItemEvent>,
    ) -> bool {
        let replace = match self {
            Self::Conveyor => {
                create_item_events.send(CreateItemEvent::Conveyor);

                false
            }
            Self::Crafter => {
                create_item_events.send(CreateItemEvent::Crafter);

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
