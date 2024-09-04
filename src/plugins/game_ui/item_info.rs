use bevy::{prelude::*, window::PrimaryWindow};

use crate::plugins::{game::ItemInfo, ui::UiAssets};
use crate::ui::*;

/// Game Item info window tag
#[derive(Debug, Component)]
pub struct ItemInfoWindow;

pub(super) fn setup_window(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    let content_id = create_window(
        &mut commands,
        &ui_assets,
        window,
        (400, 200),
        "Item Info",
        false,
        ItemInfoWindow,
    );
    commands.entity(content_id).with_children(|_parent| {
        // TODO:
    });
}

pub(super) fn should_update_item_info_ui(
    item: Option<Res<ItemInfo>>,
    window_query: Query<&ViewVisibility, With<ItemInfoWindow>>,
) -> bool {
    let window_visible = window_query
        .get_single()
        .map(|visible| visible.get())
        .unwrap_or_default();

    item.is_some() && window_visible
}

// TODO: this running constantly while the UI is shown isn't great
// we should use an event or something to just update it when something changes
// (or update it once entirely on show, and then only update dynamic stuff while open)
#[allow(clippy::type_complexity)]
pub(super) fn update_item_info_ui(
    _item: Res<ItemInfo>,
    //item_query: Query<&Item>,
) {
    // TODO:
}
