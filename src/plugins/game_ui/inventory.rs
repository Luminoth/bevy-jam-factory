use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct InventoryWindow;

#[derive(Debug, Component)]
pub struct InventoryContent;

pub(super) fn show_inventory(mut window_query: Query<&mut Visibility, With<InventoryWindow>>) {
    *window_query.single_mut() = Visibility::Visible;
}
