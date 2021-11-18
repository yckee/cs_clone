use crate::consts::PLAYER_TILE_SIZE;
use crate::map::{MapTile, Map};
use crate::player::{IntendedMovement, FinalMovement};
use crate::GameState;
use bevy::{prelude::*, sprite::collide_aabb::{collide, Collision}};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Collider {
    Solid,
    Phantom,
}
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(player_movement_map_collision.system())
        );
    }
}


fn player_movement_map_collision(
    mut commands: Commands,
    map: Res<Map>,
    mut player_query: Query<(Entity,&Transform, &mut IntendedMovement), Added<IntendedMovement>>,
    collider_query: Query<(&Collider, &Transform), With<MapTile>>
) {
    for (player, player_transform, mut intended_movement) in player_query.iter_mut() {
        let player_size = Vec2::new(PLAYER_TILE_SIZE, PLAYER_TILE_SIZE * 0.5);
        
        for (collider, tile_transform) in collider_query.iter(){
            // // println!("colision check in tile query");
            // if *collider != Collider::Solid {
            //     continue;
            // }
            // let future_tranform = player_transform.translation + intended_movement.0;
            // let collision = collide(
            //     tile_transform.translation,
            //     map.tile_size,
            //     future_tranform,
            //     player_size,
            // ); 
            // if let Some(collision) = collision {
            //     println!("{:?}", collision);
            // }

            // loop {
            //     let future_tranform = player_transform.translation + intended_movement.0;
            //     let collision = collide(
            //         future_tranform,
            //         player_size,
            //         tile_transform.translation,
            //         map.tile_size
            //     ); 
            //     println!("Future: {:?} | Intended: {:?} | Collision: {:?}", future_tranform, intended_movement.0, collision);
                
            //     if let Some(collision) = collision {
            //         match collision {
            //             Collision::Left => intended_movement.0.x += 1.0,
            //             Collision::Right => intended_movement.0.x -= 1.0,
            //             Collision::Top => intended_movement.0.y -= 1.0,
            //             Collision::Bottom => intended_movement.0.y += 1.0,
            //         }
            //     } else {
            //         break
            //     } 
            // }
         

        }
        commands.entity(player).remove::<IntendedMovement>();
        commands.entity(player).insert(FinalMovement(intended_movement.0));
    }
}