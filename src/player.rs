use bevy::ecs::{component::Component, system::Commands};

use crate::FOREGROUND;

#[derive(Component)]
pub struct Player;

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
