use bevy::prelude::*;

use crate::game::objects::ObjectData;

#[derive(Debug, Component, Deref)]
pub struct Object(pub ObjectData);
