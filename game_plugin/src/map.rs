use crate::consts::*;
use crate::loading::{MapAsset, MapAssets, TextureAssets};
use crate::GameState;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;



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

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_map.system())
                .with_system(spawn_camera.system()),
        );
        // .add_system_set(
        //     SystemSet::on_update(GameState::Playing)
        //     .with_system(rescale_map.system())
        // );
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
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
) {
    let mut map_asset = map_assets
        .get(maps.map_one.clone())
        .expect("Failed to find MapAsset");

    let tileset_material_handle = materials.add(ColorMaterial::texture(textures.tileset.clone()));

    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let layer_settings = LayerSettings::new(
        MapSize(2, 2),
        ChunkSize(16, 16),
        TileSize(32.0, 32.0),
        TextureSize(192.0, 64.0),
    );
    let center = layer_settings.get_pixel_center();

    let (mut layer_builder, _) = LayerBuilder::<TileBundle>::new(
        &mut commands,
        layer_settings,
        0u16,
        0u16,
        None,
    );

    layer_builder.set_all(TileBundle::default());

    for (y, row) in map_asset.map.iter().enumerate() {
        for (x, tile_index) in row.iter().enumerate() {
            let tile_pos = TilePos(x as u32, y as u32);
            let _ = layer_builder.set_tile(
                tile_pos, 
                Tile{
                    texture_index: *tile_index as u16,
                    ..Default::default()
                }.into()
            );

            let tile_entity = layer_builder.get_tile_entity(&mut commands, tile_pos).unwrap();
            commands.entity(tile_entity).insert(TileType::get_tiletype_from_index(*tile_index));
        }
    }

    let layer_entity = map_query.build_layer(&mut commands, layer_builder, tileset_material_handle);

    map.add_layer(&mut commands, 0u16, layer_entity);

    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform {
            scale: Vec3::new(0.625, 0.625, 0.625),
            translation: Vec3::new(-center.x * 0.625, -center.y * 0.625, 0.0),
            ..Default::default()
        })
        // .insert(Transform::from_xyz(-center.x, -center.y, 0.0))
        .insert(GlobalTransform::default());
}

// fn rescale_map(
//     mut commands: Commands,
//     mut map_query: Query<&mut Transform, With<Map>>
// ) {
//     for mut transform in map_query.iter_mut() {
//         transform.scale = Vec3::new(0.625, 0.625, 0.625);
//     }
// }