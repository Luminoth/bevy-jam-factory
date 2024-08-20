use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct UiWindow;

#[derive(Debug, Component)]
pub struct UiWindowTitleBar;

#[derive(Debug, Component)]
pub struct UiWindowCloseButton;

#[derive(Debug, Component)]
pub struct UiWindowContent;

// TODO: this makes more sense as a resource
#[derive(Debug, Component, Deref, DerefMut)]
pub struct IsPointerCaptured(pub bool);
