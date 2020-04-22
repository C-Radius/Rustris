use ggez::conf::{FullscreenType, WindowMode};
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::graphics::*;
use ggez::{Context, GameResult};

pub struct Tetris {}

impl Tetris {
    pub fn new(_ctx: &mut Context) -> Tetris {
        Tetris {}
    }
}

impl EventHandler for Tetris {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        GameResult::Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        let d_param = DrawParam::new();
        d_param.color(Color::new(255f32, 0f32, 0f32, 1f32));
        let rect = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(20f32, 20f32, 320f32, 320f32),
            Color::new(1f32, 0f32, 0f32, 1f32),
        )
        .unwrap();
        graphics::draw(ctx, &rect, d_param).unwrap();
        graphics::present(ctx)
    }
}
