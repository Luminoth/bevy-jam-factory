//! Crafter game Item

use super::{ItemData, ItemType};

#[allow(dead_code)]
#[derive(Debug)]
pub struct CrafterData {}

impl ItemData for CrafterData {
    #[inline]
    fn get_type(&self) -> ItemType {
        ItemType::Crafter
    }
}

impl CrafterData {}
