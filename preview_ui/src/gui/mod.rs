use bevy_egui::{EguiContext, EguiPlugin, egui, egui::{Color32, CtxRef, Pos2, Ui}};
use grim::ark::{Ark, ArkOffsetEntry};
use super::{AppSettings, AppState, ArkDirNode, AppEvent};

pub fn render_gui(ctx: &mut &CtxRef, settings: &mut AppSettings, state: &mut AppState) {
    // Top Toolbar
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        // ui.heading("Main");

        egui::menu::bar(ui, |ui| {
            // File dropdown
            egui::menu::menu(ui, "File", |ui| {
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
                    //event_writer.send(bevy::app::AppExit);
                    state.add_event(AppEvent::Exit);
                }
            });

            // Edit dropdown
            egui::menu::menu(ui, "Edit", |ui| {
                ui.set_min_width(80.0);

                ui.button("Undo");
                ui.button("Redo");
            });

            // View dropdown
            egui::menu::menu(ui, "View", |ui| {
                ui.set_min_width(80.0);

                if ui.checkbox(&mut settings.show_controls, "Controls").changed() {
                    state.save_settings(&settings);
                }
            });

            // Tools dropdown
            egui::menu::menu(ui, "Tools", |ui| {
                ui.set_min_width(80.0);

                if ui.button("Options").clicked() {
                    state.show_options = true;
                }
            });

            // Help dropdown
            egui::menu::menu(ui, "Help", |ui| {
                ui.set_min_width(120.0);

                ui.button("About");
                ui.separator();
                ui.button("Check for Updates");
            });
        });
    });

    //ctx.set_visuals(egui::Visuals::light());

    // Side panel
    egui::SidePanel::left("side_panel").min_width(400.0).resizable(true).show(ctx, |ui| {
        egui::ScrollArea::auto_sized().show_viewport(ui, |ui, _viewport| {
            //ui.horizontal(|ui| {
                if settings.show_side_panel {
                    //ui.set_min_width(300.0);

                    ui.vertical(|ui| {
                        draw_ark_tree(state, ctx, ui);

                        ui.group(|ui| {
                            ui.heading("Options");
                            ui.label("Do something 1");
                            ui.label("Do something 2");

                            let popup_id = ui.make_persistent_id("popup_id");
                            let popup_btn = ui.button("Show popup");

                            if popup_btn.clicked() {
                                ui.memory().toggle_popup(popup_id);
                            }

                            egui::popup::popup_below_widget(ui, popup_id, &popup_btn, |ui| {
                                ui.group(|ui| {
                                    ui.label("Some more info, or things you can select:");
                                    ui.label("…");
                                });
                            });
                        });
                    });

                    ui.separator();
                }

                ui.style_mut().spacing.interact_size = bevy_egui::egui::Vec2::default();

                ui.vertical(|ui| {
                    ui.style_mut().spacing.item_spacing = bevy_egui::egui::Vec2::default();

                    if ui.checkbox(&mut settings.show_side_panel, "").changed() {
                        state.save_settings(&settings);
                    }
                });
            //});
        });
    });

    /*let mut frame = egui::Frame::default();
    frame.fill = Color32::from_rgba_premultiplied(0, 128, 128, 16);

    egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
        ui.label("Hello world");
    });*/

/*
    let frame = egui::Frame::none().fill(Color32::GREEN).multiply_with_opacity(0.1);
    egui::CentralPanel::default().frame(frame).show(ctx, |_| {});
*/
    // Hide menu shadow
    let mut style: egui::Style = (*ctx.style()).clone();
    let shadow_color = style.visuals.window_shadow.color;
    style.visuals.window_shadow.color = shadow_color.linear_multiply(0.0);
    ctx.set_style(style);

    /*egui::Window::new("Hello").show(ctx, |ui| {
        // let mut style = ui.style_mut();
        // style.visuals.code_bg_color = style.visuals.code_bg_color.linear_multiply(0.1);

        ui.label("world");
    });*/

    let size = ctx.used_size();
    let _size_pos = Pos2::new(size.x, size.y);

    // Camera controls
    if settings.show_controls {
        egui::Window::new("Controls").resizable(false).collapsible(false).anchor(bevy_egui::egui::Align2::RIGHT_BOTTOM, bevy_egui::egui::Vec2::new(-10.0, -10.0)).show(ctx, |ui| {
            egui::Grid::new("grid_controls").striped(true).show(ui, |ui| {
                ui.label("Move");
                ui.label("W/A/S/D");
                ui.end_row();

                ui.label("Up");
                ui.label("Space");
                ui.end_row();

                ui.label("Down");
                ui.label("L-Shift");
                ui.end_row();

                ui.label("View");
                ui.label("L-Click + Mouse");
                ui.end_row();
            });
        });
    }

    // Bottom Toolbar
    egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        ui.label("Created by PikminGuts92");
    });

    if state.show_options {
        egui::Window::new("Options")
            //.id("options_window")
            .collapsible(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .fixed_size(bevy_egui::egui::Vec2::new(600.0, 400.0))
            .open(&mut state.show_options)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        egui::Grid::new("options_list")
                            .striped(true)
                            .min_col_width(120.0)
                            .show(ui, |ui| {
                                ui.label("General");
                                ui.end_row();

                                ui.label("Ark Paths");
                                ui.end_row();

                                ui.label("Preferences");
                                ui.end_row();
                        });
                    });

                    ui.separator();

                    ui.vertical_centered_justified(|ui| {
                        ui.heading("Ark Paths");

                        egui::Grid::new("ark_paths")
                            .striped(true)
                            .show(ui, |ui| {
                                for g in settings.game_paths.iter() {
                                    ui.label(&g.game.to_string());
                                    ui.label(&g.platform.to_string());
                                    ui.label(&g.path);

                                    ui.end_row();
                                }
                            });

                        ui.add_space(500.0);
                    });
                });

                /*ui.columns(2, |cols| {
                    egui::Grid::new("options_list")
                        .striped(true)
                        .show(&mut cols[0], |ui| {
                            ui.label("Ark Paths");
                            ui.end_row();

                            ui.label("Preferences");
                            ui.end_row();
                        });

                    let ui = &mut cols[1];
                    ui.add(egui::Separator::default().vertical());

                    ui.group(|ui| {
                        ui.vertical_centered_justified(|ui| {
                            ui.heading("Ark Paths");

                            ui.add_space(500.0);
                        });
                    });

                    /*egui::Grid::new("options_view")
                        .striped(true)
                        .show(&mut cols[1], |ui| {
                            ui.label("Options view goes here");
                            ui.end_row();
                        });*/
                });*/
            });
    }
}

fn draw_ark_tree(state: &mut AppState, ctx: &mut &CtxRef, ui: &mut Ui) {
    if let Some(root) = &state.root {
        let entries = &state.ark.as_ref().unwrap().entries;

        draw_node(root, entries, ctx, ui);
    }
}

fn draw_node(node: &ArkDirNode, entries: &Vec<ArkOffsetEntry>, ctx: &mut &CtxRef, ui: &mut Ui) {
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

                    ui.selectable_label(false, file_name);
                    ui.end_row();

                    //ui.small_button(file_name);
                }
            });
        });
}

pub fn get_file_name(path: &str) -> &str {
    path.split('/').last().unwrap_or(path)
}