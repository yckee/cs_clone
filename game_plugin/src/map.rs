use crate::consts::*;
use crate::loading::{MapAsset, MapAssets, TextureAssets};
use crate::GameState;
use bevy::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TileType {
    Ground,
    Special,
    Lava,
    TreeGround,
    Floor,
    Background,
}

impl TileType {
    fn get_tiletype_from_index(ind: u32) -> TileType {
        match ind {
            4 => TileType::Floor,
            5 => TileType::Lava,
            9 => TileType::Ground,
            10 => TileType::Special,
            11 => TileType::TreeGround,
            _ => TileType::Background,
        }
    }
}

pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}
impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub struct MapTile {
    pub tiletype: TileType,
    pub position: Coordinate,
}

pub struct Map {
    pub size: Vec2,
    pub tile_size: Vec2,
    pub topology: Vec<Vec<u32>>,
    pub texture_atlas: Handle<TextureAtlas>,
    pub texture_tile_size: f32,
}

impl Map {
    fn new(
        size: Vec2,
        tile_size: Vec2,
        topology: Vec<Vec<u32>>,
        texture_atlas: Handle<TextureAtlas>,
        texture_tile_size: f32,
    ) -> Self {
        Self {
            size,
            tile_size,
            topology,
            texture_atlas,
            texture_tile_size,
        }
    }

    fn coordinate_to_pixel(&self, pos: &Coordinate, bound_w: f32, bound_h: f32) -> Vec2 {
        Vec2::new(
            pos.x as f32 / self.size.x * bound_w - 0.5 * bound_w + (0.5 * self.tile_size.x),
            0.5 * bound_h - pos.y as f32 / self.size.y * bound_h - (0.5 * self.tile_size.y),
        )
    }

    fn get_transform_scale(&self) -> Vec2 {
        Vec2::new(
            self.tile_size.x / self.texture_tile_size,
            self.tile_size.y / self.texture_tile_size,
        )
    }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_map.system())
                .with_system(spawn_camera.system()),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_map(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    maps: Res<MapAssets>,
    map_assets: Res<Assets<MapAsset>>,
) {
    let map_asset = map_assets
        .get(maps.map_one.clone())
        .expect("Failed to find MapAsset");

    let map = Map::new(
        Vec2::new(MAP_W, MAP_H),
        Vec2::new(ARENA_W / MAP_W, ARENA_H / MAP_H),
        map_asset.map.clone(),
        textures.tileset.clone(),
        32.0,
    );

    for (y, row) in map.topology.iter().enumerate() {
        for (x, tile_type) in row.iter().enumerate() {
            let coords = Coordinate::new(x, y);
            let pos = map.coordinate_to_pixel(&coords, ARENA_W, ARENA_H);
            let scale = map.get_transform_scale();

            commands
                .spawn_bundle(SpriteSheetBundle {
                    transform: Transform {
                        translation: pos.extend(1.0),
                        scale: scale.extend(1.0),
                        ..Default::default()
                    },
                    sprite: TextureAtlasSprite::new(*tile_type),
                    texture_atlas: textures.tileset.clone(),
                    ..Default::default()
                })
                .insert(MapTile {
                    tiletype: TileType::get_tiletype_from_index(*tile_type),
                    position: coords,
                });
        }
    }

    commands.insert_resource(map);
}
