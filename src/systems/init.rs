use bevy::{camera::Camera2d, color::Color, ecs::system::Commands, math::Vec2, sprite::Sprite, transform::components::Transform, utils::default};

use crate::components::{player::Player, synchronized::Synchronized};




pub fn init_game(mut commands:Commands){
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
            Synchronized(123)
        )
    );
    commands.spawn(
                (
                        Sprite{
                            color: Color::WHITE,
                            custom_size: Some(Vec2::new(100.0, 100.0)),
                            ..default()
                        },
                        Transform::from_xyz(0.0, 0.0, 0.0),
                        Player,
                        Synchronized(124)
                )
    );
}


