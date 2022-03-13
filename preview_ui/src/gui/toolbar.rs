use bevy_egui::{EguiContext, EguiPlugin, egui, egui::{Color32, Context, Pos2, Ui}};
use super::{AppSettings, AppState, ArkDirNode, AppEvent};

pub fn render_toolbar(ctx: &mut &Context, settings: &mut AppSettings, state: &mut AppState) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        // ui.heading("Main");

        egui::menu::bar(ui, |ui| {
            // File dropdown
            egui::menu::menu_button(ui, "File", |ui| {
                ui.set_min_width(80.0);

                ui.button("Open");
                ui.separator();

                ui.button("Save");
                ui.button("Save As...");
                ui.separator();

                ui.button("Close");
                ui.separator();

                if ui.button("Exit").clicked() {
                    // Close app
                    state.add_event(AppEvent::Exit);
                }
            });

            // Edit dropdown
            egui::menu::menu_button(ui, "Edit", |ui| {
                ui.set_min_width(80.0);

                ui.button("Undo");
                ui.button("Redo");
            });

            // View dropdown
            egui::menu::menu_button(ui, "View", |ui| {
                ui.set_min_width(80.0);

                if ui.checkbox(&mut settings.show_controls, "Controls").changed() {
                    state.save_settings(&settings);
                }
            });

            // Tools dropdown
            egui::menu::menu_button(ui, "Tools", |ui| {
                ui.set_min_width(80.0);

                if ui.button("Options").clicked() {
                    state.show_options = true;
                }
            });

            // Help dropdown
            egui::menu::menu_button(ui, "Help", |ui| {
                ui.set_min_width(120.0);

                ui.button("About");
                ui.separator();
                ui.button("Check for Updates");
            });
        });
    });
}