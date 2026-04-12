use bevy::prelude::*;

use crate::{State, gui::menu::{self, handle_focus, handle_input_field}};


pub struct MenuPlugin;


impl Plugin for MenuPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(State::IN_MENU), menu::setup_menu)
        .add_systems(OnExit(State::IN_MENU), menu::teardown_menu)
        .add_systems(Update, menu::handle_button.run_if(in_state(State::IN_MENU)))
        .add_systems(Update, (handle_focus, handle_input_field).run_if(in_state(State::IN_MENU)));
    }
}