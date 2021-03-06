use ggez::conf::FullscreenType;
use ggez::conf::WindowMode;
use ggez::event;
use ggez::ContextBuilder;

mod rustris;
mod types;

use rustris::Rustris;

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("Tetris", "Chris Kritsotalakis")
        .window_mode(WindowMode {
            width: 800.0f32,
            height: 600.0f32,
            maximized: false,
            fullscreen_type: FullscreenType::Windowed,
            borderless: false,
            min_width: 0.0,
            max_width: 0.0,
            min_height: 0.0,
            max_height: 0.0,
            resizable: false,
        })
        .build()
        .unwrap();

    let mut rustris = Rustris::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut rustris) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}
