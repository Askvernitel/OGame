use std::sync::Arc;

use bevy::{ecs::{query::With, system::Query}, input::{ButtonInput, keyboard::KeyCode}, time::Time, transform::components::Transform};
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use crate::{OperationSender, components::player::Player, services::client::ClientOperation};
use crate::services::client::Client;
use crate::traits::sender::Sender;

const SPEED:u32 = 100;

pub fn handle_input(keyboard: Res<ButtonInput<KeyCode>>,
                mut query: Query<&mut Transform, With<Player>>,
                mut client:ResMut<OperationSender>,
                time: Res<Time>
){

    for mut transform in query.iter_mut(){
        if keyboard.pressed(KeyCode::KeyD){
            transform.translation.x +=100.0;
            client.0.try_send(ClientOperation::MoveRight);
        }
        if keyboard.pressed(KeyCode::KeyA) {
            transform.translation.x -=100.0;
            client.0.try_send(ClientOperation::MoveLeft);
        }
        if keyboard.pressed(KeyCode::KeyW) {
            transform.translation.y +=100.0;
            client.0.try_send(ClientOperation::MoveUp);
        }
        if keyboard.pressed(KeyCode::KeyS) {
            transform.translation.y -=100.0;
            client.0.try_send(ClientOperation::MoveDown);
        }
    }
}




