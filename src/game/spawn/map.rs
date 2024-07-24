use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_map);
}

#[derive(Event, Debug)]
pub struct SpawnMap;

fn spawn_map(_trigger: Trigger<SpawnMap>, mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn((
            Collider::cuboid(100.0, 0.1, 100.0),
            StateScoped(Screen::Playing),
        ))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn((RigidBody::Dynamic, StateScoped(Screen::Playing)))
        .insert(Collider::ball(2.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 16.0, -10.0)));
}
