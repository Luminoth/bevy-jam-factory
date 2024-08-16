use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::components::objects::Object;
use crate::resources::game::ObjectInfo;

pub fn have_object_info(object: Option<Res<ObjectInfo>>) -> bool {
    object.is_some()
}

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
            ui.label(format!("Got object: {:?}", object));

            if ui.button("Close").clicked() {
                commands.remove_resource::<ObjectInfo>();
            }
        });
    });
}
