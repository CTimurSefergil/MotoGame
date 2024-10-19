use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use rand::seq::SliceRandom;
use std::collections::HashMap;
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
    .init_resource::<Kare>()
    .init_resource::<AssetData>()
    .init_resource::<WFCRules>();
}

const TIME: Duration = Duration::from_millis(500);

#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
pub struct Block {
    pub id: i32,
}

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Kare {
    pub bir_siradaki_kare_sayisi: usize,
    pub kare_kenar_uzunlugu: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TileType {
    Wall,
    Tree,
    Column,
    Ground,
}

#[derive(Resource)]
pub struct WFCRules {
    allowed_neighbors: HashMap<TileType, Vec<TileType>>,
}

impl Default for WFCRules {
    fn default() -> Self {
        let mut allowed_neighbors = HashMap::new();
        allowed_neighbors.insert(
            TileType::Ground,
            vec![
                TileType::Ground,
                TileType::Wall,
                TileType::Tree,
                TileType::Column,
            ],
        );
        allowed_neighbors.insert(TileType::Wall, vec![TileType::Ground, TileType::Wall]);
        allowed_neighbors.insert(TileType::Tree, vec![TileType::Ground]);
        allowed_neighbors.insert(TileType::Column, vec![TileType::Ground]);
        Self { allowed_neighbors }
    }
}

#[derive(Resource)]
pub struct AssetData {
    assets: HashMap<TileType, String>,
}

impl Default for AssetData {
    fn default() -> Self {
        let mut assets = HashMap::new();
        assets.insert(TileType::Wall, "models/wall.glb#Scene0".to_string());
        assets.insert(TileType::Tree, "models/tree.glb#Scene0".to_string());
        assets.insert(TileType::Column, "models/column.glb#Scene0".to_string());
        assets.insert(TileType::Ground, "models/floor.glb#Scene0".to_string());
        Self { assets }
    }
}

fn create_tile_map(
    mut commands: Commands,
    kare: Res<Kare>,
    asset_data: Res<AssetData>,
    player: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    rules: Res<WFCRules>,
) {
    let Ok(player) = player.get_single() else {
        return;
    };

    let grid_size = kare.bir_siradaki_kare_sayisi;
    let mut grid = vec![vec![None; grid_size]; grid_size];

    for x in 0..grid_size {
        for z in 0..grid_size {
            let neighbors = [
                if x > 0 { grid[x - 1][z] } else { None },
                if z > 0 { grid[x][z - 1] } else { None },
                if x < grid_size - 1 {
                    grid[x + 1][z]
                } else {
                    None
                },
                if z < grid_size - 1 {
                    grid[x][z + 1]
                } else {
                    None
                },
            ];

            let tile_type = wfc_select_tile(&rules, &neighbors);
            grid[x][z] = Some(tile_type);

            let (world_x, world_z) = grid_to_world(x as f32, z as f32, kare.kare_kenar_uzunlugu);
            let (location_x, location_y, location_z) =
                calculate_tile_position(player, world_x, world_z, &kare);

            let asset = asset_data.assets.get(&tile_type).unwrap();

            commands.spawn((
                Name::new("Tile"),
                SceneBundle {
                    scene: asset_server.load(asset),
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
                Block { id: 1 },
            ));
        }
    }
}

fn update_tiles(
    mut commands: Commands,
    kare: Res<Kare>,
    asset_data: Res<AssetData>,
    player: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    rules: Res<WFCRules>,
    mut last_update: Local<Duration>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if !input.any_pressed([
        KeyCode::KeyW,
        KeyCode::ArrowUp,
        KeyCode::KeyS,
        KeyCode::ArrowDown,
        KeyCode::KeyA,
        KeyCode::ArrowLeft,
        KeyCode::KeyD,
        KeyCode::ArrowRight,
    ]) {
        return;
    }

    let now = time.elapsed();
    if *last_update + Duration::from_millis(500) > now {
        return;
    }
    *last_update = now;

    let Ok(player) = player.get_single() else {
        return;
    };

    let grid_size = kare.bir_siradaki_kare_sayisi;
    let mut grid = vec![vec![None; grid_size]; grid_size];

    for x in 0..grid_size {
        for z in 0..grid_size {
            let (world_x, world_z) = grid_to_world(x as f32, z as f32, kare.kare_kenar_uzunlugu);
            let (location_x, location_y, location_z) =
                calculate_tile_position(player, world_x, world_z, &kare);

            if is_tile_in_range(player, location_x, location_z, &kare) {
                let neighbors = [
                    if x > 0 { grid[x - 1][z] } else { None },
                    if z > 0 { grid[x][z - 1] } else { None },
                    if x < grid_size - 1 {
                        grid[x + 1][z]
                    } else {
                        None
                    },
                    if z < grid_size - 1 {
                        grid[x][z + 1]
                    } else {
                        None
                    },
                ];

                let tile_type = wfc_select_tile(&rules, &neighbors);
                grid[x][z] = Some(tile_type);

                let asset = asset_data.assets.get(&tile_type).unwrap();

                commands.spawn((
                    Name::new("Tile"),
                    SceneBundle {
                        scene: asset_server.load(asset),
                        transform: Transform::from_xyz(location_x, location_y, location_z)
                            .with_scale(Vec3::new(
                                kare.kare_kenar_uzunlugu as f32 / 4.0,
                                10.0,
                                kare.kare_kenar_uzunlugu as f32 / 4.0,
                            )),
                        ..default()
                    },
                    StateScoped(Screen::Playing),
                    Block { id: 1 },
                ));
            }
        }
    }
}

fn destroy_tiles(
    mut commands: Commands,
    kare: Res<Kare>,
    player: Query<&Transform, With<Player>>,
    block: Query<(Entity, &Transform), With<Block>>,
    mut last_sfx: Local<Duration>,
    time: Res<Time>,
) {
    let now = time.elapsed();

    let Ok(player) = player.get_single() else {
        return;
    };
    if *last_sfx + TIME < now {
        *last_sfx = now;
        for (entity, location) in block.iter() {
            if player.translation.distance(location.translation)
                > (kare.bir_siradaki_kare_sayisi as f32 * kare.kare_kenar_uzunlugu as f32) / 1.5
            {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn run_if_empty_map(query: Query<(), With<Block>>) -> bool {
    query.is_empty()
}

fn grid_to_world(x: f32, z: f32, kare_kenar_uzunlugu: usize) -> (f32, f32) {
    (
        x * kare_kenar_uzunlugu as f32,
        z * kare_kenar_uzunlugu as f32,
    )
}

fn wfc_select_tile(rules: &WFCRules, neighbors: &[Option<TileType>]) -> TileType {
    let mut possible_tiles: Vec<TileType> = vec![
        TileType::Ground,
        TileType::Wall,
        TileType::Tree,
        TileType::Column,
    ];

    for neighbor in neighbors.iter().flatten() {
        let allowed = rules.allowed_neighbors.get(neighbor).unwrap();
        possible_tiles.retain(|t| allowed.contains(t));
    }

    *possible_tiles
        .choose(&mut rand::thread_rng())
        .unwrap_or(&TileType::Ground)
}

fn calculate_tile_position(
    player: &Transform,
    world_x: f32,
    world_z: f32,
    kare: &Kare,
) -> (f32, f32, f32) {
    (
        world_x - (kare.kare_kenar_uzunlugu as f32 * kare.bir_siradaki_kare_sayisi as f32 / 2.0)
            + (((player.translation.x.round()) / kare.kare_kenar_uzunlugu as f32).round()
                * kare.kare_kenar_uzunlugu as f32),
        0.0,
        world_z - (kare.kare_kenar_uzunlugu as f32 * kare.bir_siradaki_kare_sayisi as f32 / 2.0)
            + (((player.translation.z.round()) / kare.kare_kenar_uzunlugu as f32).round()
                * kare.kare_kenar_uzunlugu as f32),
    )
}

fn is_tile_in_range(player: &Transform, location_x: f32, location_z: f32, kare: &Kare) -> bool {
    player
        .translation
        .distance(Vec3::new(location_x, 0.0, location_z))
        <= (kare.bir_siradaki_kare_sayisi as f32 / 4.5) * kare.kare_kenar_uzunlugu as f32
}
