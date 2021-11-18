use crate::GameState;
use bevy::prelude::*;
pub struct PhysicsPlugin;

pub struct Velocity{
  pub vel: Vec2
}

pub struct IntendentMovement{
  pub movement: Vec3
}

// pub struct Velocity{
//   pub vel: Vec3
// }


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
    velocity.vel.y -= gravity.0 * time.delta_seconds();    
    //velocity.vel.x -= gravity.0 * time.delta_seconds();
  }  
}

fn velocity_system(
  mut commands: Commands,
  time: Res<Time>,
  mut positions:Query<(Entity, &Velocity), With<AffectedByGravity>>
  )  
  {
    
    let delta = time.delta_seconds();
      for (entity, velocity) in positions.iter_mut() {
        let movement = Vec3::new(
          velocity.vel.x,
          velocity.vel.y * delta,
          0.,
      );
      commands.entity(entity).insert(IntendentMovement{movement:movement});
      }
  }
