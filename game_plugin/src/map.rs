use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

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
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let atlas = texture_atlases
        .get(textures.simple_background.clone())
        .expect("Failed to find atlas");

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(atlas.texture.clone().into()),
        transform: Transform::from_xyz(0.0, -100.0, 1.0),
        ..Default::default()
    });

    commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(0., 150., 0.),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: textures.simple_background.clone(),
            ..Default::default()
        });

}