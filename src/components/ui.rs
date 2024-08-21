use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct UiWindow;

#[derive(Debug, Component)]
pub struct UiWindowTitleBar(pub Entity);

#[derive(Debug, Component)]
pub struct UiWindowCloseButton(pub Entity);

#[derive(Debug, Component)]
pub struct UiWindowContent;

#[derive(Debug, Component)]
pub struct ObjectInfoWindow;

#[derive(Debug, Component)]
pub struct InventoryWindow;
