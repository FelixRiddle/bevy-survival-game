use bevy::prelude::*;
use bevy::ecs::{component::Component, system::Commands};

use crate::FOREGROUND;
use crate::properties::{
    Name,
    Position,
    Velocity,
    Hitbox,
};

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub name: Name,
    pub position: Position,
    pub velocity: Velocity,
    pub hitbox: Hitbox,
    pub sprite: SpriteBundle,
}

/// Spawn a player
/// 
/// 
pub fn spawn_player(
    mut commands: Commands,
) {
    commands.spawn((
        Player,
        FOREGROUND
    ));
}
