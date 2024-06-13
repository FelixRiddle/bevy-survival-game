use bevy::ecs::schedule::States;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    MainMenu,
    SettingsMenu,
    
    CharactersMenu,
    CharacterCreation,
    
    WorldsMenu,
    WorldCreation,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InGameState {
    #[default]
    Playing,
    Paused,
    Dead,
    // Terraria's and minecraft menus don't pause the game when the in-game menu is open.
    InGameMenu,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameMode {
    #[default]
    NotInGame,
    Singleplayer,
    Multiplayer,
}

/// Pause state on running = the player is playing
/// 
/// 
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PauseState {
    #[default]
    Paused,
    Running,
}

/// Loading state
/// 
/// We also have ephemeral states, but I don't think they fit the finite state machine standard
/// Ephemeral states
/// Loaded, Transition from Loading to NotLoading, executed only once.
/// Unloaded, ...
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LoadingState {
    #[default]
    NotLoading,
    Loading,
    Unloading,
}
