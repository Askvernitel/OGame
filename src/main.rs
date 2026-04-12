use bevy::platform::collections::HashSet;
use bevy::{app::{App, Startup, Update}, ecs::{system::{Command, Commands, Query}}, DefaultPlugins};
use bevy::prelude::*;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
mod components;
mod systems;
mod services;
mod traits;
mod gui;
mod plugins;

use crate::components::player::Player;
use crate::components::synchronized::Synchronized;
use crate::services::auth::Auth;
use crate::services::client::{Client, ClientOperation, GameState};
use crate::systems::input::handle_input;
use crate::traits::receiver::Receiver;
use crate::traits::sender::Sender;


#[derive(Component, States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum State{
    #[default]
    IN_MENU,
    IN_GAME
}

#[derive(Resource)]
struct GameStateReceiver(pub mpsc::Receiver<GameState>);
#[derive(Resource)]
struct OperationSender(pub mpsc::Sender<ClientOperation>);
fn main() {
    let (state_tx, mut state_rx) = mpsc::channel::<GameState>(32);
    let (cmd_tx, mut cmd_rx) = mpsc::channel::<ClientOperation>(32);

    std::thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async move {
            let auth = Auth::new(123);
            let mut client = Client::new(auth).await;
            loop {
                tokio::select! {
                    result = client.read() => {
                        match result {
                            Ok(state) => { if state_tx.send(state).await.is_err() { break; } }
                            Err(e) => { eprintln!("Read error: {e}");  }
                        }
                    }
                    Some(cmd) = cmd_rx.recv() => {
                        match cmd {
                            ClientOperation::MoveRight => client.move_right().await,
                            ClientOperation::MoveLeft  => client.move_left().await,
                            ClientOperation::MoveUp    => client.move_up().await,
                            ClientOperation::MoveDown  => client.move_down().await,
                        }
                    }
                }
            } 
        });
    }); 

    let game_state_receiver = GameStateReceiver(state_rx);
    let operation_sender = OperationSender(cmd_tx);

    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<State>()
        .insert_resource(game_state_receiver)
        .insert_resource(operation_sender)
        .add_plugins(plugins::game_plugin::GamePlugin)
        .add_plugins(plugins::menu_plugin::MenuPlugin)
        .add_plugins(plugins::input_plugin::InputPlugin)
        .run();
}
