use macroquad::prelude::*;

#[derive(Clone)]
pub struct Planet{
    pub pos: Vec2,
    pub code: u8,
    pub tether: String,
    pub color: Color,
    pub mass: f32,
}

impl Planet{
    // fn new(pos: Vec2, code: u8, tether) -> Self{
    //     Self{
    //         pos, 
    //         code,
    //     }
    // }
    pub fn draw(&self){
        draw_circle(self.pos.x, self.pos.y, 10., self.color);
        draw_text(self.tether.as_str(), self.pos.x, self.pos.y, 10., BLACK);
        // draw_text_ex(self.tether.as_str(), self.pos.x, self.pos.y, TextParams{})
    }
}