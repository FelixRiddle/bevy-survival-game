use bevy::prelude::*;
use bevy::render::view::RenderLayers;

use crate::properties::{
    Name,
    NameId,
};

#[derive(Clone, Component, PartialEq)]
pub struct Item {
    // Visual name
    pub name: Name,
    
    // Unique identifier
    pub name_id: NameId,
    
    pub quantity: u64,
}

#[derive(Bundle)]
pub struct ItemBundle {
    // Identification
    pub item: Item,
    
    // May have physics if it's in the ground
    // // Physics and other things
    // pub gravity: Gravity,
    // pub speed: Speed,
    // pub rigidbody: RigidBody,
    // pub collider: Collider,
    // pub kinematic_character_controller: KinematicCharacterController,
    
    // Rendering
    pub layer: RenderLayers,
    pub sprite: SpriteBundle,
}
