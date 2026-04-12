
use bevy::prelude::*;

use crate::{State, systems::network::sync};

pub struct NetworkPlugin;


impl Plugin for NetworkPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sync.run_if(in_state(State::IN_GAME)));
    }
}