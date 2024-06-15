use bevy::prelude::*;

use crate::properties;

pub mod player;

use properties::{
    Position, Velocity
};

/// Game entity that can move
/// 
/// 
#[derive(Component)]
pub struct Entity;

/// Move entities
/// 
/// 
pub fn move_entities(
    mut entities: Query<(&mut Position, &Velocity), With<Entity>>
) {
    for(mut position, velocity) in &mut entities {
        let new_position = position.0 + velocity.0;
        
        position.0 = new_position;
    }
}
