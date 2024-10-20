use bevy::{app::App, prelude::*, render::view::RenderLayers, DefaultPlugins};
use bevy_rapier2d::prelude::*;

pub mod camera;
pub mod objects;
pub mod plugin;
pub mod properties;
pub mod state_transition;
pub mod states;
pub mod tutorial;
pub mod ui;

use objects::{
    block,
    entities::player::{self, handle_player_input},
};

// const BACKGROUND: RenderLayers = RenderLayers::layer(1);
const FOREGROUND: RenderLayers = RenderLayers::layer(2);

use states::{AppState, GameMode, InGameState, LoadingState, PauseState};

/// Function to create a platform of grass blocks
/// 
/// 
fn create_grass_platform(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
	let start_position = Vec2::new(0.0, 100.0);
	let width = 20;
	let height = 5;
	
    for x in 0..width {
        for y in 0..height {
            let position = start_position + Vec2::new(x as f32 * 32.0, y as f32 * 32.0);
            // Destructure to create a temporary reference inside the loop
            let (server,) = (&asset_server,);
            
            commands.spawn(
                block::BlockBundle::new(
                    server, // Use the temporary reference here
                    block::BlockType::Grass,
					Transform::from_xyz(position.x, position.y, 0.0),
                )
            );
        }
    }
}

/// Main
///
///
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.))
        .add_plugins(RapierDebugRenderPlugin::default())
        // Add state to app definition
        .init_state::<AppState>()
        .init_state::<GameMode>()
        .init_state::<PauseState>()
        .init_state::<LoadingState>()
        .init_state::<InGameState>()
        .add_systems(
            Startup,
            (
                camera::initialize_camera,
                block::spawn_grass_block,
                player::spawn_player,
                // block::move_blocks.after(block::spawn_grass_block),
				create_grass_platform,
            ),
        )
        .add_systems(Update, (handle_player_input,))
        // Update states
        // We can add systems to trigger during transitions
        .add_systems(OnEnter(AppState::MainMenu), state_transition::main_menu)
        // Set pause state to running when the player is playing
        .add_systems(OnEnter(PauseState::Running), state_transition::play_game)
        .add_systems(
            OnEnter(AppState::SettingsMenu),
            state_transition::open_settings_menu,
        )
        .add_systems(
            OnEnter(InGameState::InGameMenu),
            state_transition::in_game_menu,
        )
        .run();
}
