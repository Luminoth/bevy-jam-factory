pub mod button;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui, EguiContexts};

use crate::components::{objects::Object, ui::*};
use crate::resources::{game::ObjectInfo, ui::*};
use crate::ui::*;

pub fn have_object_info(object: Option<Res<ObjectInfo>>) -> bool {
    object.is_some()
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

pub fn show_egui_object_info(
    mut commands: Commands,
    object: Res<ObjectInfo>,
    object_query: Query<&Object>,
    mut contexts: EguiContexts,
) {
    let object = object_query
        .get(object.0)
        .expect("Object tile missing Object!");

    egui::Window::new("Object Info").show(contexts.ctx_mut(), |ui| {
        ui.vertical(|ui| {
            object.show_egui_info(ui);

            if ui.button("Close").clicked() {
                commands.remove_resource::<ObjectInfo>();
            }
        });
    });
}

pub fn _show_object_info(mut window_query: Query<&mut Visibility, With<ObjectInfoWindow>>) {
    *window_query.single_mut() = Visibility::Visible;
}

pub fn show_inventory(mut window_query: Query<&mut Visibility, With<InventoryWindow>>) {
    *window_query.single_mut() = Visibility::Visible;
}
