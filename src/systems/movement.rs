use bevy::{ecs::{query::With, system::Query}, input::{ButtonInput, keyboard::KeyCode}, time::Time, transform::components::Transform};
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use crate::components::player::Player;
use crate::services::client::Client;
use crate::SharedClient;
use crate::traits::sender::Sender;

const SPEED:u32 = 100;

pub fn handle_input(keyboard: Res<ButtonInput<KeyCode>>,
                mut query: Query<&mut Transform, With<Player>>,
                mut client:ResMut<Client>,
                time: Res<Time>
){
    if keyboard.pressed(KeyCode::KeyD){
        client.into_inner().move_right();
    }

    if keyboard.pressed(KeyCode::KeyA) {
    }
    if keyboard.pressed(KeyCode::KeyW) {
    }
    if keyboard.pressed(KeyCode::KeyS) {
    }

}




