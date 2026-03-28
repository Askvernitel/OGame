use std::default;
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};
use bevy::ecs::message::MessageCursor;
use bevy::prelude::Resource;
use serde_json::json;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;
use crate::services::auth::Auth;
use crate::traits::receiver::Receiver;
use crate::traits::sender::Sender;

pub enum ClientOperation {
    MoveRight,
    MoveLeft,
    MoveUp,
    MoveDown,
}




#[derive(Resource)]
pub struct Client{
    stream:WebSocketStream<MaybeTlsStream<TcpStream>>,

    pub auth:Auth,
    pub game_state:GameState,
}

#[derive(Serialize, Debug)]
struct Operation {
    #[serde(rename = "type")]
    pub typ: i32,
}

#[derive(Serialize, Debug)]
struct OperationRequest{ 
    pub caller:i32,
    pub operations:Vec<Operation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player{
    pub id : i32,
    pub x: f32,
    pub y: f32,
    pub speed: f32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameState{
    pub players: Vec<Player>
}


impl GameState{
    pub fn new()->Self{
        GameState{players:vec![]}
    }
    pub fn get_player_by_id(&mut self, id: i32)->Option<Player>{
        for player in &self.players{
            if player.id == id {
                //println!("Found Player {:?}", player);
                return Some(player.clone());
            }
        }
        None
    }

    pub fn get_all_players(&mut self)->Vec<Player>{
        return self.players.clone();
    }
}


impl Client{
    pub async fn new(auth:Auth )-> Self{
        let (ws_stream, _) = connect_async("ws://localhost:8080/connect").await.expect("Failed to connect");
        Client{
            stream:ws_stream,
            auth:auth,
            game_state:GameState::new()
        }
    }

    pub fn get_state(self) -> GameState{
        self.game_state
    }

    pub fn get_auth(self) -> Auth{
        return self.auth;
    }
}
impl Sender for Client{
    async fn move_right(&mut self){
        let json_req = serde_json::to_string(
            &OperationRequest{
                caller:123,
                operations:vec![Operation{typ:0}]
            }
        );
        self.stream.send(Message::Text(json_req.unwrap().into())).await;
        println!("Moving Right");
    }
    async fn move_left(&mut self){
        let json_req = serde_json::to_string(
            &OperationRequest{
                caller:123,
                operations:vec![Operation{typ:1}]
            }
        );
        self.stream.send(Message::Text(json_req.unwrap().into())).await;
        println!("Moving Left");

    }
    async fn move_down(&mut self){
        let json_req = serde_json::to_string(
            &OperationRequest{
                caller:123,
                operations:vec![Operation{typ:2}]
            }
        );
        self.stream.send(Message::Text(json_req.unwrap().into())).await;
        println!("Moving Down");


    }
    async fn move_up(&mut self){
        let json_req = serde_json::to_string(
            &OperationRequest{
                caller:123,
                operations:vec![Operation{typ:3}]
            }
        );
        self.stream.send(Message::Text(json_req.unwrap().into())).await;
        println!("Moving Up");
    }

}


impl Debug for Client{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.stream.fmt(f)
    }
}
impl Receiver for Client{
    async fn read(&mut self) -> Result<GameState, String> {
    let result = self.stream.next().await.ok_or("Stream ended")?;

    let message = result.map_err(|e| e.to_string())?;

    if message.is_text() {
        let text = message.to_text().map_err(|e| e.to_string())?;
        let game_state = serde_json::from_str::<GameState>(text)
            .map_err(|e| e.to_string())?;
        Ok(game_state)
    } else if message.is_close() {
        Err("Connection closed".into())
    } else {
        Err("Unsupported message type".into())
    }
}
}




