use bevy::prelude::*;

use crate::properties::Name;

use super::BlockType;

pub struct BlockAndSprite {
    pub name: Name,
    pub sprite: SpriteBundle,
}

impl BlockAndSprite {
    pub fn new(
        asset_server: &Res<AssetServer>,
        block_type: BlockType,
		transform: Transform,
    ) -> BlockAndSprite {
        
        match block_type {
            BlockType::Grass => {
                let name_id = "grass";
                // Is it possible to load jpg?, most blocks don't have transparency.
                let sprite_path = format!("object/block/{name_id}/{name_id}.png");
                let sprite = SpriteBundle {
                    transform,
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
