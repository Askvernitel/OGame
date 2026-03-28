

pub trait Sender{
    async fn move_right(&mut self);
    async fn move_left(&mut self);
    async fn move_down(&mut self);
    async fn move_up(&mut self);
}
