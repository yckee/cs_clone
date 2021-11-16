use crate::{GameState, loading::TextureAssets};
use bevy::prelude::*;


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_player.system()),
        )
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(anim_player.system()));
    }
}

fn spawn_player(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // commands
    //     .spawn_bundle(SpriteBundle{
    //         material: materials.add(textures.player.clone().into()),
    //         transform: Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
    //         ..Default::default()
    //     });
    
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: textures.player_stay.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.3, true));
    
}

fn anim_player(time: Res<Time>, mut query: Query<(&mut Timer, &mut TextureAtlasSprite)>) {
    for (mut timer, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            sprite.index = ((sprite.index as usize + 1) % 2) as u32;
        }
    }
}
