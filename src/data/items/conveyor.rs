//! Conveyor game Item

use super::{ItemData, ItemType};

#[allow(dead_code)]
#[derive(Debug)]
pub struct ConveyorData {}

impl ItemData for ConveyorData {
    #[inline]
    fn get_type(&self) -> ItemType {
        ItemType::Conveyor
    }
}

impl ConveyorData {}
