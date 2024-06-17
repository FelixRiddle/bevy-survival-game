use bevy::prelude::*;
use bevy::ecs::component::Component;
use bevy::render::view::RenderLayers;
use bevy_rapier2d::dynamics::{
    RigidBody,
    Velocity,
};

pub mod player;

use crate::FOREGROUND;
use crate::properties::{
    Gravity,
    Name,
    Hitbox,
    Speed,
};

#[derive(Component)]
pub struct Entity;

#[derive(Bundle)]
pub struct EntityBundle {
    // Identification
    pub entity: Entity,
    
    // Visual name
    pub name: Name,
    
    // Physics and other things
    // Fixed on dynamic though
    pub gravity: Gravity,
    pub rigidbody: RigidBody,
    pub velocity: Velocity,
    pub hitbox: Hitbox,
    pub speed: Speed,
    
    // Rendering
    pub layer: RenderLayers,
    pub sprite: SpriteBundle,
}

/// Convert name to name id
/// 
/// 'name_id' will often times be the same as 'name' lowercase and with underscores for spaces
pub fn name_id(name: &str) -> String {
    let mut name_id = name.to_string();
    name_id = name_id.replace(" ", "_");
    name_id = name_id.to_lowercase();
    return name_id;
}

/// Constructors
/// 
/// 
impl EntityBundle {
    /// Standard
    /// 
    /// 
    pub fn new(
        asset_server: Res<AssetServer>,
        name: &str
    ) -> EntityBundle {
        let name_id = name_id(name);
        let sprite_path = format!("object/character/{name_id}/{name_id}.png");
        let sprite = SpriteBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            texture: asset_server.load(sprite_path),
            ..default()
        };
        
        EntityBundle {
            // Identification
            entity: Entity,
            
            // Visual name
            name: Name(name.to_string()),
            
            gravity: Gravity(5.),
            rigidbody: RigidBody::Dynamic,
            velocity: Velocity {
                linvel: Vec2::new(0., 0.),
                angvel: 80.,
            },
            hitbox: Hitbox(Vec2::new(32., 64.)),
            speed: Speed(300.),
            
            // Rendering
            layer: FOREGROUND,
            sprite
        }
    }
}
