pub mod line    {
    pub struct Line {
        pub x1:i16,
        pub y1:i16,
        pub x2:i16,
        pub y2:i16
    }
    impl Line{
        pub fn new(pont_x_1:i16, pont_y_1:i16, pont_x_2:i16, pont_y_2:i16) -> Self {
            Self { x1: (pont_x_1), y1: (pont_y_1), x2: (pont_x_2), y2: (pont_y_2) }

        }
    }
}

pub mod vector {
    use super::line::Line;


    pub struct Vector {
        pub speed:f32,
        pub direction:f32
    }
    impl Vector {
        pub fn new() -> Self {
            Self { speed: (1.5), direction: (0.0) }
        }

        pub fn projection(&self, line:Line) -> Self {

            let dx = (line.x2 - line.x1) as f32;
            let dy = (line.y2 - line.y1) as f32;
            let mut slope:f32 = 0.0;
            if dx == 0.0 {
                let slope:f32 = f32::MAX;
            }
            else {
                let slope:f32 = (dy.abs() / dx.abs()) as f32;
            }


            Self { speed: self.speed.clone(), direction: (-1.0/slope.abs()).atan()}



        }
        pub fn reflect_vector(&mut self, line:Line) {
            self.direction = self.direction + self.projection(line).direction;
        }
    }
}
