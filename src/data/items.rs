use bevy::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, strum::EnumString, strum::Display, Reflect)]
pub enum ItemType {}

#[derive(Debug, Clone, PartialEq, Eq, strum::Display)]
pub enum ItemData {}
