use bevy::prelude::*;
use bevy::ecs::{component::Component, system::Commands};
use bevy_rapier2d::control::KinematicCharacterController;

use crate::properties::{
    Speed,
    Username,
    Health
};
use super::EntityBundle;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    // Identification
    pub entity_bundle: EntityBundle,
    pub player: Player,
    pub username: Username,
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
                &asset_server,
                "Player"
            ),
            player: Player,
            username: Username("Player1".to_string()),
        }
    );
}

/// Handle player input
/// 
/// 
pub fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut players: Query<(&mut KinematicCharacterController, &Speed), With<Player>>
) {
    for (mut controller, speed) in players.iter_mut() {
        let speed = speed.0;
        
        let mut velocity = Vec2::new(0., 0.);
        
        if keyboard_input.pressed(KeyCode::KeyW) {
            velocity.y = speed;
        } else if keyboard_input.pressed(KeyCode::KeyS) {
            velocity.y = -speed;
        } else {
            velocity = Vec2::new(velocity.x, 0.);
        }
        
        if keyboard_input.pressed(KeyCode::KeyD) {
            velocity.x = speed;
        } else if keyboard_input.pressed(KeyCode::KeyA) {
            velocity.x = -speed;
        } else {
            velocity = Vec2::new(0., velocity.y);
        }
        
        if velocity.ne(&Vec2::new(0., 0.)) {
            println!("Velocity: {:?}", velocity);
        }
        
        controller.translation = Some(velocity);
    }
}

/// Damage
/// 
/// 
pub fn damage(
    mut players: Query<(&mut Health, &Username), With<Player>>,
    username: &Username,
    damage: f32,
) {
    for (mut health, player_username) in players.iter_mut() {
        if player_username.0 == username.0 {
            health.0 -= damage;
            if health.0 <= 0. {
                println!("Player {} has died!", player_username.0);
            } else {
                println!("Player {} has taken {} damage!", player_username.0, damage);
            }
        }
    }
}

/// Respawn system
/// 
/// 
pub fn respawn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut players: Query<(Entity, &mut Health, &Username), With<Player>>,
) {
    for (_entity, mut health, player_username) in players.iter_mut() {
        if health.0 <= 0. {
            println!("Player {} has respawned!", player_username.0);
            health.0 = 100.;
            commands.spawn(
                PlayerBundle {
                    entity_bundle: EntityBundle::new(
                        &asset_server,
                        "Player"
                    ),
                    player: Player,
                    username: player_username.clone(),
                }
            );
        }
    }
}


