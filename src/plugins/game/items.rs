use bevy::prelude::*;

use crate::data::items::ItemData;

#[derive(Debug, Component, Deref)]
pub struct Item(pub ItemData);
