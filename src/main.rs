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

fn init(mut commands:Commands){
    commands.spawn(Camera2d::default());

    commands.spawn(
        (
            Sprite{
                color: Color::BLACK,
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
            Player,
            Synchronized(123)
        )
    );
    commands.spawn(
                (
                        Sprite{
                            color: Color::WHITE,
                            custom_size: Some(Vec2::new(100.0, 100.0)),
                            ..default()
                        },
                        Transform::from_xyz(0.0, 0.0, 0.0),
                        Player,
                        Synchronized(124)
                )
    );
}

fn sync(mut commands:Commands, mut client:ResMut<GameStateReceiver>, mut query:Query<(&Synchronized, &mut Transform), With<Synchronized>>){

    match client.0.try_recv(){
        Ok(mut game_state) =>{
            let mut spawned_player_ids: HashSet<i32> = HashSet::new(); 
            for (sync, mut transform) in query.iter_mut(){

                

                let id = sync.0;
                spawned_player_ids.insert(id);
                println!("Player Id: {}", id);
                println!("Game State: {:?}", game_state);
                if let Some(player) = game_state.get_player_by_id(id){
                    println!("Transform Player State: {:?}", player);
                    transform.translation.x = player.x;
                    transform.translation.y = player.y;
                }
                
            }
            for player in game_state.get_all_players(){
                if !spawned_player_ids.contains(&player.id){
                }
            }
        },
        Err(err) =>{
            eprintln!("game_state receive error {:?}", &err)
        }
    }
}



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
        .add_systems(OnEnter(State::IN_MENU), gui::menu::setup_menu)
        .add_systems(OnExit(State::IN_MENU), gui::menu::teardown_menu)
        .add_systems(OnEnter(State::IN_GAME), init)
        .add_systems(Update, (gui::menu::handle_button, handle_input, sync).chain())
        .run();
}
