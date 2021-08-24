// Hide console if release build
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod render;
mod settings;

use render::render_milo;
use settings::*;
use bevy::{prelude::*, render::camera::PerspectiveProjection, window::{WindowMode, WindowResized}};
use bevy_egui::{EguiContext, EguiPlugin, egui, egui::{Color32, CtxRef, Pos2, Ui}};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use grim::*;
use grim::ark::{Ark, ArkOffsetEntry};
use grim::scene::*;
use std::{env::args, path::{Path, PathBuf}};
use itertools::Itertools;

use crate::render::open_and_unpack_milo;

const SETTINGS_FILE_NAME: &str = "settings.json";
const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Default)]
pub struct AppState {
    pub ark: Option<Ark>,
    pub root: Option<ArkDirNode>,
    pub system_info: Option<SystemInfo>,
    pub milo: Option<ObjectDir>,
    pub settings_path: PathBuf,
    pub show_options: bool,
}

enum UpdateState {
    RefreshMilo,
}

impl AppState {
    pub fn save_settings(&self, settings: &AppSettings) {
        settings.save_to_file(&self.settings_path);
        println!("Saved settings to \"{}\"", &self.settings_path.to_str().unwrap());
    }
}

#[derive(Debug)]
pub struct ArkDirNode {
    pub name: String,
    pub path: String,
    pub dirs: Vec<ArkDirNode>,
    pub files: Vec<usize>,
    pub loaded: bool,
}

impl ArkDirNode {
    pub fn expand(&mut self, ark: &Ark) {
        if self.loaded {
            return;
        }

        let (mut dirs, mut files) = get_dirs_and_files(&self.path, ark);
        self.dirs.append(&mut dirs);
        self.files.append(&mut files);
        self.loaded = true;

        // TODO: Rely on lazy load
        for c in &mut self.dirs {
            c.expand(ark);
        }
    }
}

pub fn get_file_name(path: &str) -> &str {
    path.split('/').last().unwrap_or(path)
}

fn main() {
    let app_state = load_state();
    let app_settings = load_settings(&app_state.settings_path);

    App::build()
        .insert_resource(WindowDescriptor {
            title: String::from("Preview"),
            width: app_settings.window_width,
            height: app_settings.window_height,
            mode: WindowMode::Windowed,
            vsync: true,
            resizable: true,
            ..Default::default()
        })
        .add_event::<UpdateState>()
        .insert_resource(Msaa { samples: 8 })
        .insert_resource(app_state)
        .insert_resource(app_settings)
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(FlyCameraPlugin)
        .add_system(ui_example.system())
        .add_system(control_camera.system())
        .add_system(drop_files.system())
        .add_system(window_resized.system())
        .add_system(update_state.system())
        .add_startup_system(setup_args.system())
        .add_startup_system(setup.system())
        .run();
}

fn ui_example(mut settings: ResMut<AppSettings>, mut state: ResMut<AppState>, egui_ctx: ResMut<EguiContext>, mut event_writer: EventWriter<bevy::app::AppExit>) {
    let ctx = &mut egui_ctx.ctx();

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
                    event_writer.send(bevy::app::AppExit);
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
                        draw_ark_tree(&state, ctx, ui);

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

fn draw_ark_tree(state: &ResMut<AppState>, ctx: &mut &CtxRef, ui: &mut Ui) {
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut windows: ResMut<Windows>,
    settings: Res<AppSettings>,
    state: Res<AppState>,
) {
    // Set primary window to maximized if preferred
    if settings.maximized {
        let window = windows.get_primary_mut().unwrap();
        window.set_maximized(true);
    }

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.3, 0.5, 0.3),
            double_sided: true,
            unlit: false,
            ..Default::default()
        }),
        ..Default::default()
    });

    /*
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(
            shape::Icosphere {
                radius: 0.8,
                subdivisions: 5,
            })
        ),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(1.0, 0.0, 1.0),
            double_sided: true,
            unlit: false,
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 2.0, 0.0),
        ..Default::default()
    });

    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.8, 0.7, 0.6),
            double_sided: true,
            unlit: false,
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });*/
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    let mut camera = PerspectiveCameraBundle::new_3d();
    camera.transform = Transform::from_xyz(-2.0, 2.5, 5.0)
        .looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn_bundle(camera).insert(FlyCamera {
        enabled: false,
        sensitivity: 0.0,
        ..Default::default()
    });
}

fn load_state() -> AppState {
    let exe_path = &std::env::current_exe().unwrap();
    let exe_dir_path = exe_path.parent().unwrap();
    let settings_path = exe_dir_path.join(&format!("{}.{}", PROJECT_NAME, SETTINGS_FILE_NAME));

    AppState {
        settings_path,
        //show_options: true, // TODO: Remove after testing
        ..Default::default()
    }
}

fn load_settings(settings_path: &Path) -> AppSettings {
    let settings = AppSettings::load_from_file(settings_path);
    println!("Loaded settings from \"{}\"", settings_path.to_str().unwrap());

    settings
}

