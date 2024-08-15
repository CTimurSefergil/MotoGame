use bevy::{prelude::*, utils::hashbrown::HashSet};
use bevy_inspector_egui::prelude::*;
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
            destroy_tiles,
        )
            .chain(),
    )
    .insert_resource(Kare {
        bir_siradaki_kare_sayisi: 15,
        kare_kenar_uzunlugu: 8,
    });
}

const TIME: Duration = Duration::from_millis(500);

#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
pub struct Map {
    pub id: i32,
}
#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Kare {
    pub bir_siradaki_kare_sayisi: usize,
    pub kare_kenar_uzunlugu: usize,
}

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub enum Asset {
    Wall,
    Tree,
    #[default]
    Column,
}

#[derive(Resource, Reflect, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct AssetData {
    asset: Asset,
}

fn create_tile_map(
    mut commands: Commands,
    kare: ResMut<Kare>,
    asset: ResMut<AssetData>,
    player: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    let mut grids = HashSet::new();

    let Ok(player) = player.get_single() else {
        return;
    };

    for (x, z) in (0..kare.bir_siradaki_kare_sayisi)
        .flat_map(|x| (0..kare.bir_siradaki_kare_sayisi).map(move |z| (x, z)))
    {
        grids.insert((x, z));
    }

    for (x, z) in grids.iter() {
        let assets = match asset.asset {
            Asset::Wall => "models/Wall.glb#Scene0",
            Asset::Column => "models/column.glb#Scene0",
            Asset::Tree => "models/tree.glb#Scene0",
        };

        let (world_x, world_z) = grid_to_world(*x as f32, *z as f32, kare.kare_kenar_uzunlugu);

        let (location_x, location_y, location_z) = (
            world_x
                - (kare.kare_kenar_uzunlugu as f32 * kare.bir_siradaki_kare_sayisi as f32 / 2.0)
                + (((player.translation.x.round()) / kare.kare_kenar_uzunlugu as f32).round()
                    * kare.kare_kenar_uzunlugu as f32),
            0.0,
            world_z
                - (kare.kare_kenar_uzunlugu as f32 * kare.bir_siradaki_kare_sayisi as f32 / 2.0)
                + (((player.translation.z.round()) / kare.kare_kenar_uzunlugu as f32).round()
                    * kare.kare_kenar_uzunlugu as f32),
        );

        commands.spawn((
            Name::new("Kare"),
            SceneBundle {
                scene: asset_server.load(assets),
                transform: Transform::from_xyz(location_x, location_y, location_z).with_scale(
                    Vec3::new(
                        kare.kare_kenar_uzunlugu as f32 / 4.0,
                        10.0,
                        kare.kare_kenar_uzunlugu as f32 / 4.0,
                    ),
                ),
                ..default()
            },
            StateScoped(Screen::Playing),
            Map { id: 1 },
        ));

        commands.spawn((
            Name::new("Kare"),
            SceneBundle {
                scene: asset_server.load(assets),
                transform: Transform::from_xyz(
                    location_x + kare.kare_kenar_uzunlugu as f32 / 2.0,
                    location_y,
                    location_z,
                )
                .with_scale(Vec3::new(
                    kare.kare_kenar_uzunlugu as f32 / 4.0,
                    10.0,
                    kare.kare_kenar_uzunlugu as f32 / 4.0,
                )),
                ..default()
            },
            StateScoped(Screen::Playing),
            Map { id: 1 },
        ));

        commands.spawn((
            Name::new("Kare"),
            SceneBundle {
                scene: asset_server.load(assets),
                transform: Transform::from_xyz(
                    location_x,
                    location_y,
                    location_z + kare.kare_kenar_uzunlugu as f32 / 2.0,
                )
                .with_scale(Vec3::new(
                    kare.kare_kenar_uzunlugu as f32 / 4.0,
                    10.0,
                    kare.kare_kenar_uzunlugu as f32 / 4.0,
                )),
                ..default()
            },
            StateScoped(Screen::Playing),
            Map { id: 1 },
        ));

        commands.spawn((
            Name::new("Kare"),
            SceneBundle {
                scene: asset_server.load(assets),
                transform: Transform::from_xyz(
                    location_x + kare.kare_kenar_uzunlugu as f32 / 2.0,
                    location_y,
                    location_z + kare.kare_kenar_uzunlugu as f32 / 2.0,
                )
                .with_scale(Vec3::new(
                    kare.kare_kenar_uzunlugu as f32 / 4.0,
                    10.0,
                    kare.kare_kenar_uzunlugu as f32 / 4.0,
                )),
                ..default()
            },
            StateScoped(Screen::Playing),
            Map { id: 1 },
        ));
    }
}

