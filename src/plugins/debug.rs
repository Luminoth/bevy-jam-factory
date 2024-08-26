use bevy::{
    diagnostic::{
        DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    input::common_conditions::input_toggle_active,
    prelude::*,
};
use bevy_egui::{egui, EguiContexts};

use crate::plugins::LogTextContent;

#[derive(Debug, Default)]
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // TODO: move to a plugin
        app.add_systems(
            Update,
            debug_ui.run_if(input_toggle_active(false, KeyCode::Backquote)),
        );
    }
}

fn debug_ui(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    log: Res<LogTextContent>,
    //mut inspector: ResMut<WorldInspectorParams>,
    mut contexts: EguiContexts,
) {
    egui::Window::new("Debug").show(contexts.ctx_mut(), |ui| {
        ui.vertical(|ui| {
            ui.label(format!(
                "{:.1} avg fps, {:.3} avg ms/frame",
                diagnostics
                    .get(&FrameTimeDiagnosticsPlugin::FPS)
                    .and_then(|fps| fps.smoothed())
                    .unwrap_or_default(),
                diagnostics
                    .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
                    .and_then(|frame_time| frame_time.smoothed())
                    .unwrap_or_else(|| time.delta_seconds_f64())
            ));
            ui.label(format!(
                "{:.2}% avg cpu, {:.2}% memory",
                diagnostics
                    .get(&SystemInformationDiagnosticsPlugin::CPU_USAGE)
                    .and_then(|cpu| cpu.smoothed())
                    .unwrap_or_default(),
                diagnostics
                    .get(&SystemInformationDiagnosticsPlugin::MEM_USAGE)
                    .and_then(|memory| memory.value())
                    .unwrap_or_default()
            ));
            ui.label(format!(
                "{} entities",
                diagnostics
                    .get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT)
                    .and_then(|count| count.value())
                    .unwrap_or_default()
            ));
            ui.label("Log:");
            ui.label(log.0.to_string())

            /*if ui.button("Inspector").clicked() {
                inspector.enabled = !inspector.enabled;
            }*/
        });
    });
}
