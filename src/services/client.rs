use std::default;
use std::fmt::{Debug, Formatter};
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
use crate::traits::receiver::Receiver;
use crate::traits::sender::Sender;

#[derive(Resource)]
pub struct Client{
    stream:WebSocketStream<MaybeTlsStream<TcpStream>>,

    rx:Option<mpsc::Receiver<GameState>>,
}

#[derive(Serialize, Debug)]
struct Operation {
    #[serde(rename = "type")]
    pub typ: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Player{
    pub id : i32,
    pub x: f32,
    pub y: f32,
    pub speed: f32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GameState{
    pub players: Vec<Player>
}


impl GameState{

    pub fn get_player_by_id(id: i32){
    }
}


impl Client{
    pub async fn new(rx:Option<mpsc::Receiver<GameState>> )-> Self{
        let (ws_stream, _) = connect_async("ws://localhost:8080/connect").await.expect("Failed to connect");
        Client{
            stream:ws_stream,
            rx:rx
        }
    }

    pub fn get_state(self) -> Result<GameState, String>{
        match self.rx.unwrap().try_recv(){
            Ok(game_state) => {
                println!("Got game state: {:?}", game_state);
                Ok(game_state)
            }
            Err(TryRecvError::Empty) => {
                Err(String::from("Empty"))
            }
            Err(TryRecvError::Disconnected) => {
                Err(String::from("Disconnected"))
            }
        }
    }
}
impl Sender for Client{
    fn move_right(&mut self){
        let json = serde_json::to_string(&Operation{typ:1});
        self.stream.send(Message::Text(json.unwrap().into()));

        println!("Moving Right");
    }
    fn move_left(self){
    }
    fn move_down(self){
    }
    fn move_up(self){
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




