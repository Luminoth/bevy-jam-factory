use bevy::prelude::*;

#[derive(Debug, Default, Reflect, Resource, Deref, DerefMut)]
pub struct IsPointerCaptured(pub bool);
