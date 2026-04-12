use bevy::prelude::*;

use crate::{State, systems::input::handle_input};

pub struct InputPlugin;

impl Plugin for InputPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_input.run_if(in_state(State::IN_GAME)));
    }
}