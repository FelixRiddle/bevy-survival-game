use bevy::{
    app::App, prelude::*, utils::hashbrown::Equivalent, DefaultPlugins,
    render::view::RenderLayers,
};

pub mod objects;
pub mod camera;
pub mod plugin;
pub mod states;
pub mod properties;
pub mod tutorial;

use plugin::camera::CameraPlugin;

// const BACKGROUND: RenderLayers = RenderLayers::layer(1);
const FOREGROUND: RenderLayers = RenderLayers::layer(2);

use states::{
    AppState,
    GameMode,
    PauseState,
    LoadingState,
    InGameState,
};

/// Open settings menu
/// 
/// 
fn open_settings_menu(mut next_state: ResMut<NextState<AppState>>, current_state: Res<State<AppState>>, input: Res<ButtonInput<KeyCode>>) {
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
fn main_menu() {
    
}

/// Play game
/// 
/// 
fn play_game() {
    
}

/// In game menu
/// 
/// 
fn in_game_menu(mut next_state: ResMut<NextState<AppState>>, _current_state: Res<State<AppState>>, input: Res<ButtonInput<KeyCode>>) {
    // If it's in the dead menu
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::MainMenu);
    }
}

/// Main
/// 
/// 
fn main() {
    App::new()
        // Add state to app definition
        .init_state::<AppState>()
        .init_state::<GameMode>()
        .init_state::<PauseState>()
        .init_state::<LoadingState>()
        .init_state::<InGameState>()
        // We can add systems to trigger during transitions
        .add_systems(OnEnter(AppState::MainMenu), main_menu)
        // Set pause state to running when the player is playing
        .add_systems(OnEnter(PauseState::Running), play_game)
        .add_systems(OnEnter(AppState::SettingsMenu), open_settings_menu)
        .add_systems(OnEnter(InGameState::InGameMenu), in_game_menu)
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .run();
    
}
