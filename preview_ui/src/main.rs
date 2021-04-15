// Hide console if release build
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{prelude::*, render::camera::PerspectiveProjection};
use bevy_egui::{EguiContext, EguiPlugin, egui, egui::{Color32, Pos2}};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

#[derive(Debug)]
pub struct AppSettings {
    pub show_controls: bool
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            show_controls: true
        }
    }
}

fn main() {
    App::build()
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
        .add_startup_system(setup.system())
        .run();
}

fn ui_example(mut settings: ResMut<AppSettings>, mut egui_ctx: ResMut<EguiContext>, mut event_writer: EventWriter<bevy::app::AppExit>) {
    let ctx = &mut egui_ctx.ctx();

    // Toolbar
    egui::TopPanel::top("top_panel").show(ctx, |ui| {
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
    egui::SidePanel::left("side_panel", 500.0).show(ctx, |ui| {
        let mut style = ui.style_mut();
        style.visuals.extreme_bg_color = Color32::BLUE;

        ui.set_min_width(300.0);
        ui.heading("Options");
    });
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
        egui::Window::new("Controls").resizable(false).collapsible(false).show(ctx, |ui| {
            egui::Grid::new("grid_controls").striped(true).show(ui, |ui| {
                ui.label("Move");
                ui.label("W/A/S/D");
                ui.end_row();

                ui.label("Up");
                ui.label("Space");
                ui.end_row();

                ui.label("Down");
                ui.label("LShift");
                ui.end_row();

                ui.label("View");
                ui.label("LButton + Mouse Move");
                ui.end_row();
            });
        });
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
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

fn control_camera(
    key_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    egui_ctx: Res<bevy_egui::EguiContext>,
    mut cam_query: Query<&mut FlyCamera>,
) {
    let key_down = is_camera_button_down(&key_input);
    let mouse_down = mouse_input.pressed(MouseButton::Left);

    for mut cam in cam_query.iter_mut() {
        // Disable camera move if mouse button not held
        cam.sensitivity = match mouse_down {
            true => 3.0,
            _ => 0.0
        };

        cam.enabled = !egui_ctx.ctx().wants_pointer_input() && (key_down || mouse_down);
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