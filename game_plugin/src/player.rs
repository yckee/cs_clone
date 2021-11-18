use crate::actions::Actions;
use crate::consts::{ARENA_H, ARENA_W, PLAYER_TILE_SIZE};
use crate::GameState;
use crate::physics::*;
use bevy::prelude::*;
use crate::loading::{TextureAssets};
use  crate::map::*;
use bevy::sprite::collide_aabb::collide;


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

pub struct  FinaMovement{
    movement: Vec3,
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
                .with_system(register_movement.system())
                .with_system(anim_player.system())
                .with_system(move_player.system())
                .with_system(change_anim.system()), 
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
        .insert(Velocity{vel:Vec2::ZERO})
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

fn register_movement(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&mut Velocity), With<Player>>,
){
    
    if actions.player_movement.is_none() { 
        for (mut velocity)in player_query.iter_mut() {        
            velocity.vel.x = 0.0;            
        }
        return;
    }    
    for (mut velocity) in player_query.iter_mut() {        
        velocity.vel.x = actions.player_movement.unwrap().x * 150.0 * time.delta_seconds();
        if actions.player_movement.unwrap().y != 0.0 {
            velocity.vel.y = 450.0;
        }    
    }
}

fn change_anim(
    mut commands: Commands,
    textures: Res<TextureAssets>, 
    mut player_anim: Query<(Entity, &mut PlayerAnim, &mut TextureAtlasSprite, &Velocity)>,
){
    for (entity,mut anim, mut sprite, velocity) in player_anim.iter_mut() {

        sprite.flip_x = velocity.vel.x >= 0.0;     

        if velocity.vel.y > 0.0 {
            if anim.anim != Animation::Jump{
                sprite.index = 0;
            }else if anim.anim == Animation::Jump {
                continue;
            }
            anim.anim = Animation::Jump;            
            anim.n_frames = 3;
            commands.entity(entity).insert(textures.player_jump.clone());
            continue;            
        }
        else if  velocity.vel.x != 0.0 {
            if anim.anim != Animation::Walk{
                sprite.index = 0;
            }else if anim.anim == Animation::Walk {
                continue;
            }
            anim.anim = Animation::Walk;
            anim.n_frames = 7;
            commands.entity(entity).insert(textures.player_walk.clone()); 
        }
        else {
            if anim.anim == Animation::Stay {
                continue;
            }
            anim.anim = Animation::Stay;
            anim.n_frames = 2;
            sprite.index = 0;
            commands.entity(entity).insert(textures.player_stay.clone());
        }
    }
}

fn collision(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform, &IntendentMovement), (With<Player>, Changed<IntendentMovement>)>,
    mut tile_qeury: Query<(&Transform, &MapTile), With<MapTile>>,
){
    let tile_size = Vec2::new(32.0, 32.0);
    let player_size = Vec2::new(64.0, 64.0);
    for (player_entity, player_transform, intendent_movement) in player_query.iter() {
        for (tile_transform, map_tile) in tile_qeury.iter() {
            if map_tile.tiletype!=TileType::Background {

                let temp_player_transform = player_transform.translation + intendent_movement.movement;

                if collide(tile_transform.translation, tile_size,temp_player_transform, player_size,).is_some() {
                    
                    let final_movement = Vec3::new(
                        0.0,
                        0.0,
                        0.,
                    );
                    commands.entity(player_entity).insert(FinaMovement{movement:final_movement});
                }
                // else {
                //     commands.entity(player_entity).insert(FinaMovement{movement:intendent_movement.movement});
                // }
            }
        }
    }
}

fn move_player(   
    mut player_query: Query<(&mut Transform, &IntendentMovement), (With<Player>, Changed<IntendentMovement>)>    
) { 
    for (mut player_transform, movement) in player_query.iter_mut() {
        println!("{:}", movement.movement);
        player_transform.translation += movement.movement;

        player_transform.translation.x = player_transform.translation.x.clamp(
            0.5 * (-ARENA_W + PLAYER_TILE_SIZE),
            0.5 * (ARENA_W - PLAYER_TILE_SIZE),
        );
        player_transform.translation.y = player_transform.translation.y.clamp(
            0.5 * (-ARENA_H + PLAYER_TILE_SIZE),
            0.5 * (ARENA_H - PLAYER_TILE_SIZE),
        );        
    }
}
