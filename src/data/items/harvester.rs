//! Harvester game Item

use bevy::prelude::*;

use super::{ItemData, ItemType};

#[allow(dead_code)]
#[derive(Debug)]
pub struct HarvesterData {
    pub current: u32,
    pub resource_object_id: Entity,
}

impl ItemData for HarvesterData {
    #[inline]
    fn get_type(&self) -> ItemType {
        ItemType::Harvester
    }
}

impl HarvesterData {}
