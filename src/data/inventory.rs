use std::collections::HashMap;

use bevy::prelude::*;

use super::objects::ResourceType;

#[derive(Debug, Default, Reflect)]
pub struct InventoryData {
    pub _resources: HashMap<ResourceType, u32>,
}
