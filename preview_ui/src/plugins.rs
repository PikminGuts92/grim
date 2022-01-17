use bevy::{prelude::*, log::LogPlugin};
#[cfg(feature = "dev")] use bevy_remote_devtools_plugin::RemoteDevToolsPlugin;

pub struct GrimPlugin;

impl Plugin for GrimPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(not(feature = "dev"))]
        app
            .add_plugins(DefaultPlugins);

        #[cfg(feature = "dev")]
        app
            .add_plugin(RemoteDevToolsPlugin::new("Grim Preview", 3030))
            .add_plugins_with(DefaultPlugins, |grp| grp.disable::<LogPlugin>());
    }
}