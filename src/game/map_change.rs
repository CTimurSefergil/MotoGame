use bevy::{prelude::*, utils::hashbrown::HashSet};
use bevy_rapier3d::prelude::*;
use std::time::Duration;

use crate::screen::Screen;

use super::{spawn::player::Player, GameSystem};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            create_tile_map.in_set(GameSystem::MapChange),
            destroy_tiles.in_set(GameSystem::MapChange),
        )
            .chain(),
    );
}

const GRID_X: usize = 50;
const GRID_Z: usize = 50;

const TILE_W: usize = 12;
const TILE_H: usize = 12;

const TIME: Duration = Duration::from_millis(500);

#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
pub struct Map {
    pub id: i32,
}

fn create_tile_map(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    mut last_sfx: Local<Duration>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.pressed(KeyCode::KeyW)
        || input.pressed(KeyCode::ArrowUp)
        || input.pressed(KeyCode::KeyS)
        || input.pressed(KeyCode::ArrowDown)
        || input.pressed(KeyCode::KeyA)
        || input.pressed(KeyCode::ArrowLeft)
        || input.pressed(KeyCode::KeyD)
        || input.pressed(KeyCode::ArrowRight)
    {
        let mut grids = HashSet::new();

        let now = time.elapsed();

        let Ok(player) = player.get_single() else {
            return;
        };

        if *last_sfx + TIME < now {
            *last_sfx = now;

            for x in 0..GRID_X {
                for z in 0..GRID_Z {
                    grids.insert((x, z));
                }
            }

            for (en, (x, z)) in grids.iter().enumerate() {
                let (world_x, world_z) = grid_to_world(*x as f32, *z as f32);

                //if önceden dolu değilse {
                commands.spawn((
                    Collider::cuboid(TILE_W as f32 / 2.0, 0.0, TILE_H as f32 / 2.0),
                    StateScoped(Screen::Playing),
                    TransformBundle::from(Transform::from_xyz(
                        world_x
                            + GRID_X as f32
                            + (((player.translation.x.round()) / TILE_W as f32).round()
                                * TILE_W as f32)
                            - ((GRID_X as f32 + 2.0) / 2.0) * TILE_W as f32,
                        0.0,
                        world_z
                            + GRID_Z as f32
                            + (((player.translation.z.round()) / TILE_H as f32).round()
                                * TILE_H as f32)
                            - ((GRID_Z as f32 + 2.0) / 2.0) * TILE_H as f32,
                    )),
                    Map {
                        //id: Vec3::new(world_x / 2.0, 0.0, world_z / 2.0),
                        id: en as i32,
                    },
                ));
            }
        }
    }
}
//}

fn destroy_tiles(
    mut commands: Commands,
    query: Query<(&Transform, Entity), With<Map>>,
    player: Query<&Transform, With<Player>>,
    mut last_sfx: Local<Duration>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.pressed(KeyCode::KeyW)
        || input.pressed(KeyCode::ArrowUp)
        || input.pressed(KeyCode::KeyS)
        || input.pressed(KeyCode::ArrowDown)
        || input.pressed(KeyCode::KeyA)
        || input.pressed(KeyCode::ArrowLeft)
        || input.pressed(KeyCode::KeyD)
        || input.pressed(KeyCode::ArrowRight)
    {
        let now = time.elapsed();

        let Ok(player) = player.get_single() else {
            return;
        };

        if *last_sfx + TIME < now {
            *last_sfx = now;
            for (location, entity) in query.iter() {
                if player.translation.distance(location.translation)
                    > GRID_Z as f32 * 2_f32.sqrt() * 2.0
                {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

fn is_free(tile: Query<&Transform, With<Map>>) {}

fn grid_to_world(x: f32, z: f32) -> (f32, f32) {
    (x * TILE_W as f32, z * TILE_H as f32)
}
