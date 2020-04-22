use ggez::conf::FullscreenType;
use ggez::conf::WindowMode;
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::{Context, ContextBuilder, GameResult};

mod grid;
mod tetris;
mod tetromino;
mod util;

use crate::tetris::Tetris;

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("Tetris", "Chris Kritsotalakis")
        .window_mode(WindowMode {
            width: 1920.0f32,
            height: 1080.0f32,
            maximized: true,
            fullscreen_type: FullscreenType::True,
            borderless: true,
            min_width: 0.0,
            max_width: 0.0,
            min_height: 0.0,
            max_height: 0.0,
            resizable: false,
        })
        .build()
        .unwrap();

    let mut tetris = Tetris::new(&mut ctx);
    match event::run(&mut ctx, &mut event_loop, &mut tetris) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}
