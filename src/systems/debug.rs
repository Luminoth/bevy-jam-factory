use bevy::{
    diagnostic::{
        DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    prelude::*,
};
use bevy_egui::{egui, EguiContexts};

// TODO: this should be a plugin

pub fn debug_ui(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
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

            /*if ui.button("Inspector").clicked() {
                inspector.enabled = !inspector.enabled;
            }*/
        });
    });
}
