use bevy::prelude::*;
use bevy_egui::egui::Ui;

use crate::game::objects::ObjectData;

#[derive(Debug, Component, Deref)]
pub struct Object(pub ObjectData);

impl Object {
    pub fn show_egui_info(&self, ui: &mut Ui) {
        ui.label(format!("Object Type: {}", self.get_type()));
        self.0.show_egui_info(ui);
    }
}
