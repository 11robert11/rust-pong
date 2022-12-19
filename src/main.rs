pub mod ball;
pub mod board;
pub mod lib;

use std::io::{stdout, Write};
use std::{thread, time};
use std::ops::Sub;
use crate::lib::line::Line;
use crossterm;
use crossterm::{cursor, ExecutableCommand, QueueableCommand};
use crossterm::cursor::Hide;
use crossterm::style::{Color, SetBackgroundColor};
use crossterm::style::Colored::BackgroundColor;
use crossterm::terminal::{Clear, ClearType, SetTitle, size};

fn main() {

    println!("Hello, world!");
    let mut game_ball: ball::Ball = ball::Ball::new();
    let mut stdout = stdout();
    let tick_time_target:i64 = 40;
    stdout.queue(SetBackgroundColor(Color::Black));
    stdout.queue(Hide);


// some other code ...

    stdout.flush();
    //crossterm::terminal::SetTitle("Starting..."), crossterm::terminal::Clear(ClearType::All));
    game_ball.vector.direction = 48.0_f32.to_radians();
    println!("{}", (1.0 as f32).atan());
    println!("{}", (-1.0/1.0 as f32).atan());

    loop    {

        let start = time::Instant::now();

        stdout.queue(Clear(ClearType::All));

        if game_ball.posx + game_ball.radius as f32 >= crossterm::terminal::size().unwrap().0 as f32 {   // Right wall
            game_ball.vector.reflect_vector(Line {x1: size().unwrap().0 as i16, y1:0, x2: size().unwrap().0 as i16, y2: size().unwrap().1 as i16 });
            stdout.queue(SetBackgroundColor(Color::Red));
        }
        if game_ball.posx - (game_ball.radius as f32) <= 0.0 as f32 {    // Left Wall
            game_ball.vector.reflect_vector(Line {x1: 0, y1:0, x2: 0, y2: size().unwrap().1 as i16 });
            stdout.queue(SetBackgroundColor(Color::Black));
        }
        if game_ball.posy - (game_ball.radius as f32) <= 0.0 as f32 {    // Ceiling
            game_ball.vector.reflect_vector(Line {x1: 0, y1:0, x2: size().unwrap().0 as i16, y2:0 });
            stdout.queue(SetBackgroundColor(Color::Black));
        }
        if game_ball.posy + (game_ball.radius as f32) >= size().unwrap().1 as f32 {  // Floor
            game_ball.vector.reflect_vector(Line {x1: 0, y1:size().unwrap().1 as i16, x2: size().unwrap().0 as i16, y2: size().unwrap().1 as i16 });
            stdout.queue(SetBackgroundColor(Color::Black));
        }
        game_ball.tick();




        let mut delay = (tick_time_target - start.elapsed().as_millis() as i64);
        if(delay.is_positive()) {
            thread::sleep(time::Duration::from_millis(delay as u64));

        }
        stdout.queue(SetTitle(format!("tick_delay: {}, game_ball.posx: {}, game_ball.posx: {}. game_ball.vector.speed: {}, game_ball.vector.direction: {}", delay, game_ball.posx, game_ball.posy, game_ball.vector.speed, game_ball.vector.direction))).expect("");


    }







        println!("{}", (1.0 as f32).atan().to_degrees());
}
