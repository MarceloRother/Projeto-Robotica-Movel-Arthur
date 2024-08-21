pub struct Caixa{
    pos_x: i32,
    pos_y: i32,
}

impl Caixa{
    pub fn new(x: i32, y: i32) -> Self{
        Self { pos_x: x, pos_y: y }
    }

    pub fn get_pos_x(&self) -> i32 {
        self.pos_x
    }

    pub fn get_pos_y(&self) -> i32 {
        self.pos_y
    }
}
