# Multiplayer

To implement multiplayer in the 'bevy-survival-game', you can use the Bevy networking library, such as bevy_net. Here's a high-level overview of how you can add multiplayer support to the game:

1.
Add the bevy_net and bevy_net_websocket crates to your project's dependencies in the Cargo.toml file:

```toml
[dependencies]
bevy = "0.8"
bevy_rapier2d = "0.16"
bevy_net = "0.8"
bevy_net_websocket = "0.8"
```

2.
Import the necessary modules:

```rust
use bevy_net::prelude::*;
use bevy_net_websocket::WsConfig;
```

3.
Define a new network message struct for player input, such as PlayerInput:

```rust
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct PlayerInput {
    pub movement: Vec2,
    pub action: Action,
}
```

4.
Implement the network setup in the main function:

```rust
fn main() {
    App::new()
        // ... other plugins ...
        .add_plugin(bevy_net::NetPlugin::default())
        .add_plugin(bevy_net_websocket::WsPlugin::default())
        // ... other systems ...
        .add_startup_system(setup_network.system())
        .run();
}

fn setup_network(mut commands: Commands, mut ws_config: ResMut<WsConfig>) {
    ws_config.url = "ws://localhost:8080".parse().unwrap();
    commands.insert_resource(ws_config);
}
```

5.
Implement the network connection and synchronization logic:

```rust
fn handle_player_input(
    mut commands: Commands,
    mut input: ResMut<Input<KeyCode>>,
    mut player_input: ResMut<PlayerInput>,
    mut ws_client: ResMut<WsClient>,
) {
    // Update player input based on keyboard input
    // ...

    // Send player input to the server
    if let Ok(ws_channel) = ws_client.get_mut() {
        if ws_channel.is_open() {
            ws_channel.send_binary(bincode::serialize(&player_input).unwrap());
        }
    }
}

fn receive_player_input(
    mut commands: Commands,
    mut input_events: EventReader<WsMessage<PlayerInput>>,
    mut players: Query<(Entity, &mut Player)>,
) {
    for event in input_events.iter() {
        match event {
            WsMessage::Binary(data) => {
                if let Ok(received_input) = bincode::deserialize::<PlayerInput>(&data) {
                    // Update player state based on received input
                    // ...
                }
            }
            _ => {}
        }
    }
}
```

6.
Add the handle_player_input and receive_player_input systems to the main function:

```rust
fn main() {
    App::new()
        // ... other plugins ...
        .add_system(handle_player_input.system())
        .add_system(receive_player_input.system())
        // ... other systems ...
        .run();
}
```

This is a basic example of how you can implement multiplayer support in the 'bevy-survival-game' using the Bevy networking library. You can further customize and enhance the network setup and synchronization logic based on your specific requirements.

Please note that implementing multiplayer support in a game requires a good understanding of networking concepts and the Bevy game engine. It's recommended to refer to the Bevy and Bevy networking documentation for more detailed information and examples.
