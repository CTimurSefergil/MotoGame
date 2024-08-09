use bevy::{prelude::*, utils::hashbrown::HashSet};
use bevy_rapier3d::prelude::*;
use std::time::Duration;

use crate::screen::Screen;

use super::{spawn::player::Player, GameSystem};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            create_tile_map
                .in_set(GameSystem::MapChange)
                .run_if(run_if_empty_map),
            update_tiles.in_set(GameSystem::MapChange),
        )
            .chain(),
    );
}

const BIR_SIRADAKI_KARE_SAYISI: usize = 40;

const KARE_KENAR_UZUNLUGU: usize = 8;

const TIME: Duration = Duration::from_millis(2000);

const PATH: Vec<&'static str> = Vec::new();

#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
pub struct Map {
    pub id: i32,
}

fn create_tile_map(mut commands: Commands, player: Query<&Transform, With<Player>>) {
    let mut grids = HashSet::new();

    let Ok(player) = player.get_single() else {
        return;
    };

    for x in 1..BIR_SIRADAKI_KARE_SAYISI {
        for z in 1..BIR_SIRADAKI_KARE_SAYISI {
            grids.insert((x, z));
        }
    }

    for (en, (x, z)) in grids.iter().enumerate() {
        let (world_x, world_z) = grid_to_world(*x as f32, *z as f32);
        commands.spawn((
            Collider::cuboid(
                KARE_KENAR_UZUNLUGU as f32 / 2.0,
                0.0,
                KARE_KENAR_UZUNLUGU as f32 / 2.0,
            ),
            StateScoped(Screen::Playing),
            TransformBundle::from(Transform::from_xyz(
                world_x - (KARE_KENAR_UZUNLUGU as f32 * BIR_SIRADAKI_KARE_SAYISI as f32 / 2.0)
                    + (((player.translation.x.round()) / KARE_KENAR_UZUNLUGU as f32).round()
                        * KARE_KENAR_UZUNLUGU as f32),
                0.0,
                world_z - (KARE_KENAR_UZUNLUGU as f32 * BIR_SIRADAKI_KARE_SAYISI as f32 / 2.0)
                    + (((player.translation.z.round()) / KARE_KENAR_UZUNLUGU as f32).round()
                        * KARE_KENAR_UZUNLUGU as f32),
            )),
            Map { id: en as i32 },
        ));
    }
}

fn update_tiles(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    map: Query<(Entity, &Transform), With<Map>>,
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

        let mut grids = HashSet::new();

        if *last_sfx + TIME < now {
            *last_sfx = now;

            for x in 1..BIR_SIRADAKI_KARE_SAYISI {
                for z in 1..BIR_SIRADAKI_KARE_SAYISI {
                    grids.insert((x, z));
                }
            }

            for (en, (x, z)) in grids.iter().enumerate() {
                let (world_x, world_z) = grid_to_world(*x as f32, *z as f32);

                let (location_x, location_y, location_z) = (
                    world_x - (KARE_KENAR_UZUNLUGU as f32 * BIR_SIRADAKI_KARE_SAYISI as f32 / 2.0)
                        + (((player.translation.x.round()) / KARE_KENAR_UZUNLUGU as f32).round()
                            * KARE_KENAR_UZUNLUGU as f32),
                    0.0,
                    world_z - (KARE_KENAR_UZUNLUGU as f32 * BIR_SIRADAKI_KARE_SAYISI as f32 / 2.0)
                        + (((player.translation.z.round()) / KARE_KENAR_UZUNLUGU as f32).round()
                            * KARE_KENAR_UZUNLUGU as f32),
                );

                if player.translation.distance(Vec3 {
                    x: location_x,
                    y: location_y,
                    z: location_z,
                }) > (BIR_SIRADAKI_KARE_SAYISI as f32 / 4.0) * KARE_KENAR_UZUNLUGU as f32
                {
                    commands.spawn((
                        Collider::cuboid(
                            KARE_KENAR_UZUNLUGU as f32 / 2.0,
                            0.0,
                            KARE_KENAR_UZUNLUGU as f32 / 2.0,
                        ),
                        StateScoped(Screen::Playing),
                        TransformBundle::from(Transform::from_xyz(
                            location_x, location_y, location_z,
                        )),
                        Map {
                            //id: Vec3::new(world_x / 2.0, 0.0, world_z / 2.0),
                            id: en as i32,
                        },
                    ));
                }
            }

            for (entity, location) in map.iter() {
                if player.translation.distance(location.translation)
                    > BIR_SIRADAKI_KARE_SAYISI as f32 * KARE_KENAR_UZUNLUGU as f32
                {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

fn grid_to_world(x: f32, z: f32) -> (f32, f32) {
    (
        x * KARE_KENAR_UZUNLUGU as f32,
        z * KARE_KENAR_UZUNLUGU as f32,
    )
}

fn run_if_empty_map(query: Query<(), With<Map>>) -> bool {
    query.is_empty()
}
