use bevy::{app::{App, Startup, Update}, ecs::{component, system::{Command, Commands, Query}}, DefaultPlugins};
use bevy::input::ButtonInput;
use bevy::mesh::Mesh2d;
//use bevy::prelude::{Changed, Component, Entity, GlobalTransform, IntoScheduleConfigs, KeyCode, Res, Transform, With};
use bevy::prelude::*;


use components;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn hello_world(){
    println!("Hello, world!");
}
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
        )
    );

}

fn handle_input(keyboard: Res<ButtonInput<KeyCode>>){
}





fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .add_systems(Update, (handle_input).chain()).run();
}
