use bevy_egui::{EguiContext, EguiPlugin, egui, egui::{Color32, Context, Pos2, Ui}};
use grim::ark::{Ark, ArkOffsetEntry};
use super::{AppSettings, AppState, ArkDirNode, AppEvent};

pub fn draw_ark_tree(state: &mut AppState, ctx: &mut &Context, ui: &mut Ui) {
    if let Some(root) = &state.root {
        let entries = &state.ark.as_ref().unwrap().entries;

        draw_node(root, entries, ctx, ui);
    }
}

fn draw_node(node: &ArkDirNode, entries: &Vec<ArkOffsetEntry>, ctx: &mut &Context, ui: &mut Ui) {
    egui::CollapsingHeader::new(&node.name)
        .id_source(format!("dir_{}", &node.path))
        .default_open(false)
        .show(ui, |ui| {
            for child in &node.dirs {
                draw_node(child, entries, ctx, ui);
            }

            egui::Grid::new(format!("files_{}", &node.path)).striped(true).show(ui, |ui| {
                for file_idx in &node.files {
                    let ark_entry = &entries[*file_idx];
                    let file_name = get_file_name(&ark_entry.path);

                    #[allow(unused_must_use)] {
                        ui.selectable_label(false, file_name);
                    }
                    ui.end_row();

                    //ui.small_button(file_name);
                }
            });
        });
}

pub fn get_file_name(path: &str) -> &str {
    path.split('/').last().unwrap_or(path)
}