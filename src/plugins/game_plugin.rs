use bevy::prelude::*;

use crate::{State, systems::init::{*}};



pub struct GamePlugin;

impl Plugin for GamePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(State::IN_GAME), init_game);
    }
}