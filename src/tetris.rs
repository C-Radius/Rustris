#![allow(dead_code)]
use crate::types::Tetromino;
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::graphics::*;
use ggez::{Context, GameResult};

use crate::types::*;
use ggez::nalgebra::geometry::Point2;

pub struct Tetris {
    grid: Grid,
    tetromino: Option<Tetromino>,
    tetromino_next: Option<Tetromino>,
    score: u32,
}

impl Tetris {
    pub fn new(_ctx: &mut Context) -> Tetris {
        Tetris {
            grid: Grid::new(11, 21, &Color::new(0.5, 0.5, 0.5, 0.5)),
            tetromino: None,
            tetromino_next: None,
            score: 0,
        }
    }

    pub fn generate_tetromino(&mut self) {
        self.tetromino = Some(Tetromino::new(
            Point2::new(0.0, 0.0),
            TetrominoType::Random,
            Color::new(1.0, 0.0, 0.0, 1.0),
        ))
        /*
        match self.tetromino_next.as_ref().as_mut() {
            Some(tetr) => {
                let mut t = self.tetromino.as_ref().as_mut().unwrap();
                t = tetr;

                let mut new_t = Tetromino::new(
                    Point2::new(5.0, 0.0),
                    TetrominoType::Random,
                    Color::new(1.0, 0.0, 0.0, 1.0),
                );

                tetr = &new_t;
            }
            None => {}
        }
        */
    }

    pub fn draw_grid(&self, ctx: &mut Context) -> GameResult<()> {
        let mut grid = graphics::MeshBuilder::new();
        self.grid.blocks.iter().for_each(|x| {
            x.iter().for_each(|y| {
                grid.rectangle(
                    DrawMode::Stroke(StrokeOptions::default()),
                    Rect::new(
                        y.position.x * BLOCK_WIDTH,
                        y.position.y * BLOCK_HEIGHT,
                        BLOCK_WIDTH,
                        BLOCK_HEIGHT,
                    ),
                    y.color,
                );
            })
        });

        let d_param = DrawParam::default();
        let mesh = grid.build(ctx).unwrap();
        graphics::draw(ctx, &mesh, d_param)
    }

    #[allow(unused_variables)]
    pub fn draw_tetromino(&self, ctx: &mut Context) -> GameResult<()> {
        GameResult::Ok(())
    }
}

impl EventHandler for Tetris {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        GameResult::Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        self.draw_grid(ctx).unwrap();
        graphics::present(ctx)
    }
}
