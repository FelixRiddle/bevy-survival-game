use bevy::prelude::*;

use crate::properties::{
    Name,
    Position,
    Velocity,
    Hitbox,
};

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

/// Wood and leaves will be intangible
/// so that the player can go through them
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
    pub block_type: BlockType,
    
    // Shape amd physics
    pub position: Position,
    // A block can move?
    pub velocity: Velocity,
    pub hitbox: Hitbox,
    pub tangible: Tangible,
    
    // Rendering
    pub sprite: SpriteBundle,
}

pub struct BlockAndSprite {
    pub name: Name,
    pub sprite: SpriteBundle,
}

impl BlockAndSprite {
    pub fn new(
        asset_server: Res<AssetServer>,
        block_type: BlockType,
    ) -> BlockAndSprite {
        
        match block_type {
            BlockType::Grass => {
                let name_id = "grass";
                let sprite_path = format!("sprite/block/{name_id}/{name_id}.jpg");
                let sprite = SpriteBundle {
                    transform: Transform::from_xyz(0., 0., 0.),
                    texture: asset_server.load(sprite_path),
                    ..default()
                };
                
                BlockAndSprite {
                    name: Name(String::from("Grass")),
                    sprite,
                }
            }
            _ => panic!("Block type not implemented")
        }
    }
}

/// Constructors
/// 
/// 
impl BlockBundle {
    /// Standard
    /// 
    /// 
    pub fn new(
        asset_server: Res<AssetServer>,
        block_type: BlockType,
    ) -> BlockBundle {
        
        // Load sprite
        let block_and_sprite = BlockAndSprite::new(asset_server, block_type.clone());
        
        BlockBundle {
            block: Block { },
            tangible: Tangible::Tangible,
            name: block_and_sprite.name,
            block_type,
            position: Position(Vec2::new(0., 0.)),
            velocity: Velocity(Vec2::new(0., 0.)),
            // The default, most blocks will have a size of 32x32
            hitbox: Hitbox(Vec2::new(32., 32.)),
            sprite: block_and_sprite.sprite
        }
    }
}

/// Spawn grass block
/// 
/// 
pub fn spawn_grass_block(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(BlockBundle::new(
        asset_server,
        BlockType::Grass
    ));
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