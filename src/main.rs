use std::ops::Add;
use bevy::{app::{App, Startup, Update}, ecs::{component, system::{Command, Commands, Query}}, DefaultPlugins};
use bevy::input::ButtonInput;
use bevy::mesh::Mesh2d;
use bevy::prelude::*;
mod components;
mod systems;

use crate::components::player::Player;
use crate::systems::movement::handle_input;

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

#[tokio::main]
async fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .add_systems(Update, handle_input)
        .run();
}
