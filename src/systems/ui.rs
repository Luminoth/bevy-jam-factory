use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::components::objects::Object;
use crate::resources::game::ObjectInfo;
use crate::ui::*;

pub fn have_object_info(object: Option<Res<ObjectInfo>>) -> bool {
    object.is_some()
}

pub fn load_assets() {}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    create_inventory_ui(&mut commands, asset_server);
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
