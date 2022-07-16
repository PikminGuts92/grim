use bevy_egui::{EguiContext, EguiPlugin, egui, egui::{Color32, Context, Pos2, Ui}};
use grim::ark::{Ark, ArkOffsetEntry};
use itertools::*;
use super::{AppSettings, AppState, ArkDirNode, AppEvent};

pub fn draw_milo_tree(state: &mut AppState, _ctx: &mut &Context, ui: &mut Ui) {
    if let Some(milo) = state.milo.take() {
        let mut entries = milo.get_entries().iter().map(|e| e).collect::<Vec<_>>();

        ui.horizontal(|ui| {
            ui.label("Filter:");
            ui.text_edit_singleline(&mut state.milo_view.filter);

            // Selected class name
            let classes = entries.iter().map(|x| x.get_type()).unique().sorted().collect::<Vec<_>>();
            egui::ComboBox::from_label("")
                .width(100.0)
                .selected_text(state.milo_view.class_filter.as_ref().unwrap_or(&String::from("(None)")))
                .show_ui(ui, |ui| {
                    if ui.selectable_label(state.milo_view.class_filter.is_none(), "(None)")
                        .clicked() {
                            state.milo_view.class_filter = None;
                        };

                    for class in classes {
                        let mut checked = false;
                        if let Some(filter) = &state.milo_view.class_filter {
                            checked = filter.eq(class);
                        }

                        if ui.selectable_label(checked, class).clicked() {
                            state.milo_view.class_filter = Some(class.to_string());
                        }
                    }
                });
        });


        if let Some(selected_class) = &state.milo_view.class_filter {
            entries.retain(|e| e.get_type().eq(selected_class));
        }

        if !state.milo_view.filter.is_empty() {
            entries.retain(|e| e.get_name().contains(&state.milo_view.filter));
        }

        // Sort objects
        entries.sort_by_key(|e| e.get_name());

        egui::Grid::new("milo_tree").min_col_width(200.0).striped(true).show(ui, |ui| {
            for entry in entries.iter() {
                let entry_name = entry.get_name();
                let mut checked = false;

                if let Some(selected) = &state.milo_view.selected_entry {
                    checked = selected.eq(entry_name);
                }

                if ui.selectable_label(checked, entry_name).clicked() {
                    state.add_event(AppEvent::SelectMiloEntry(Some(entry_name.to_owned())));
                }
                ui.end_row();
            }

            if entries.is_empty() {
                ui.label("No objects found");
            }
        });

        // Give milo back
        state.milo = Some(milo);
    }
}