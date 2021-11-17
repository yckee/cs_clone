use crate::actions::Actions;
use crate::consts::{ARENA_H, ARENA_W, PLAYER_TILE_SIZE};
use crate::loading::TextureAssets;
use crate::GameState;
use crate::physics::*;
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
        .insert(Player)
        .insert(Velocity{v:Vec2::ZERO})
        .insert(AffectedByGravity);
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
    mut player_query: Query<(&mut Transform, &mut Velocity, With<Player>)>,
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

    for (mut player_transform, mut vel, p) in player_query.iter_mut() {

        if actions.player_movement.unwrap().y != 0.0 {
            println!("{:}",actions.player_movement.unwrap().y);
            vel.v.y = 400.0;
        }       

        player_transform.translation.x += actions.player_movement.unwrap().x * 150.0 * time.delta_seconds();

        player_transform.translation.x = player_transform.translation.x.clamp(
            0.5 * (-ARENA_W + PLAYER_TILE_SIZE),
            0.5 * (ARENA_W - PLAYER_TILE_SIZE),
          );
    }

    for (entity, mut anim, mut sprite) in player_anim.iter_mut() {
        sprite.flip_x = actions.player_movement.unwrap().x >= 0.0;

        if actions.player_movement.unwrap().y > 0.0 {
            if anim.anim != Animation::Jump{
                sprite.index = 0;
            }
            anim.anim = Animation::Jump;            
            anim.n_frames = 3;
            commands.entity(entity).insert(textures.player_jump.clone());
            continue;            
        }
        if  actions.player_movement.unwrap().x != 0.0 {
            anim.anim = Animation::Walk;
            anim.n_frames = 7;
            commands.entity(entity).insert(textures.player_walk.clone()); 
        }
    }

}
