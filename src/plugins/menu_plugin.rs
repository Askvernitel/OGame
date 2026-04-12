use bevy::prelude::*;

use crate::{State, gui::menu};


pub struct MenuPlugin;


pub impl Plugin for MenuPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(State::IN_MENU), menu::setup_menu);
        app.add_systems(OnExit(State::IN_MENU), menu::teardown_menu);
        app.add_systems(Update, menu::handle_button.run_if(in_state(State::IN_MENU)));
    }
}