use bevy::prelude::*;
use bevy::ecs::component::Component;
use bevy::render::view::RenderLayers;

pub mod player;

use crate::FOREGROUND;
use crate::properties::{
    Name,
    Position,
    Velocity,
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
    pub position: Position,
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
        let sprite_path = format!("sprite/entities/{name_id}/{name_id}.png");
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
            
            position: Position(Vec2::new(0., 0.)),
            velocity: Velocity(Vec2::new(0., 0.)),
            hitbox: Hitbox(Vec2::new(32., 64.)),
            speed: Speed(5.),
            
            // Rendering
            layer: FOREGROUND,
            sprite
        }
    }
}

/// Move entities
/// 
/// 
pub fn move_entities(
    mut entities: Query<(&mut Position, &mut Transform, &Velocity), With<Entity>>,
) {
    for(mut position, mut transform, velocity) in &mut entities {
        let new_position = position.0 + velocity.0;
        
        position.0 = new_position;
        
        let new_position = Vec3::new(new_position.x, new_position.y, 0.);
        transform.translation = new_position;
    }
}
