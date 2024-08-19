use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui, EguiContexts};

use crate::components::{game::OnInGame, objects::Object, ui::*};
use crate::resources::game::ObjectInfo;
use crate::ui::*;

pub fn have_object_info(object: Option<Res<ObjectInfo>>) -> bool {
    object.is_some()
}

pub fn update_pointer_capture(
    mut capture_query: Query<&mut IsPointerCaptured>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    node_query: Query<(&Node, &GlobalTransform, &ViewVisibility), Without<NoCaptureInput>>,
    mut contexts: EguiContexts,
) {
    let mut is_pointer_captured = capture_query.single_mut();
    let window = window_query.single();
    let context = contexts.ctx_mut();

    is_pointer_captured.0 = window
        .cursor_position()
        .map(|cursor_position| {
            node_query
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

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    create_inventory_ui(&mut commands, asset_server);

    commands.spawn((IsPointerCaptured(false), OnInGame));
}

pub fn teardown() {}

pub fn show_object_info(
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
            object.show_info(ui);

            if ui.button("Close").clicked() {
                commands.remove_resource::<ObjectInfo>();
            }
        });
    });
}
