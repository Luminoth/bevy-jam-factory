use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct UiWindow;

#[derive(Debug, Component)]
pub struct UiWindowTitleBar(/*pub Entity*/);

#[derive(Debug, Component)]
pub struct UiWindowCloseButton(pub Entity);

#[derive(Debug, Component)]
pub struct UiWindowContent;

// TODO: this makes more sense as a resource
#[derive(Debug, Component, Deref, DerefMut)]
pub struct IsPointerCaptured(pub bool);

#[derive(Debug, Component)]
pub struct InventoryWindow;
