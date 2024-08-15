//! Camera setup.

use bevy::pbr::ClusterConfig;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.1, 1.0),
            ..Default::default()
        },
        IsDefaultUiCamera,
        ClusterConfig::Single,
    ));
}
