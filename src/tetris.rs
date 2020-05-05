use crate::types::Tetromino;
use ggez::event::EventHandler;
use ggez::event::KeyMods;
use ggez::graphics;
use ggez::graphics::*;
use ggez::input::{self, keyboard::KeyCode};
use ggez::timer;
use ggez::{Context, GameResult};
use std::ops::Add;
use std::time::{Duration, Instant};

use crate::types::*;
use ggez::nalgebra::geometry::Point2;

const MOVE_TETROMINO_EVERY: u128 = 500;

const GRID_WIDTH: u32 = 11;
const GRID_HEIGHT: u32 = 21;

const UPDATES_PER_SECOND: f32 = 4.0;
const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;

pub enum MoveDirection {
    Left,
    Right,
    Up,
    Down,
}

pub struct Tetris {
    grid: Grid,
    tetromino: Option<Tetromino>,
    tetromino_next: Option<Tetromino>,
    game_running: bool,
    move_tetromino_down: std::time::Duration,
    last_update: Instant,
}

impl Tetris {
    pub fn new(_ctx: &mut Context) -> Tetris {
        Tetris {
            grid: Grid::new(GRID_WIDTH, GRID_HEIGHT, Color::new(0.5, 0.5, 0.5, 1.0)),
            tetromino: None,
            tetromino_next: None,
            game_running: false,
            move_tetromino_down: Duration::new(0, 0),
            last_update: Instant::now(),
        }
    }

    pub fn draw_intro(&mut self, ctx: &mut Context) -> GameResult<()> {
        let info_text = graphics::Text::new("Press any key to start.");
        info_text.draw(ctx, DrawParam::new().dest(Point2::new(30.0, 30.0)))
    }

    pub fn generate_tetromino(&mut self) {
        if self.tetromino_next.is_none() {
            self.tetromino_next =
                Some(Tetromino::new(Point2::new(5.0, 0.0), TetrominoType::Random));
        }
        self.tetromino = self.tetromino_next.take();
        self.tetromino_next = Some(Tetromino::new(Point2::new(5.0, 0.0), TetrominoType::Random));
    }

    pub fn move_tetromino(&mut self, direction: &MoveDirection) {
        let tetromino = self.tetromino.as_mut().unwrap();
        let grid = &self.grid;

        match direction {
            MoveDirection::Left => {
                if tetromino.position.x - 1.0 >= 0.0 {
                    tetromino.position.x -= 1.0;
                }
            }
            MoveDirection::Right => {
                if tetromino.position.x + 1.0 < grid.width as f32 {
                    tetromino.position.x += 1.0;
                }
            }
            MoveDirection::Up => {
                tetromino.rotate();
            }
            MoveDirection::Down => {
                tetromino.position.y += 1.0;
            }
        }

        if tetromino.position.y >= GRID_HEIGHT as f32 {
            self.generate_tetromino();
        }
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

    pub fn draw_tetromino(&self, ctx: &mut Context) -> GameResult<()> {
        let mut tetromino = graphics::MeshBuilder::new();
        let tet = self.tetromino.as_ref().unwrap();
        tet.blocks.iter().for_each(|x| {
            tetromino.rectangle(
                DrawMode::stroke(3.0),
                Rect::new(
                    (tet.position.x + x.position.x) * BLOCK_WIDTH,
                    (tet.position.y + x.position.y) * BLOCK_HEIGHT,
                    BLOCK_WIDTH,
                    BLOCK_HEIGHT,
                ),
                x.color,
            );
        });

        let d_param = DrawParam::default();
        let mesh = tetromino.build(ctx).unwrap();
        graphics::draw(ctx, &mesh, d_param)
    }
}

impl EventHandler for Tetris {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.last_update.elapsed() >= Duration::from_millis(MILLIS_PER_UPDATE) {
            if !self.game_running {
                if input::keyboard::pressed_keys(ctx).len() > 0 {
                    self.grid.reset();
                    self.game_running = true;
                    self.generate_tetromino();
                }
            } else {
                self.move_tetromino_down = self.move_tetromino_down.add(self.last_update.elapsed());

                if self.move_tetromino_down.as_millis() >= MOVE_TETROMINO_EVERY {
                    self.move_tetromino(&MoveDirection::Down);
                    self.move_tetromino_down = Duration::new(0, 0);
                }

                if input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::Up) {
                    self.tetromino.as_mut().unwrap().rotate();
                }
            }
            self.last_update = Instant::now();
        }
        GameResult::Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let info_text = graphics::Text::new(format!("{:?}", timer::fps(ctx)));
        info_text.draw(ctx, DrawParam::new().dest(Point2::new(30.0, 30.0)))?;

        if !self.game_running {
            self.draw_intro(ctx).unwrap();
        } else {
            self.draw_grid(ctx).unwrap();
            self.draw_tetromino(ctx).unwrap();
        }
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if self.game_running {
            match keycode {
                KeyCode::Up => {
                    self.move_tetromino(&MoveDirection::Up);
                }
                KeyCode::Down => {
                    self.move_tetromino(&MoveDirection::Down);
                }
                KeyCode::Left => {
                    self.move_tetromino(&MoveDirection::Left);
                }
                KeyCode::Right => {
                    self.move_tetromino(&MoveDirection::Right);
                }
                KeyCode::Escape => {
                    ggez::event::quit(ctx);
                }
                _ => (),
            }
        }

        timer::sleep(Duration::new(0, 500));
    }
}
