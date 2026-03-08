use crate::traits::Sender;



pub static CLIENT:Client = Client::new();
pub struct Client{ }
impl Client{
    fn new()-> Self{
        Client{ };
    }
}
impl Sender for Client{

    fn move_right(){
    }
    fn move_left(){
    }
    fn move_up(){
    }
    fn move_down(){
    }
}
impl Receiver for Client{

}


