#![allow(unused_imports)]

// Hide console if release build
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod events;
mod gui;
mod plugins;
mod render;
mod settings;
mod state;

use events::*;
use gui::*;
use render::{render_milo, render_milo_entry};
use settings::*;
use bevy::{prelude::*, render::camera::PerspectiveProjection, window::{PresentMode, WindowMode, WindowResized}};
use bevy_egui::{EguiContext, EguiPlugin, egui, egui::{Color32, Context, Pos2, Ui}};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use grim::*;
use grim::ark::{Ark, ArkOffsetEntry};
use grim::scene::*;
use plugins::*;
use state::*;
use std::{env::args, path::{Path, PathBuf}};

use crate::render::open_and_unpack_milo;

const SETTINGS_FILE_NAME: &str = "settings.json";
const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Component)]
pub struct WorldMesh {
    name: String,
}

fn main() {
    #[cfg(target_family = "wasm")] std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    #[cfg(target_family = "wasm")] let app_state = AppState::default();
    #[cfg(target_family = "wasm")] let app_settings = AppSettings::default();

    #[cfg(not(target_family = "wasm"))] let app_state = load_state();
    #[cfg(not(target_family = "wasm"))] let app_settings = load_settings(&app_state.settings_path);

    App::new()
        .insert_resource(WindowDescriptor {
            title: format!("Preview v{}", VERSION),
            width: app_settings.window_width,
            height: app_settings.window_height,
            mode: WindowMode::Windowed,
            present_mode: PresentMode::Fifo, // vsync
            resizable: true,
            ..Default::default()
        })
        .add_event::<AppEvent>()
        //.insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(app_state)
        .insert_resource(app_settings)
        .add_plugin(GrimPlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(FlyCameraPlugin)
        .add_system(render_gui_system)
        .add_system(control_camera)
        .add_system(drop_files)
        .add_system(window_resized)
        .add_system(update_state)
        .add_startup_system(setup_args)
        .add_startup_system(setup)
        .run();
}

fn render_gui_system(mut settings: ResMut<AppSettings>, mut state: ResMut<AppState>, mut egui_ctx: ResMut<EguiContext>, mut event_writer: EventWriter<AppEvent>) {
    render_gui(&mut egui_ctx.ctx(), &mut *settings, &mut *state);

    let mut test = Vec::new();
    test.push("Test");

    state.consume_events(|ev| {
        event_writer.send(ev);
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
    /*commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.3, 0.5, 0.3),
            double_sided: true,
            unlit: false,
            ..Default::default()
        }),
        ..Default::default()
    });*/

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
    /*commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });*/
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
    mut ev_update_state: EventWriter<AppEvent>,
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
    } else if ext.contains("milo")
        || ext.contains("gh")
        || ext.contains("rnd") { // TODO: Break out into static regex
        // Open milo
        println!("Opening milo from \"{}\"", arg0);

        match open_and_unpack_milo(file_path) {
            Ok((milo, info)) => {
                println!("Successfully opened milo with {} entries", milo.get_entries().len());

                state.milo = Some(milo);
                state.system_info = Some(info);

                //ev_update_state.send(AppEvent::RefreshMilo);

                const name_prefs: [&str; 5] = ["venue", "top", "lod0", "lod1", "lod2"];

                let groups = state.milo
                    .as_ref()
                    .unwrap()
                    .get_entries()
                    .iter()
                    .filter(|o| o.get_type() == "Group")
                    .collect::<Vec<_>>();

                let mut selected_entry = None;
                /*for name in name_prefs {
                    let group = groups
                        .iter()
                        .find(|g| g.get_name().starts_with(name));

                    if let Some(grp) = group {
                        selected_entry = Some(grp.get_name().to_owned());
                        break;
                    }
                }*/

                ev_update_state.send(AppEvent::SelectMiloEntry(selected_entry));
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
    mut textures: ResMut<Assets<Image>>,
    mut update_events: EventReader<AppEvent>,
    mut event_writer: EventWriter<bevy::app::AppExit>,
    mut world_meshes: Query<(Entity, &WorldMesh)>,
    mut state: ResMut<AppState>,
) {
    for e in update_events.iter() {
        match e {
            AppEvent::Exit => {
                event_writer.send(bevy::app::AppExit);
            }
            AppEvent::SelectMiloEntry(entry_name) => {
                /*let render_entry = match &state.milo_view.selected_entry {
                    Some(name) => name.ne(entry_name),
                    None => true,
                };*/

                // Clear everything
                let mut i = 0;
                for (entity, _) in world_meshes.iter() {
                    i += 1;
                    commands.entity(entity).despawn_recursive();
                }
                if i > 0 {
                    println!("Removed {} meshes in scene", i);
                }

                /*if render_entry {
                    let milo = state.milo.as_ref().unwrap();
                    let info = state.system_info.as_ref().unwrap();

                    render_milo_entry(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        &mut textures,
                        milo,
                        Some(entry_name.to_owned()),
                        info
                    );
                }*/

                let milo = state.milo.as_ref().unwrap();
                let info = state.system_info.as_ref().unwrap();

                // Render everything for now
                render_milo_entry(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &mut textures,
                    milo,
                    entry_name.to_owned(),
                    info
                );

                state.milo_view.selected_entry = entry_name.to_owned();

                println!("Updated milo");
            },
            AppEvent::RefreshMilo => {
                return;

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