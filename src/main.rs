use std::ops::Add;
use bevy::{app::{App, Startup, Update}, ecs::{component, system::{Command, Commands, Query}}, DefaultPlugins};
use bevy::input::ButtonInput;
use bevy::mesh::Mesh2d;
//use bevy::prelude::{Changed, Component, Entity, GlobalTransform, IntoScheduleConfigs, KeyCode, Res, Transform, With};
use bevy::prelude::*;

mod components;
mod systems;

use crate::components::player::Player;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);


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
            Player
        )
    );

}

fn handle_input(keyboard: Res<ButtonInput<KeyCode>>,
                mut query: Query<&mut Transform, With<Player>>,
                time: Res<Time>
){
    let delta=time.delta_secs();
    let mut player_transform = query.iter_mut().next().unwrap();

    let mut dir_vec = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyD){
        dir_vec.x = 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        dir_vec.x = -1.0;
    }
    if keyboard.pressed(KeyCode::KeyW) {
        dir_vec.y = 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        dir_vec.y = -1.0;
    }
    if dir_vec != Vec2::ZERO {
        let norm_dir_vec = dir_vec.normalize();
        let dir_vec = norm_dir_vec * 300.0 * delta;
        player_transform.translation.x += dir_vec.x;
        player_transform.translation.y += dir_vec.y;
    }
}





fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .add_systems(Update, (handle_input).chain()).run();
}
