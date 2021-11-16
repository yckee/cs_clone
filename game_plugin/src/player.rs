use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub enum Animation {
    Stay,
    Walk,
    Jump,
}

pub struct PlayerAnim {
    pub anim: Animation,
    pub n_frames: usize,
}

pub struct Player;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_player.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
            .with_system(anim_player.system())
            .with_system(move_player.system())
        );
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
    
    // commands
    //     .spawn_bundle(SpriteSheetBundle {
    //         sprite: TextureAtlasSprite::new(0),
    //         texture_atlas: textures.player_stay.clone(),
    //         transform: Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
    //         ..Default::default()
    //     })
    //     .insert(PlayerAnim{ anim: Animation::Stay, n_frames: 2})
    //     .insert(Timer::from_seconds(0.3, true));


    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: textures.player_walk.clone(),
            transform: Transform::from_translation(Vec3::new(-100.0, 0.0, 2.0)),
            ..Default::default()
        })
        .insert(PlayerAnim{ anim: Animation::Walk, n_frames: 8})
        .insert(Timer::from_seconds(0.3, true))
        .insert(Player);
    
}

fn anim_player(time: Res<Time>, mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &PlayerAnim)>) {
    for (mut timer, mut sprite, player_anim) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            sprite.index = ((sprite.index as usize + 1) % player_anim.n_frames) as u32;
        }
    }
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_transform in player_query.iter_mut() {
        player_transform.translation += movement;
    }
}