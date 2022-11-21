use bevy::{prelude::*, log::LogPlugin, app::PluginGroupBuilder, window::{PresentMode, WindowMode, WindowResized}};
use crate::settings::*;
use crate::state::*;
use std::{env::args, path::{Path, PathBuf}};

const SETTINGS_FILE_NAME: &str = "settings.json";
const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

struct MinimalPlugins;

pub struct GrimPlugin;

// Using until bevy_fly_camera is updated
// https://github.com/mcpar-land/bevy_fly_camera/pull/19
impl PluginGroup for MinimalPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            // Basic stuff
            .add(bevy::log::LogPlugin::default())
            .add(bevy::core::CorePlugin::default())
            .add(bevy::time::TimePlugin::default())
            .add(bevy::transform::TransformPlugin::default())
            .add(bevy::hierarchy::HierarchyPlugin::default())
            .add(bevy::diagnostic::DiagnosticsPlugin::default())
            .add(bevy::input::InputPlugin::default())
            .add(bevy::window::WindowPlugin::default())
            // Optional features being used
            .add(bevy::asset::AssetPlugin::default())
            .add(bevy::scene::ScenePlugin::default())
            .add(bevy::winit::WinitPlugin::default())
            .add(bevy::render::RenderPlugin::default())
            .add(bevy::render::texture::ImagePlugin::default())
            .add(bevy::core_pipeline::CorePipelinePlugin::default())
            .add(bevy::pbr::PbrPlugin::default())
    }
}

impl Plugin for GrimPlugin {
    fn build(&self, app: &mut App) {
        // Load settings
        #[cfg(target_family = "wasm")] std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        #[cfg(target_family = "wasm")] let app_state = AppState::default();
        #[cfg(target_family = "wasm")] let app_settings = AppSettings::default();

        #[cfg(not(target_family = "wasm"))] let app_state = load_state();
        #[cfg(not(target_family = "wasm"))] let app_settings = load_settings(&app_state.settings_path);

        app
            //.add_plugins(DefaultPlugins);
            .add_plugins(MinimalPlugins.set(WindowPlugin {
                window: WindowDescriptor {
                    title: format!("Preview v{}", VERSION),
                    width: app_settings.window_width,
                    height: app_settings.window_height,
                    mode: WindowMode::Windowed,
                    present_mode: PresentMode::Fifo, // vsync
                    resizable: true,
                    ..Default::default()
                },
                ..Default::default()
            }))
            .insert_resource(app_state)
            .insert_resource(app_settings);
    }
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