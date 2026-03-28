use std::ops::Add;
use std::sync::mpsc::TryRecvError;
use std::sync::{Arc, Mutex, OnceLock};
use bevy::{app::{App, Startup, Update}, ecs::{component, system::{Command, Commands, Query}}, DefaultPlugins};
use bevy::input::ButtonInput;
use bevy::mesh::Mesh2d;
use bevy::prelude::*;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
mod components;
mod systems;
mod services;
mod traits;



use crate::components::player::Player;
use crate::components::synchronized::Synchronized;
use crate::services::auth::Auth;
use crate::services::*;
use crate::services::client::{Client, ClientOperation, GameState};
use crate::systems::movement::handle_input;
use crate::traits::receiver::Receiver;
use crate::traits::sender::Sender;



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
}

fn sync(mut commands:Commands, mut client:ResMut<GameStateReceiver>, mut query:Query<(&Synchronized, &mut Transform), With<Synchronized>>){

    match client.0.try_recv(){
        Ok(mut game_state) =>{

            for (sync, mut transform) in query.iter_mut(){

                


                let id = sync.0;
                println!("Game State: {:?}", game_state);
                if let Some(player) = game_state.get_player_by_id(id){
                    println!("Transform Player State: {:?}", player);
                    transform.translation.x = player.x;
                    transform.translation.y = player.y;
                }else{
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
        .insert_resource(game_state_receiver)
        .insert_resource(operation_sender)
        .add_systems(Startup, init)
        .add_systems(Update, (handle_input, sync).chain())
        .run();
}
