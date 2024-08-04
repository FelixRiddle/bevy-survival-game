use bevy::prelude::*;

/// Entity speed
/// 
/// 
#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Gravity(pub f32);

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

// #[derive(Component)]
// pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Hitbox(pub Vec2);

/// The visible name of an object or entity
/// 
/// 
#[derive(Clone, Component, PartialEq)]
pub struct Name(pub String);

/// Entity identifiable string id
/// 
/// 
#[derive(Clone, Component, PartialEq)]
pub struct NameId(pub String);

/// Username
/// 
/// Public unique identifier
/// 
/// This is to not show user email, and use a unique text identifier
#[derive(Clone, Component, PartialEq)]
pub struct Username(pub String);

/// Health
/// 
/// 
#[derive(Component)]
pub struct Health(pub f32);
