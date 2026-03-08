use bevy::{ecs::{query::With, system::Query}, input::{ButtonInput, keyboard::KeyCode}, time::Time, transform::components::Transform};
use bevy::prelude::*;

use crate::components::player::Player;


const SPEED:u32 = 100;

pub fn handle_input(keyboard: Res<ButtonInput<KeyCode>>,
                mut query: Query<&mut Transform, With<Player>>,
                time: Res<Time>
){

    if keyboard.pressed(KeyCode::KeyD){
    }
    if keyboard.pressed(KeyCode::KeyA) {
    }
    if keyboard.pressed(KeyCode::KeyW) {
    }
    if keyboard.pressed(KeyCode::KeyS) {
    }

}




