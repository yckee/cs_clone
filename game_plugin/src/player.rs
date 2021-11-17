use crate::actions::Actions;
use crate::consts::{ARENA_H, ARENA_W, PLAYER_TILE_SIZE};
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
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
            SystemSet::on_enter(GameState::Playing).with_system(spawn_player.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(anim_player.system())
                .with_system(move_player.system()),
        );
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: textures.player_stay.clone(),
            transform: Transform::from_translation(Vec3::new(-100.0, 0.0, 2.0)),
            ..Default::default()
        })
        .insert(PlayerAnim {
            anim: Animation::Stay,
            n_frames: 2,
        })
        .insert(Timer::from_seconds(0.2, true))
        .insert(Player);
}

fn anim_player(
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &PlayerAnim)>,
) {
    for (mut timer, mut sprite, player_anim) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            sprite.index = ((sprite.index as usize + 1) % player_anim.n_frames) as u32;
        }
    }
}

fn move_player(
    mut commands: Commands,
    time: Res<Time>,
    actions: Res<Actions>,
    textures: Res<TextureAssets>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut player_anim: Query<(Entity, &mut PlayerAnim, &mut TextureAtlasSprite)>,
) {
    if actions.player_movement.is_none() {
        for (entity, mut anim, mut sprite) in player_anim.iter_mut() {
            if anim.anim == Animation::Stay {
                continue;
            }
            anim.anim = Animation::Stay;
            anim.n_frames = 2;
            sprite.index = 0;
            commands.entity(entity).insert(textures.player_stay.clone());
        }
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
        player_transform.translation.x = player_transform.translation.x.clamp(
            0.5 * (-ARENA_W + PLAYER_TILE_SIZE),
            0.5 * (ARENA_W - PLAYER_TILE_SIZE),
        );
        player_transform.translation.y = player_transform.translation.y.clamp(
            0.5 * (-ARENA_H + PLAYER_TILE_SIZE),
            0.5 * (ARENA_H - PLAYER_TILE_SIZE),
        );
    }

    for (entity, mut anim, mut sprite) in player_anim.iter_mut() {
        if anim.anim == Animation::Walk {
            sprite.flip_x = movement.x > 0.0;
            continue;
        }

        anim.anim = Animation::Walk;
        anim.n_frames = 7;
        commands.entity(entity).insert(textures.player_walk.clone());
    }
}
