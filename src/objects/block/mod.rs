use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_rapier2d::prelude::*;

use crate::{
    properties::Name,
    FOREGROUND
};

pub mod block_and_sprite;

use block_and_sprite::BlockAndSprite;

#[derive(Component)]
pub struct Block;

#[derive(Component, Clone)]
pub enum BlockType {
    Air,
    Grass,
    Dirt,
    Stone,
    // Trees
    OakWood,
    OakLeaves
}

/// Some blocks should be passed through
/// 
/// Wood, leaves, torches, etc. are intangible.
#[derive(Component)]
pub enum Tangible {
    Tangible,
    Intangible,
}

/// Block bundle
/// 
/// Creating a rust script for every block type, would be a nightmare if you want to update a single field on each.
/// 
/// That's why we're gonna use block type to identify the block type.
#[derive(Bundle)]
pub struct BlockBundle {
    // Information
    pub block: Block,
    pub name: Name,
    // TODO: Should be removed
    // I should add configuration in json files on every object and then dynamically load them.
    pub block_type: BlockType,
    
    // Shape and physics
	pub rigidbody: RigidBody,
    pub collider: Collider,
    
    // Layer is dependent on tangibility
    pub layer: RenderLayers,
    pub tangible: Tangible,
    
    // Rendering
	// Sprite bundle includes the transform
    pub sprite: SpriteBundle,
}

/// Constructors
/// 
/// 
impl BlockBundle {
    /// Standard
    /// 
    /// 
    pub fn new(
        asset_server: &Res<AssetServer>,
        block_type: BlockType,
		transform: Transform,
    ) -> BlockBundle {
        
        // Load sprite
        let block_and_sprite = BlockAndSprite::new(asset_server, block_type.clone(), transform);
        
        BlockBundle {
            block: Block { },
            name: block_and_sprite.name,
            block_type,
            
            rigidbody: RigidBody::Fixed,
            collider: Collider::cuboid(32., 32.),
            
            layer: FOREGROUND,
            tangible: Tangible::Tangible,
            
            sprite: block_and_sprite.sprite
        }
    }
}

/// Spawn grass block
/// 
/// Dumb function, because you can't specify the location
pub fn spawn_grass_block(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(BlockBundle::new(
        &asset_server,
        BlockType::Grass,
		Transform::from_xyz(0., 0., 0.)
    ));
}

/// Move blocks
pub fn move_blocks(
    mut blocks: Query<(&mut Transform, ), With<Block>>
) {
    for mut block in blocks.iter_mut() {
        block.0.translation.y -= 128.;
    }
}

/// Print sprite bounding sizes
/// 
/// To get sprite sizes
pub fn print_sprite_bounding_sizes(
    mut sprites: Query<(&Transform, &Handle<Image>), With<Sprite>>,
    assets: Res<Assets<Image>>
) {
    for (transform, image_handle) in &mut sprites {
        let image_size = assets
            .get(image_handle)
            .unwrap()
            .size_f32();
        
        info!("Image dimensions: {:?}", image_size);
        info!("Image position: {:?}", transform.translation);
        info!("Image scale: {:?}", transform.scale);
        
        let scaled = image_size * transform.scale.truncate();
        let bounding_box = Rect::from_center_size(
            transform.translation.truncate(),
            scaled,
        );
        
        info!("Bounding box: {:?}", bounding_box);
    }
}
