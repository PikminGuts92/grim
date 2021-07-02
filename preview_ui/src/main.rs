// Hide console if release build
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{prelude::*, render::camera::PerspectiveProjection};
use bevy_egui::{EguiContext, EguiPlugin, egui, egui::{Color32, CtxRef, Pos2, Ui}};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use grim::ark::{Ark, ArkOffsetEntry};
use std::env::args;
use itertools::Itertools;

#[derive(Debug)]
pub struct AppSettings {
    pub show_controls: bool,
    pub show_side_panel: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            show_controls: true,
            show_side_panel: true,
        }
    }
}

#[derive(Debug, Default)]
pub struct AppState {
    pub ark: Option<Ark>,
    pub root: Option<ArkDirNode>,
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
    path.split("/").last().unwrap_or(&path)
}

fn main() {
    App::build()
        .insert_resource(AppState::default())
        .insert_resource(AppSettings::default())
        .insert_resource(Msaa { samples: 8 })
        .insert_resource(WindowDescriptor {
            title: String::from("Preview"),
            width: 1920.0,
            height: 1080.0,
            vsync: true,
            resizable: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(FlyCameraPlugin)
        .add_system(ui_example.system())
        .add_system(control_camera.system())
        .add_startup_system(setup_args.system())
        .add_startup_system(setup.system())
        .run();
}

fn ui_example(mut settings: ResMut<AppSettings>, mut state: ResMut<AppState>, mut egui_ctx: ResMut<EguiContext>, mut event_writer: EventWriter<bevy::app::AppExit>) {
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

                ui.checkbox(&mut settings.show_controls, "Controls");
            });

            // Tools dropdown
            egui::menu::menu(ui, "Tools", |ui| {
                ui.set_min_width(80.0);

                ui.button("Options");
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
        egui::ScrollArea::auto_sized().show_viewport(ui, |ui, viewport| {
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

                    ui.checkbox(&mut settings.show_side_panel, "");
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
    let shadow_color = style.visuals.window_shadow.color.clone();
    style.visuals.window_shadow.color = shadow_color.linear_multiply(0.0);
    ctx.set_style(style);

    /*egui::Window::new("Hello").show(ctx, |ui| {
        // let mut style = ui.style_mut();
        // style.visuals.code_bg_color = style.visuals.code_bg_color.linear_multiply(0.1);

        ui.label("world");
    });*/

    let size = ctx.used_size();
    let size_pos = Pos2::new(size.x, size.y);

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
}

fn draw_ark_tree(mut state: ResMut<AppState>, ctx: &mut &CtxRef, ui: &mut Ui) {
    if let Some(root) = &state.root {
        let entries = &state.ark.as_ref().unwrap().entries;

        draw_node(root, entries, ctx, ui);
    }
}

fn draw_node(node: &ArkDirNode, entries: &Vec<ArkOffsetEntry>, ctx: &mut &CtxRef, ui: &mut Ui) {
    egui::CollapsingHeader::new(&node.name)
        .default_open(false)
        .show(ui, |ui| {
            for child in &node.dirs {
                draw_node(child, entries, ctx, ui);
            }

            egui::Grid::new(&node.path).striped(true).show(ui, |ui| {
                for file_idx in &node.files {
                    let ark_entry = &entries[*file_idx];
                    let file_name = get_file_name(&ark_entry.path);

                    ui.label(file_name);
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
) {
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
    });
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

fn setup_args(mut state: ResMut<AppState>) {
    let args = args().skip(1).collect::<Vec<String>>();
    if args.is_empty() {
        return;
    }

    let hdr_path = &args[0];
    println!("Hdr path is \"{}\"", hdr_path);

    let ark_res = Ark::from_path(hdr_path);
    if let Ok(ark) = ark_res {
        state.root = Some(create_ark_tree(&ark));
        state.ark = Some(ark);
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
            .filter(|(i, e)| !e.path.contains("/")
                || (e.path.starts_with("./") && e.path.matches(|c: char | c.eq(&'/')).count() == 1))
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        let dirs = ark.entries
            .iter()
            .filter(|e| e.path.contains("/"))
            .map(|e| e.path.split("/").next().unwrap())
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
        .filter(|(i, e)| e.path.starts_with(&dir_path)
            && e.path.matches(|c: char| c.eq(&'/')).count() == slash_count)
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    let dirs = ark.entries
        .iter()
        .filter(|e| e.path.starts_with(&dir_path)
            && e.path.matches(|c: char| c.eq(&'/')).count() > slash_count)
        .map(|e| e.path.split("/").skip(slash_count).next().unwrap())
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