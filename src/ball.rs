use std::f32::consts::PI;
use std::io::{stdout, Write};
use crossterm::cursor::MoveTo;
use crossterm::{queue, QueueableCommand, style};
use crossterm::style::Stylize;
use crate::lib::vector;
use crate::lib::line;
use crate::crossterm;
pub struct Ball {
    pub posx:f32,
    pub posy:f32,
    pub radius:i8,
    pub vector:vector::Vector
}
impl Ball {
    pub fn new() -> Self {
        Self {
            posx:50.0,
            posy:15.0,
            radius:6,
            vector:vector::Vector {speed:0.08, direction:2.0001*PI}
    }
}
    pub fn tick(&mut self) {
        self.posx = self.posx + (self.vector.speed * self.vector.direction.sin()).to_degrees();
        self.posy = self.posy + (self.vector.speed * self.vector.direction.cos()).to_degrees();
        self.draw();
    }
    pub fn set_pos(&mut self, posx:f32, posy:f32)   {
        self.posx = posx;
        self.posy = posy;
    }

    pub fn draw(&mut self) {
        let res = 2;
        let mut angle: f32 = 0.0;
        let radius = self.radius as u32 as f32;
        let horz_stretch: f32 = 2.2;

            while angle <= 360.0 {
                stdout().queue(MoveTo((self.posx + radius * angle.to_radians().cos() * horz_stretch) as u32 as i16 as u16, (self.posy + radius * angle.to_radians().sin()) as u32 as i16 as u16));
                stdout().queue(style::PrintStyledContent("·".white()));
                angle = angle + 6.0;
            }




        //stdout().queue(MoveTo(self.posx as u32 as i16 as u16, self.posy as u32 as i16 as u16));
        stdout().flush();

    }
    
}
