use bevy::{ecs::{query::With, system::Query}, input::{ButtonInput, keyboard::KeyCode}, time::Time, transform::components::Transform};
use bevy::prelude::*;

use crate::components::player::Player;


const SPEED:u32 = 100;

fn move_input(
            keyboard: Res<ButtonInput<KeyCode>>,
            mut query: Query<&mut Transform, With<Player>>,
            time: Res<Time>
            ){

}




