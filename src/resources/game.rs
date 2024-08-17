use std::collections::HashSet;

use bevy::prelude::*;

use crate::game::inventory::InventoryData;

#[derive(Debug, Default, Reflect, Resource, Deref)]
pub struct Inventory(pub InventoryData);

#[derive(Debug, Default, Resource)]
pub struct TileDrag {
    pub tiles: HashSet<Entity>,
}

impl TileDrag {
    pub fn new(start: Entity) -> Self {
        Self {
            tiles: HashSet::from([start]),
        }
    }
}

#[derive(Debug, Resource, Deref)]
pub struct ObjectInfo(pub Entity);