fn update_tiles(
    mut commands: Commands,
    kare: ResMut<Kare>,
    asset: ResMut<AssetData>,
    player: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
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

        let mut grids = HashSet::new();

        let Ok(player) = player.get_single() else {
            return;
        };

        if *last_sfx + TIME < now {
            *last_sfx = now;

            for (x, z) in (0..kare.bir_siradaki_kare_sayisi)
                .flat_map(|x| (0..kare.bir_siradaki_kare_sayisi).map(move |z| (x, z)))
            {
                grids.insert((x, z));
            }

            for (x, z) in grids.iter() {
                let assets = match asset.asset {
                    Asset::Wall => "models/wall.glb#Scene0",
                    Asset::Column => "models/column.glb#Scene0",
                    Asset::Tree => "models/tree.glb#Scene0",
                };

                let (world_x, world_z) =
                    grid_to_world(*x as f32, *z as f32, kare.kare_kenar_uzunlugu);

                let (location_x, location_y, location_z) = (
                    world_x
                        - (kare.kare_kenar_uzunlugu as f32 * kare.bir_siradaki_kare_sayisi as f32
                            / 2.0)
                        + (((player.translation.x.round()) / kare.kare_kenar_uzunlugu as f32)
                            .round()
                            * kare.kare_kenar_uzunlugu as f32),
                    0.0,
                    world_z
                        - (kare.kare_kenar_uzunlugu as f32 * kare.bir_siradaki_kare_sayisi as f32
                            / 2.0)
                        + (((player.translation.z.round()) / kare.kare_kenar_uzunlugu as f32)
                            .round()
                            * kare.kare_kenar_uzunlugu as f32),
                );

                if player.translation.distance(Vec3 {
                    x: location_x,
                    y: location_y,
                    z: location_z,
                }) > (kare.bir_siradaki_kare_sayisi as f32 / 4.5)
                    * kare.kare_kenar_uzunlugu as f32
                {
                    commands.spawn((
                        Name::new("Kare"),
                        SceneBundle {
                            scene: asset_server.load(assets),
                            transform: Transform::from_xyz(location_x, location_y, location_z)
                                .with_scale(Vec3::new(
                                    kare.kare_kenar_uzunlugu as f32 / 4.0,
                                    10.0,
                                    kare.kare_kenar_uzunlugu as f32 / 4.0,
                                )),
                            ..default()
                        },
                        StateScoped(Screen::Playing),
                        Map { id: 1 },
                    ));

                    commands.spawn((
                        Name::new("Kare"),
                        SceneBundle {
                            scene: asset_server.load(assets),
                            transform: Transform::from_xyz(
                                location_x + kare.kare_kenar_uzunlugu as f32 / 2.0,
                                location_y,
                                location_z,
                            )
                            .with_scale(Vec3::new(
                                kare.kare_kenar_uzunlugu as f32 / 4.0,
                                10.0,
                                kare.kare_kenar_uzunlugu as f32 / 4.0,
                            )),
                            ..default()
                        },
                        StateScoped(Screen::Playing),
                        Map { id: 1 },
                    ));

                    commands.spawn((
                        Name::new("Kare"),
                        SceneBundle {
                            scene: asset_server.load(assets),
                            transform: Transform::from_xyz(
                                location_x,
                                location_y,
                                location_z + kare.kare_kenar_uzunlugu as f32 / 2.0,
                            )
                            .with_scale(Vec3::new(
                                kare.kare_kenar_uzunlugu as f32 / 4.0,
                                10.0,
                                kare.kare_kenar_uzunlugu as f32 / 4.0,
                            )),
                            ..default()
                        },
                        StateScoped(Screen::Playing),
                        Map { id: 1 },
                    ));

                    commands.spawn((
                        Name::new("Kare"),
                        SceneBundle {
                            scene: asset_server.load(assets),
                            transform: Transform::from_xyz(
                                location_x + kare.kare_kenar_uzunlugu as f32 / 2.0,
                                location_y,
                                location_z + kare.kare_kenar_uzunlugu as f32 / 2.0,
                            )
                            .with_scale(Vec3::new(
                                kare.kare_kenar_uzunlugu as f32 / 4.0,
                                10.0,
                                kare.kare_kenar_uzunlugu as f32 / 4.0,
                            )),
                            ..default()
                        },
                        StateScoped(Screen::Playing),
                        Map { id: 1 },
                    ));
                }
            }
        }
    }
}

fn destroy_tiles(
    mut commands: Commands,
    kare: ResMut<Kare>,
    player: Query<&Transform, With<Player>>,
    map: Query<(Entity, &Transform), With<Map>>,
    mut last_sfx: Local<Duration>,
    time: Res<Time>,
) {
    let now = time.elapsed();

    let Ok(player) = player.get_single() else {
        return;
    };
    if *last_sfx + TIME < now {
        *last_sfx = now;
        for (entity, location) in map.iter() {
            if player.translation.distance(location.translation)
                > (kare.bir_siradaki_kare_sayisi as f32 * kare.kare_kenar_uzunlugu as f32) / 1.5
            {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn run_if_empty_map(query: Query<(), With<Map>>) -> bool {
    query.is_empty()
}

fn grid_to_world(x: f32, z: f32, kare_kenar_uzunlugu: usize) -> (f32, f32) {
    (
        x * kare_kenar_uzunlugu as f32,
        z * kare_kenar_uzunlugu as f32,
    )
}
