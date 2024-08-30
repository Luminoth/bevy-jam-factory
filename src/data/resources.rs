//! Game Resources

use bevy::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, strum::EnumString, strum::Display, Reflect)]
pub enum ResourceType {
    Iron,
}
