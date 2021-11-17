use crate::GameState;
use bevy::prelude::*;
use crate::consts::{ARENA_H, ARENA_W, PLAYER_TILE_SIZE};
pub struct PhysicsPlugin;

pub struct Velocity{
  pub v: Vec2
}

pub struct Gravity(pub f32);
pub struct AffectedByGravity;

impl Plugin for  PhysicsPlugin{
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(gravity_system.system())
                .with_system(velocity_system.system()),
        );
    }
}

fn gravity_system(
  gravity: Res<Gravity>,
  time: Res<Time>,
  mut velocities : Query <&mut Velocity, With<AffectedByGravity>>,
) {
  for mut velocity in  velocities.iter_mut() {
    velocity.v.y -= gravity.0 * time.delta_seconds();
    velocity.v.x = 150.0;
    
    velocity.v.y = velocity.v.y.clamp(            
      0.5 * (-ARENA_H + PLAYER_TILE_SIZE),
      2000.0
    );
  }  
}

fn velocity_system(
  time: Res<Time>,
  mut positions:Query<(&mut Transform, &mut Velocity)>)
   //mut velocities:Query<&mut Velocity>) 
   {
    let delta = time.delta_seconds();
    for (mut position, mut velocity) in positions.iter_mut() {
      position.translation.y = position.translation.y + velocity.v.y*delta; 

      position.translation.y = position.translation.y.clamp(            
        0.5 * (-ARENA_H + PLAYER_TILE_SIZE),
        0.5 * (ARENA_H - PLAYER_TILE_SIZE),
      );
  }
}