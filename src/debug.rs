use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::env;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            // World inspector plugin
            let inspector_enabled = env::args().any(|arg| arg == "--inspector");
            if inspector_enabled {
                app.add_plugins(WorldInspectorPlugin::new());
            }
        }
    }
}
