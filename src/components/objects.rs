use bevy::prelude::*;

use bevy_egui::egui::Ui;

#[derive(Debug, Copy, Clone, PartialEq, Eq, strum::EnumString, strum::Display)]
pub enum ObjectType {
    Resources,
}

#[derive(Debug, Component)]
pub struct Object {
    pub r#type: ObjectType,
    // TODO: need property storage
}

impl Object {
    pub fn show_info(&self, ui: &mut Ui) {
        ui.label(format!("Type: {}", self.r#type));
    }
}
