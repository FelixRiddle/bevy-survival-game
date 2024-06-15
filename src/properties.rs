use bevy::prelude::*;

/// Entity speed
/// 
/// 
#[derive(Component)]
pub struct Speed(pub f32);

/// The visible name of an object or entity
/// 
/// 
#[derive(Component)]
pub struct Name(pub String);

/// Physical attributes
/// 
/// 
#[derive(Component)]
pub struct Position(pub Vec2);

impl Default for Position {
    fn default() -> Self {
        Position(Vec2::new(0., 0.))
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Hitbox(pub Vec2);
