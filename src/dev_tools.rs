//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{dev_tools::states::log_transitions, prelude::*};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use crate::game::map_change::{AssetData, Kare};
use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    // Print state transitions in dev builds
    app.add_systems(Update, log_transitions::<Screen>)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(ResourceInspectorPlugin::<Kare>::default())
        .init_resource::<Kare>()
        .init_resource::<AssetData>();
}
