use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct InventoryWindow;

pub(super) fn show_inventory(mut window_query: Query<&mut Visibility, With<InventoryWindow>>) {
    *window_query.single_mut() = Visibility::Visible;
}
