use bevy::prelude::*;
use bevy_egui::egui::Ui;

use crate::game::ObjectData;

#[derive(Debug, Component, Deref)]
pub struct Object(pub ObjectData);

impl Object {
    pub fn show_info(&self, ui: &mut Ui) {
        ui.label(format!("Object Type: {}", self.get_type()));
        self.0.show_info(ui);
    }
}
