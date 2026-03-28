
#[derive(Copy,Clone)]
pub struct Auth{ 
    id:i32
}


impl Auth{ 
    pub fn new(id:i32)->Self{
        return Auth{id:id};
    }

    pub fn get_id(self) -> i32{
        return self.id;
    }
}