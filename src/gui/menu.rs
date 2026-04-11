
use bevy::{ camera::Camera2d, color::Color, ecs::system::Commands, ui::{BackgroundColor, Node, Val}, utils::default};



pub fn setup_menu(mut commands:Commands){
    //textbox
    commands.spawn(Camera2d::default());


    commands.spawn((
        Node{
            width: Val::Px(20.0),
            height: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::BLACK)
    ));

    //button
}


pub fn teardown_menu(){

}