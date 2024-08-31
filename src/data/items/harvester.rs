//! Harvester game Item

use super::{ItemData, ItemType, ObjectData};

#[allow(dead_code)]
#[derive(Debug)]
pub struct HarvesterData {
    pub current: u32,
    pub remaining: u32,
}

impl ItemData for HarvesterData {
    #[inline]
    fn get_type(&self) -> ItemType {
        ItemType::Harvester
    }
}

impl From<&ObjectData> for HarvesterData {
    fn from(object: &ObjectData) -> Self {
        // TODO: should be an if let when we have more branches
        match object {
            ObjectData::Resources { amount, .. } => Self {
                current: 0,
                remaining: *amount,
            },
            // TODO: should this be unreachable!() ?
            /*_ => Self {
                current: 0,
                remaining: 0,
            },*/
        }
    }
}
