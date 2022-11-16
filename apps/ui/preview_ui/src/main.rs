#![allow(dead_code)]
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
use bevy::{prelude::*, render::camera::PerspectiveProjection, window::{PresentMode, WindowMode, WindowResized}, winit::WinitWindows};
use bevy_egui::{EguiContext, EguiPlugin, egui, egui::{Color32, Context, Pos2, Ui}};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bevy_infinite_grid::{GridShadowCamera, InfiniteGridBundle, InfiniteGrid, InfiniteGridPlugin};
use grim::*;
use grim::ark::{Ark, ArkOffsetEntry};
use grim::scene::*;
use plugins::*;
use state::*;
use std::{env::args, path::{Path, PathBuf}};

use crate::render::open_and_unpack_milo;

#[derive(Component)]
pub struct WorldMesh {
    name: String,
    vert_count: usize,
    face_count: usize,
}

fn main() {
    App::new()
        .add_event::<AppEvent>()
        .add_event::<AppFileEvent>()
        //.insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa { samples: 4 })
        .add_plugin(GrimPlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(FlyCameraPlugin)
        .add_plugin(InfiniteGridPlugin)
        .add_system(render_gui_system)
        .add_system(detect_meshes)
        .add_system(control_camera)
        .add_system(drop_files)
        .add_system(window_resized)
        .add_system(consume_file_events)
        .add_system(consume_app_events)
        .add_startup_system(setup_args)
        .add_startup_system(setup)
        .run();
}

fn render_gui_system(mut settings: ResMut<AppSettings>, mut state: ResMut<AppState>, egui_ctx: ResMut<EguiContext>, mut event_writer: EventWriter<AppEvent>) {
    render_gui(&mut egui_ctx.ctx(), &mut *settings, &mut *state);
    render_gui_info(&mut egui_ctx.ctx(), &mut *state);

    state.consume_events(|ev| {
        event_writer.send(ev);
    });
}

fn detect_meshes(
    mut state: ResMut<AppState>,
    meshes: Res<Assets<Mesh>>,
    mesh_entities: Query<(&Handle<Mesh>, &WorldMesh, Option<&Visibility>)>,
    //added_meshes: Query<(&WorldMesh, &Handle<Mesh>), Added<WorldMesh>>,
    //removed_meshes: RemovedComponents<Mesh>,
) {
    let mut vertex_count = 0;
    let mut face_count = 0;

    for (mesh_id, world_mesh, visibility) in mesh_entities.iter() {
        if let Some(_mesh) = meshes.get(mesh_id) {
            let is_visible = visibility
                .map_or(false, |v| v.is_visible);

            // Ignore invisible meshes
            if !is_visible {
                continue;
            }

            //vertex_count += mesh.count_vertices();
            vertex_count += world_mesh.vert_count;
            face_count += world_mesh.face_count;
        }
    }

    // Update counts
    state.vert_count = vertex_count;
    state.face_count = face_count;
}

fn setup(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
    mut windows: ResMut<Windows>,
    settings: Res<AppSettings>,
    _state: Res<AppState>,
) {
    // Set primary window to maximized if preferred
    if settings.maximized {
        let window = windows.primary_mut();
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
    let mut camera = Camera3dBundle::default();
    camera.transform = Transform::from_xyz(-2.0, 2.5, 5.0)
        .looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn(camera).insert(FlyCamera {
        enabled: false,
        sensitivity: 0.0,
        ..Default::default()
    }).insert(GridShadowCamera); // Fix camera

    // Infinite grid
    commands.spawn(InfiniteGridBundle {
        grid: InfiniteGrid {
            fadeout_distance: 300.,
            shadow_color: None, // No shadow
            ..InfiniteGrid::default()
        },
        visibility: Visibility {
            is_visible: settings.show_gridlines,
        },
        ..InfiniteGridBundle::default()
    });
}

fn setup_args(
    _state: ResMut<AppState>,
    mut ev_update_state: EventWriter<AppFileEvent>,
) {
    let mut args = args().skip(1).collect::<Vec<String>>();
    if args.is_empty() {
        return;
    }

    ev_update_state.send(AppFileEvent::Open(args.remove(0).into()));
}

fn consume_file_events(
    mut file_events: EventReader<AppFileEvent>,
    mut app_event_writer: EventWriter<AppEvent>,
    mut state: ResMut<AppState>,
) {
    for e in file_events.iter() {
        match e {
            AppFileEvent::Open(file_path) => {
                //milo_event_writer.send(bevy::app::AppExit);
                open_file(file_path, &mut state, &mut app_event_writer);
            }
        }
    }
}

fn consume_app_events(
    mut app_events: EventReader<AppEvent>,
    mut bevy_event_writer: EventWriter<bevy::app::AppExit>,
    mut state: ResMut<AppState>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut textures: ResMut<Assets<Image>>,
    world_meshes: Query<(Entity, &WorldMesh)>,
) {
    for e in app_events.iter() {
        match e {
            AppEvent::Exit => {
                bevy_event_writer.send(bevy::app::AppExit);
            },
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
            /*AppEvent::RefreshMilo => {
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
            },*/
        }
    }
}

fn open_file(
    file_path: &PathBuf,
    state: &mut ResMut<AppState>,
    app_event_writer: &mut EventWriter<AppEvent>,
) {
    let ext = file_path.extension().unwrap().to_str().unwrap();

    if ext.contains("hdr") {
        // Open ark
        println!("Opening hdr from \"{}\"", file_path.display());

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
        println!("Opening milo from \"{}\"", file_path.display());

        match open_and_unpack_milo(file_path) {
            Ok((milo, info)) => {
                println!("Successfully opened milo with {} entries", milo.get_entries().len());

                state.milo = Some(milo);
                state.system_info = Some(info);

                //ev_update_state.send(AppEvent::RefreshMilo);

                const NAME_PREFS: [&str; 5] = ["venue", "top", "lod0", "lod1", "lod2"];

                let _groups = state.milo
                    .as_ref()
                    .unwrap()
                    .get_entries()
                    .iter()
                    .filter(|o| o.get_type() == "Group")
                    .collect::<Vec<_>>();

                let selected_entry = None;
                /*for name in NAME_PREFS {
                    let group = groups
                        .iter()
                        .find(|g| g.get_name().starts_with(name));

                    if let Some(grp) = group {
                        selected_entry = Some(grp.get_name().to_owned());
                        break;
                    }
                }*/

                app_event_writer.send(AppEvent::SelectMiloEntry(selected_entry));
            },
            Err(_err) => {
                // TODO: Log error
            }
        }
    } else {
        println!("Unknown file type \"{}\"", file_path.display());
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
    mut windows: ResMut<Windows>,
    mut settings: ResMut<AppSettings>,
    app_state: Res<AppState>,
    winit_windows: NonSend<WinitWindows>,
) {
    let primary_window = windows.primary_mut();
    let window = winit_windows.get_window(primary_window.id()).unwrap();
    let maximized = window.is_maximized();

    if settings.maximized != maximized {
        if maximized {
            println!("Window maximized");
        } else {
            println!("Window unmaximized");
        }

        settings.maximized = maximized;
        app_state.save_settings(&settings);
        return;
    }

    if maximized {
        // Ignore resize if maximized
        return;
    }

    for e in resize_events.iter() {
        println!("Window resized: {}x{}", e.width, e.height);

        settings.window_width = e.width;
        settings.window_height = e.height;
        app_state.save_settings(&settings);
    }
}

fn drop_files(
    mut drag_drop_events: EventReader<FileDragAndDrop>,
    mut file_event_writer: EventWriter<AppFileEvent>,
) {
    for d in drag_drop_events.iter() {
        if let FileDragAndDrop::DroppedFile { id: _, path_buf } = d {
            println!("Dropped \"{}\"", path_buf.to_str().unwrap());

            file_event_writer.send(AppFileEvent::Open(path_buf.to_owned()));
        }
    }
}

fn is_drop_event(dad_event: &FileDragAndDrop) -> bool {
    match dad_event {
        FileDragAndDrop::DroppedFile { id: _, path_buf: _ } => true,
        _ => false
    }
}