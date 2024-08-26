use std::collections::HashMap;

use bevy::prelude::*;

use super::items::ItemType;
use super::resources::ResourceType;

#[derive(Debug, Default, Reflect)]
pub struct InventoryData {
    pub resources: HashMap<ResourceType, u32>,
    pub items: HashMap<ItemType, u32>,
}
