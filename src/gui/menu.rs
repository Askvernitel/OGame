
use std::process::Command;

use bevy::prelude::*;
use bevy::{ camera::Camera2d, color::Color, ecs::{entity::Entity, query::{Changed, With}, system::{Commands, Query, ResMut}}, state::state::{NextState,State }, ui::{BackgroundColor, Interaction, Node, Val, widget::{Button, Text}}, utils::default};

use crate::components::menu::{InputField, InputValue};



pub fn setup_menu(mut commands:Commands){
    //textbox
    commands.spawn(Camera2d::default());


    commands.spawn((
        Node{
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: bevy::ui::JustifyContent::Center,
            align_items: bevy::ui::AlignItems::Center,
            border: UiRect::all(Val::Px(3.0)),
            align_content: bevy::ui::AlignContent::Center,
            //margin:UiRect { left: Val::Percent(25.0), top: Val::Percent(25.0), ..Default::default() },
            flex_direction: bevy::ui::FlexDirection::Column,
            ..default()
        },
        BorderColor{
            top:Color::BLACK,
            bottom:Color::BLACK,
            left:Color::BLACK,
            right:Color::BLACK,
        }
    )).with_children(|parent|{
        parent.spawn(
            (
                Node{
                    width: Val::Px(200.0),
                    height: Val::Px(50.0),
                    align_items: bevy::ui::AlignItems::Center,
                    justify_content: bevy::ui::JustifyContent::Center,
                    ..default()
                },
                InputField,
                InputValue(String::from("")),
            )
        ).with_children(|parent|{
            parent.spawn((
            ));
        }
        );
        parent.spawn(
            (
                Button,
                Node {
                width: Val::Px(100.0),
                height: Val::Px(50.0),
                align_items: bevy::ui::AlignItems::Center,
                justify_content: bevy::ui::JustifyContent::Center,
                ..default()
                },
            )
        ).with_children(|parent| {
            parent.spawn(
                (
                    Text(String::from("Button")),
                )
            );
        });
    });

}


pub fn teardown_menu(mut commands:Commands, 
    q1:Query<Entity,With<Node>>,
){
    for e in &q1 {
        commands.entity(e).despawn();
    }
}




pub fn handle_button(mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>)>,
        mut next_state:ResMut<NextState<crate::State>>,
    ){
    for (interaction, mut color) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(crate::State::IN_GAME);
                *color = Color::BLACK.into();
                println!("Clicked");
            }
            Interaction::Hovered => {
                *color = Color::WHITE.into();
            }
            Interaction::None => {
                *color = Color::BLACK.into();
            }
        }
    }
}
pub fn handle_input_box(){

}