use bevy::{prelude::*, utils::hashbrown::Equivalent};

use crate::states::*;

/// Open settings menu
/// 
/// 
pub fn open_settings_menu(mut next_state: ResMut<NextState<AppState>>, current_state: Res<State<AppState>>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        // If it's in the settings menu
        if current_state.equivalent(&AppState::SettingsMenu) {
            next_state.set(AppState::MainMenu);
        }
    }
}

/// Main menu
/// 
/// 
pub fn main_menu() {
    
}

/// Play game
/// 
/// 
pub fn play_game() {
    
}

/// In game menu
/// 
/// 
pub fn in_game_menu(mut next_state: ResMut<NextState<AppState>>, _current_state: Res<State<AppState>>, input: Res<ButtonInput<KeyCode>>) {
    // If it's in the dead menu
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::MainMenu);
    }
}
