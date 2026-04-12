use bevy::{ecs::{query::With, system::{Commands, Query, ResMut}}, platform::collections::HashSet, transform::components::Transform};

use crate::{GameStateReceiver, components::synchronized::Synchronized};



pub fn sync(mut commands:Commands, mut client:ResMut<GameStateReceiver>, mut query:Query<(&Synchronized, &mut Transform), With<Synchronized>>){

    match client.0.try_recv(){
        Ok(mut game_state) =>{
            let mut spawned_player_ids: HashSet<i32> = HashSet::new(); 
            for (sync, mut transform) in query.iter_mut(){

                

                let id = sync.0;
                spawned_player_ids.insert(id);
                println!("Player Id: {}", id);
                println!("Game State: {:?}", game_state);
                if let Some(player) = game_state.get_player_by_id(id){
                    println!("Transform Player State: {:?}", player);
                    transform.translation.x = player.x;
                    transform.translation.y = player.y;
                }
                
            }
            for player in game_state.get_all_players(){
                if !spawned_player_ids.contains(&player.id){
                }
            }
        },
        Err(err) =>{
            eprintln!("game_state receive error {:?}", &err)
        }
    }
}


