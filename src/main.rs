use std::ops::Add;
use std::sync::{Arc, Mutex, OnceLock};
use bevy::{app::{App, Startup, Update}, ecs::{component, system::{Command, Commands, Query}}, DefaultPlugins};
use bevy::input::ButtonInput;
use bevy::mesh::Mesh2d;
use bevy::prelude::*;
use tokio::sync::mpsc;
mod components;
mod systems;
mod services;
mod traits;



use crate::components::player::Player;
use crate::components::synchronized::Synchronized;
use crate::services::*;
use crate::services::client::{Client, GameState};
use crate::systems::movement::handle_input;
use crate::traits::receiver::Receiver;
use crate::traits::sender::Sender;




#[derive(Resource)]
struct SharedClient(pub Arc<Mutex<Client>>);


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
            Synchronized
        )
    );
}

fn sync(mut commands:Commands, mut client:ResMut<Client>, query:Query<&Transform, With<Synchronized>>){
    //println!("{:?}", client)
    let game_state = client.get_state().unwrap();

    for q in query.iter_mut(){
    }

}

#[tokio::main]
async fn main() {

    let (tx, rx) = mpsc::channel::<GameState>(1);
    let client = Client::new(Some(rx)).await;
    tokio::spawn(async move {
        let mut client = Client::new(None).await;

        loop {
            let msg = client.read().await;
                if tx.send(msg.unwrap()).await.is_err() {
                    break; 
                }
            }
        }
    );


    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(client)
        .add_systems(Startup, init)
        .add_systems(Update, (handle_input, sync).chain())
        .run();
}
