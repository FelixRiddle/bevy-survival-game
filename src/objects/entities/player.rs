use bevy::prelude::*;
use bevy::ecs::{component::Component, system::Commands};

use crate::properties::{
    Velocity,
    Speed,
};
use super::EntityBundle;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    // Identification
    pub entity_bundle: EntityBundle,
    pub player: Player,
    
}

/// Spawn a player
/// 
/// 
pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(
        PlayerBundle {
            entity_bundle: EntityBundle::new(
                asset_server,
                "Player"
            ),
            player: Player,
        }
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
