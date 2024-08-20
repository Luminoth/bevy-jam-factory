use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct UiWindow;

// TODO: this makes more sense as a resource
#[derive(Debug, Component, Deref, DerefMut)]
pub struct IsPointerCaptured(pub bool);