fn setup_args(
    mut state: ResMut<AppState>,
    mut ev_update_state: EventWriter<UpdateState>,
) {
    let args = args().skip(1).collect::<Vec<String>>();
    if args.is_empty() {
        return;
    }

    let arg0 = args[0].as_str();
    let file_path = Path::new(arg0);
    let ext = file_path.extension().unwrap().to_str().unwrap();

    if ext.contains("hdr") {
        // Open ark
        println!("Opening hdr from \"{}\"", arg0);

        let ark_res = Ark::from_path(file_path);
        if let Ok(ark) = ark_res {
            println!("Successfully opened ark with {} entries", ark.entries.len());

            state.root = Some(create_ark_tree(&ark));
            state.ark = Some(ark);
        }
    } else if ext.contains("milo") {
        // Open milo
        println!("Opening milo from \"{}\"", arg0);

        match open_and_unpack_milo(file_path) {
            Ok((milo, info)) => {
                println!("Successfully opened milo with {} entries", milo.get_entries().len());

                state.milo = Some(milo);
                state.system_info = Some(info);

                ev_update_state.send(UpdateState::RefreshMilo);
            },
            Err(_err) => {
                // TODO: Log error
            }
        }
    } else {
        println!("Unknown file type \"{}\"", arg0);
    }
}

fn update_state(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    mut update_events: EventReader<UpdateState>,
    state: Res<AppState>,
) {
    for e in update_events.iter() {
        match e {
            UpdateState::RefreshMilo => {
                if let Some(milo) = &state.milo {
                    let info = state.system_info.as_ref().unwrap();
                    render_milo(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        &mut textures,
                        milo,
                        info
                    );
                }

                println!("Updated milo");
            }
        }
    }
}

fn create_ark_tree(ark: &Ark) -> ArkDirNode {
    let mut root = ArkDirNode {
        name: ark.path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned(), // There's gotta be a better conversion...
        path: String::from(""),
        dirs: Vec::new(),
        files: Vec::new(),
        loaded: false
    };

    root.expand(ark);
    root
}

fn get_dirs_and_files(dir: &str, ark: &Ark) -> (Vec<ArkDirNode>, Vec<usize>) {
    let is_root = match dir {
        "" | "." => true,
        _ => false,
    };

    if is_root {
        let files = ark.entries
            .iter()
            .enumerate()
            .filter(|(_i, e)| !e.path.contains('/')
                || (e.path.starts_with("./") && e.path.matches(|c: char | c.eq(&'/')).count() == 1))
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        let dirs = ark.entries
            .iter()
            .filter(|e| e.path.contains('/'))
            .map(|e| e.path.split('/').next().unwrap())
            .unique()
            .filter(|s| !s.eq(&"."))
            .map(|s| ArkDirNode {
                name: s.to_owned(),
                path: s.to_owned(),
                dirs: Vec::new(),
                files: Vec::new(),
                loaded: false,
            })
            .collect::<Vec<ArkDirNode>>();

        return (dirs, files);
    }

    let dir_path = format!["{}/", dir];
    let slash_count = dir_path.matches(|c: char| c.eq(&'/')).count();

    let files = ark.entries
        .iter()
        .enumerate()
        .filter(|(_i, e)| e.path.starts_with(&dir_path)
            && e.path.matches(|c: char| c.eq(&'/')).count() == slash_count)
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    let dirs = ark.entries
        .iter()
        .filter(|e| e.path.starts_with(&dir_path)
            && e.path.matches(|c: char| c.eq(&'/')).count() > slash_count)
        .map(|e| e.path.split('/').nth(slash_count).unwrap())
        .unique()
        .map(|s| ArkDirNode {
            name: s.to_owned(),
            path: format!("{}{}", dir_path, s),
            dirs: Vec::new(),
            files: Vec::new(),
            loaded: false,
        })
        .collect::<Vec<ArkDirNode>>();

    (dirs, files)
}

fn control_camera(
    key_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    egui_ctx: Res<bevy_egui::EguiContext>,
    mut cam_query: Query<&mut FlyCamera>,
) {
    let ctx = egui_ctx.ctx();

    let key_down = is_camera_button_down(&key_input);
    let mouse_down = mouse_input.pressed(MouseButton::Left);

    for mut cam in cam_query.iter_mut() {
        // Disable camera move if mouse button not held
        cam.sensitivity = match mouse_down {
            true => 3.0,
            _ => 0.0
        };

        cam.enabled = !ctx.wants_pointer_input()
            && !ctx.is_pointer_over_area()
            && (key_down || mouse_down);
    }
}

fn is_camera_button_down(key_input: &Res<Input<KeyCode>>) -> bool {
    let control_keys = [
        KeyCode::W,
        KeyCode::A,
        KeyCode::S,
        KeyCode::D,
        KeyCode::Space,
        KeyCode::LShift,
    ];

    control_keys
        .iter()
        .any(|k| key_input.pressed(*k))
}

fn window_resized(
    mut resize_events: EventReader<WindowResized>,
    mut settings: ResMut<AppSettings>,
    app_state: Res<AppState>,
) {
    for e in resize_events.iter() {
        println!("Window resized: {}x{}", e.width, e.height);

        settings.window_width = e.width;
        settings.window_height = e.height;
        app_state.save_settings(&settings);
    }
}

fn drop_files(
    mut drag_drop_events: EventReader<FileDragAndDrop>,
) {
    // Currently doesn't work on Windows
    // https://github.com/bevyengine/bevy/issues/2096
    for d in drag_drop_events.iter() {
        if let FileDragAndDrop::DroppedFile { id: _, path_buf } = d {
            println!("Dropped \"{}\"", path_buf.to_str().unwrap())
        }
    }
}

fn is_drop_event(dad_event: &FileDragAndDrop) -> bool {
    match dad_event {
        FileDragAndDrop::DroppedFile { id: _, path_buf: _ } => true,
        _ => false
    }
}