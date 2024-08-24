use bevy::prelude::*;

use crate::data::objects::ObjectData;

#[derive(Debug, Component, Deref)]
pub struct Object(pub ObjectData);
