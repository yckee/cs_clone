use crate::actions::Actions;
use crate::collision::Collider;
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

pub struct IntendedMovement(pub Vec3);
pub struct FinalMovement(pub Vec3);


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
                .label("register_movemnt")
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(change_anim.system())
                .label("change_animation")
                .after("register_movemnt")
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(anim_player.system())
                .with_system(move_player.system())
                .after("change_animation")
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
        .insert(Collider::Solid)
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

fn register_movement(
    mut commands: Commands,
    time: Res<Time>,
    actions: Res<Actions>,
    player_query: Query<Entity, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }

    let speed = 150.;
    let movement = Vec3::new(
        (actions.player_movement.unwrap().x * speed * time.delta_seconds()).round(),
        (actions.player_movement.unwrap().y * speed * time.delta_seconds()).round(),
        0.,
    );

    for player in player_query.iter() {
        commands.entity(player).insert(IntendedMovement(movement));
    }
}


fn change_anim (
    mut commands: Commands,
    actions: Res<Actions>,
    textures: Res<TextureAssets>,
    mut player_query: Query<(Entity, &mut PlayerAnim, &mut TextureAtlasSprite, &FinalMovement)>,
) {
    for (player, mut player_anim, mut sprite, movement) in player_query.iter_mut() {
        if actions.player_movement.is_none(){
            if player_anim.anim == Animation::Stay {
                continue;
            } else {
                player_anim.anim = Animation::Stay;
                player_anim.n_frames = 2;
                sprite.index = 0;
                commands.entity(player).insert(textures.player_stay.clone());
            }
        } else {
            sprite.flip_x = movement.0.x >= 0.0;

            if movement.0.y > 0.0 {
                if player_anim.anim != Animation::Jump {
                    sprite.index = 0;
                }
                player_anim.anim = Animation::Jump;
                player_anim.n_frames = 3;
                commands.entity(player).insert(textures.player_jump.clone());
                continue;
            }

            if movement.0.x != 0.0 {
                player_anim.anim = Animation::Walk;
                player_anim.n_frames = 7;
                commands.entity(player).insert(textures.player_walk.clone());
            }   
        }
    }
}

fn move_player(
    mut commands: Commands,
    actions: Res<Actions>,
    mut player_query: Query<(Entity, &mut Transform, &FinalMovement), (With<Player>, Changed<FinalMovement>)>,

) {
    if actions.player_movement.is_none() {
        return;
    }

    
    for (player, mut player_transform, movement) in player_query.iter_mut() {
        println!("{:?}", movement.0);
        player_transform.translation += movement.0;
        player_transform.translation.x = player_transform.translation.x.clamp(
            0.5 * (-ARENA_W + PLAYER_TILE_SIZE),
            0.5 * (ARENA_W - PLAYER_TILE_SIZE),
        );
        player_transform.translation.y = player_transform.translation.y.clamp(
            0.5 * (-ARENA_H + PLAYER_TILE_SIZE),
            0.5 * (ARENA_H - PLAYER_TILE_SIZE),
        );

        commands.entity(player).remove::<FinalMovement>();
    }
}
