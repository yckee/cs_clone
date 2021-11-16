// TODO: rewrite to use struck for Map and/or MapTile. Get right size and pos of the tile sprite

use crate::loading::{MapAsset, MapAssets, TextureAssets};
use crate::GameState;
use bevy::prelude::*;


const MAP_W: f32 = 32.0;
const MAP_H: f32 = 32.0;


fn index_to_pos(x: f32, y:f32, window_x:f32, window_y:f32) -> Vec2 {
    Vec2::new(
        x / MAP_W * window_x  - 0.5 * window_x + (0.5 * window_x / MAP_W),
        0.5 * window_y - y / MAP_H * window_y - (0.5 * window_y / MAP_H),
    )
}


pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_map.system())
                .with_system(spawn_camera.system())
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
    texture_atlases: Res<Assets<TextureAtlas>>,
    ass_maps: Res<Assets<MapAsset>>,
    windows: Res<Windows>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    let mass = ass_maps
        .get(maps.map_one.clone())
        .expect("Failed to find MapAsset");

    for (y, row) in mass.map.iter().enumerate(){
        for (x, tile_type) in row.iter().enumerate(){
            let window = windows.get_primary().unwrap();
            let pos = index_to_pos(x as f32, y as f32, window.width(), window.height());
            
            commands
                .spawn_bundle(SpriteSheetBundle {
                    transform: Transform {
                        translation: pos.extend(1.0),
                        // scale: Vec3::new(0.5, 0.5, 0.5),
                        ..Default::default()
                    },
                    sprite: TextureAtlasSprite::new(*tile_type),
                    texture_atlas: textures.tileset.clone(),
                    ..Default::default()
                });

        }
    }
    // commands
    // .spawn_bundle(SpriteSheetBundle {
    //     transform: Transform {
    //         translation: Vec3::ZERO,
    //         // scale: Vec3::new(0.5, 0.5, 0.5),
    //         ..Default::default()
    //     },
    //     sprite: TextureAtlasSprite::new(4),
    //     texture_atlas: textures.tileset.clone(),
    //     ..Default::default()
    // });
       
}
