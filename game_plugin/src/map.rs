use crate::consts::*;
use crate::loading::{MapAsset, MapAssets, TextureAssets};
use crate::GameState;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
// use bevy_rapier2d::prelude::*;
use heron::prelude::*;




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

fn convert_tilepos_to_global(tile_pos: TilePos, arena_size: Vec2, grid_size: Vec2, tile_size_x: f32, tile_size_y: f32) -> Vec2 {
    Vec2::new(
        tile_pos.0 as f32 / grid_size.x * arena_size.x - 0.5 * arena_size.x + (0.5 * tile_size_x),
        tile_pos.1 as f32 / grid_size.y * arena_size.y - 0.5 * arena_size.y + (0.5 * tile_size_y),
    )
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_camera.system())
                .with_system(spawn_map.system())
                .label("spawn")
            )
            ;
            // .add_system_set(
            //     SystemSet::on_enter(GameState::Playing)
            //         .after("spawn")
            // )
            // .add_system_set(
                // SystemSet::on_update(GameState::Playing)
                // .with_system(setup_tiles.system())
                // .with_system(test.system())
                // .with_system(rescale_map.system())

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
    println!("spawn_map");
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

    // layer_builder.set_all(TileBundle::default().into());


    let arena_size = Vec2::new(
        layer_settings.grid_size.x * layer_settings.tile_size.0,
        layer_settings.grid_size.y * layer_settings.tile_size.1,
    );

    // Vec2::splat(0.625);    

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
            let tile_type = TileType::get_tiletype_from_index(*tile_index);
            let tile_pixel_pos = convert_tilepos_to_global(tile_pos, arena_size, layer_settings.grid_size, layer_settings.tile_size.0, layer_settings.tile_size.0) * MAP_SCALE;

            let tile_entity = layer_builder.get_tile_entity(&mut commands, tile_pos).unwrap();
            // println!("{:?}",layer_builder.get_tile(tile_pos).unwrap().parent.chunk);
            commands.entity(tile_entity)
                .insert(tile_type)
                .insert(Transform {
                    scale: Vec3::splat(MAP_SCALE),
                    translation: tile_pixel_pos.extend(1.0),
                    ..Default::default()
                })
                .insert(GlobalTransform::default());

            if tile_type != TileType::Background {
                    commands.entity(tile_entity)
                        // .insert(GlobalTransform::default())
                        .insert(RigidBody::Static)
                        // .insert(CollisionShape::Sphere{ radius: 14.0});
                        .insert(CollisionShape::Cuboid {
                            half_extends: Vec3::new(10.0, 10.0, 0.0),
                            border_radius: None,
                        });
            }
        }
    }

    let layer_entity = map_query.build_layer(&mut commands, layer_builder, tileset_material_handle);

    map.add_layer(&mut commands, 0u16, layer_entity);

    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform {
            scale: Vec3::splat(MAP_SCALE),
            translation: Vec3::new(-center.x * MAP_SCALE, -center.y * MAP_SCALE, 1.0),
            ..Default::default()
        })
        // .insert(Transform::from_xyz(-center.x, -center.y, 0.0))
        .insert(GlobalTransform::default());
}

// fn setup_tiles(
//     mut commands: Commands,
//     map_q: Query<&Map>,
//     // map: Res<Map>,
//     mut tiles_q: Query<(Entity, &mut TilePos), Added<Tile>>
// ) {
//     // let map = map_q.iter().next().unwrap();
//     // map.
//     for (tile_entity, mut tile_pos) in tiles_q.iter_mut() {
//         // println!("111");
//         // let pix_tile_pos = convert_tilepos_to_global(tile_pos, arena_size, grid_size, tile_size_x, tile_size_y);
//         println!("{:?}", tile_pos);
//     }
// }

// fn rescale_map(
//     mut commands: Commands,
//     mut map_query: Query<&mut Transform, With<Map>>
// ) {
//     for mut transform in map_query.iter_mut() {
//         transform.scale = Vec3::new(0.625, 0.625, 0.625);
//     }
// }

// fn test(
//     q: Query<&Tile>
// ) {
//     println!("In test");
//     for body in q.iter(){
//         println!("{:?}", body.texture_index);
//         // println!("Have trnsfrom! C: {:?} Pos: {:?}", c, t.translation);
//     }
// }