use crate::services::client::GameState;


pub trait Receiver{
    async fn read(&mut self) -> Result<GameState, String> ; 
}

