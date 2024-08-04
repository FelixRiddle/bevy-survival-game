use bevy::prelude::*;
use crate::objects::item::Item;

use crate::objects::entities::player::Player;
use crate::properties::Username;

/// Slot
#[derive(Component, PartialEq)]
pub struct Slot {
    pub item: Option<Item>,
}

/// Inventory
/// 
/// 
#[derive(Component)]
pub struct Inventory {
    // Items 1 to 9 are the hotbar
    pub items: Vec<Slot>,
    
    // Slot index
    pub selected_item: u32,
    pub inventory_size: u32,
}

impl Inventory {
    /// Create new inventory
    /// 
    /// 
    pub fn new() -> Self {
        
        let inventory_size = 36;
        
        // Initialize slots
        let mut items = Vec::new();
        for _ in 0..inventory_size {
            items.push(Slot { item: None });
        }
        
        Self {
            items: Vec::new(),
            selected_item: 0,
            inventory_size,
        }
    }
    
    /// Add item to inventory
    /// 
    /// 
    pub fn add_item(&mut self, item: Item) {
        // Add to the first empty slot
        if let Some(index) = self.items.iter().position(|x| x.item.is_none()) {
            println!("Added item: {}", item.name_id.0);
            self.items[index].item = Some(item);
        } else {
            println!("Inventory is full, cannot add item: {}", item.name_id.0);
        }
    }
}

/// Pickup item
/// 
/// 
pub fn pickup_item(
    mut players: Query<(&mut Inventory, &Username), With<Player>>,
    item: &Item,
    username: &Username,
) {
    for (mut inventory, player_username) in players.iter_mut() {
        if player_username.0 == username.0 {
            inventory.add_item(item.clone());
            println!("Player {} picked up item: {}", player_username.0, item.name_id.0);
        }
    }
}

/// Drop item
/// 
/// 
pub fn drop_item(
    mut players: Query<(&mut Inventory, &Username), With<Player>>,
    item_name_id: &str,
    username: &Username,
) {
    for (mut inventory, player_username) in players.iter_mut() {
        if player_username.0 == username.0 {
            if let Some(index) = inventory.items.iter().position(|x| x.item.as_ref().map_or(false, |i| i.name_id.0 == item_name_id)) {
                inventory.items.remove(index);
                println!("Player {} dropped item: {}", player_username.0, item_name_id);
            } else {
                println!("Player {} does not have item: {}", player_username.0, item_name_id);
            }
        }
    }
}
