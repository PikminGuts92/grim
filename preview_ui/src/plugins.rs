use bevy::{prelude::*, log::LogPlugin, app::PluginGroupBuilder};

struct MinimalPlugins;

pub struct GrimPlugin;

// Using until bevy_fly_camera is updated
// https://github.com/mcpar-land/bevy_fly_camera/pull/19
impl PluginGroup for MinimalPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(bevy::log::LogPlugin::default())
            .add(bevy::core::CorePlugin::default())
            .add(bevy::transform::TransformPlugin::default())
            .add(bevy::diagnostic::DiagnosticsPlugin::default())
            .add(bevy::input::InputPlugin::default())
            .add(bevy::window::WindowPlugin::default())
            .add(bevy::asset::AssetPlugin::default())
            .add(bevy::scene::ScenePlugin::default());

        // Optional features being used
        group
            .add(bevy::winit::WinitPlugin::default())
            .add(bevy::render::RenderPlugin::default())
            .add(bevy::core_pipeline::CorePipelinePlugin::default())
            .add(bevy::pbr::PbrPlugin::default());
    }
}

impl Plugin for GrimPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_plugins(DefaultPlugins);
            .add_plugins(MinimalPlugins);
    }
}