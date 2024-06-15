use bevy::prelude::*;
use bevy::ecs::{component::Component, system::Commands};
use bevy::render::view::RenderLayers;

use crate::FOREGROUND;
use crate::properties::{
    Name,
    Position,
    Velocity,
    Hitbox,
    Speed,
};
use super::Entity;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    // Identification
    pub entity: Entity,
    pub player: Player,
    
    // Visual name
    pub name: Name,
    
    pub position: Position,
    pub velocity: Velocity,
    pub hitbox: Hitbox,
    pub speed: Speed,
    
            // Rendering
    pub layer: RenderLayers,
    pub sprite: SpriteBundle,
}

/// Constructors
/// 
/// 
impl PlayerBundle {
    /// Standard
    /// 
    /// 
    pub fn new(
        asset_server: Res<AssetServer>,
        name: &str
    ) -> PlayerBundle {
        // Is it possible to load jpg?, most blocks don't have transparency.
        let name_id = "player";
        let sprite_path = format!("sprite/entities/{name_id}/{name_id}.png");
        let sprite = SpriteBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            texture: asset_server.load(sprite_path),
            ..default()
        };
        
        PlayerBundle {
            // Identification
            entity: Entity,
            player: Player,
            
            // Visual name
            name: Name(name.to_string()),
            
            position: Position(Vec2::new(0., 0.)),
            velocity: Velocity(Vec2::new(0., 0.)),
            hitbox: Hitbox(Vec2::new(32., 64.)),
            speed: Speed(5.),
            
            // Rendering
            layer: FOREGROUND,
            sprite: sprite
        }
    }
}

/// Spawn a player
/// 
/// 
pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(
        PlayerBundle::new(
            asset_server,
            "Player"
        )
    );
}

/// Handle player input
/// 
/// 
pub fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut players: Query<(&mut Velocity, &Speed), With<Player>>
) {
    for (mut velocity, speed) in players.iter_mut() {
        let speed = speed.0;
        
        if keyboard_input.pressed(KeyCode::KeyW) {
            velocity.0.y = speed;
        } else if keyboard_input.pressed(KeyCode::KeyS) {
            velocity.0.y = -speed;
        } else if keyboard_input.pressed(KeyCode::KeyD) {
            velocity.0.x = speed;
        } else if keyboard_input.pressed(KeyCode::KeyA) {
            velocity.0.x = -speed;
        } else {
            velocity.0 = Vec2::new(0., 0.);
        }
    }
}
