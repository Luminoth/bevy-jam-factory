pub mod button;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::EguiContexts;

use crate::components::{objects::Object, ui::*};
use crate::game::objects::ObjectData;
use crate::resources::{game::ObjectInfo, ui::*};
use crate::ui::*;

pub fn should_update_object_info_ui(
    object: Option<Res<ObjectInfo>>,
    window_query: Query<&ViewVisibility, With<ObjectInfoWindow>>,
) -> bool {
    let window_visible = window_query
        .get_single()
        .map(|visible| visible.get())
        .unwrap_or_default();

    object.is_some() && window_visible
}

pub fn update_pointer_capture(
    mut is_pointer_captured: ResMut<IsPointerCaptured>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    ui_window_query: Query<(&Node, &GlobalTransform, &ViewVisibility), With<UiWindow>>,
    mut contexts: EguiContexts,
) {
    let window = window_query.single();
    let context = contexts.ctx_mut();

    is_pointer_captured.0 = window
        .cursor_position()
        .map(|cursor_position| {
            ui_window_query
                .iter()
                .filter(|(_, _, visibility)| visibility.get())
                .any(|(node, transform, _)| {
                    let node_position = transform.translation().xy();
                    let half_size = 0.5 * node.size();
                    let min = node_position - half_size;
                    let max = node_position + half_size;
                    (min.x..max.x).contains(&cursor_position.x)
                        && (min.y..max.y).contains(&cursor_position.y)
                })
        })
        .unwrap_or_default()
        || context.is_pointer_over_area()
        || context.is_using_pointer();
}

pub fn load_assets() {}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    create_object_info_ui(&mut commands, &asset_server, window);
    create_inventory_ui(&mut commands, &asset_server, window);

    commands.init_resource::<IsPointerCaptured>();
}

pub fn teardown(mut commands: Commands) {
    commands.remove_resource::<IsPointerCaptured>();
}

pub fn update_object_info_ui(
    object: Res<ObjectInfo>,
    object_query: Query<&Object>,
    mut resources_section: Query<&mut Visibility, With<ObjectInfoResources>>,
) {
    let object = object_query
        .get(object.0)
        .expect("Object tile missing Object!");

    match object.0 {
        ObjectData::Resources(_, _) => {
            // TODO: update Resource Type

            // TODO: update Resource Amount

            let mut visibility = resources_section.single_mut();
            *visibility = Visibility::Inherited;
        }
    }
}

pub fn show_inventory(mut window_query: Query<&mut Visibility, With<InventoryWindow>>) {
    *window_query.single_mut() = Visibility::Visible;
}
