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

const GRID_WIDTH: u32 = 10;
const GRID_HEIGHT: u32 = 22;

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
            self.tetromino_next = Some(Tetromino::random(Point2::new(5.0, 0.0), Rotation::_0));
        }
        self.tetromino = self.tetromino_next.take();
        self.tetromino_next = Some(Tetromino::random(Point2::new(5.0, 0.0), Rotation::_0));
    }

    pub fn calculate_offset(grid: &Grid, tetromino: &Tetromino) -> Point2<f32> {
        let mut offset = Point2::new(0.0f32, 0.0f32);
        let grid_width = grid.width as f32 - 1.0;

        tetromino.blocks().iter().for_each(|block| {
            let abs_tetromino_x = tetromino.position.x + block.position.x;
            if abs_tetromino_x >= grid_width {
                if grid_width - abs_tetromino_x < offset.x {
                    offset.x = grid_width - abs_tetromino_x;
                }
            } else if abs_tetromino_x < 0.0 {
                offset.x = abs_tetromino_x.abs();
            }
        });

        offset
    }

    pub fn is_next_pos_empty(&self, direction: &MoveDirection) -> bool {
        let mut ret_val = true;
        let mut tmp_tetromino = self.tetromino.unwrap().clone();

        match direction {
            MoveDirection::Left => {
                tmp_tetromino.position.x -= 1.0;
            }
            MoveDirection::Right => {
                tmp_tetromino.position.x += 1.0;
            }
            MoveDirection::Down => {
                tmp_tetromino.position.y += 1.0;
            }
            MoveDirection::Up => {}
        };

        tmp_tetromino.blocks().iter().for_each(|block| {
            if tmp_tetromino.position.x + block.position.x < self.grid.width as f32
                && tmp_tetromino.position.x + block.position.x >= 0.0
            {
                if self.grid.check_occupied(
                    (tmp_tetromino.position.x + block.position.x) as u32,
                    (tmp_tetromino.position.y + block.position.y) as u32,
                ) {
                    ret_val = false;
                }
            } else {
                ret_val = false;
            }
        });
        ret_val
    }

    pub fn move_tetromino(&mut self, direction: &MoveDirection) -> GameResult<()> {
        let grid = &self.grid;

        if self.is_next_pos_empty(direction) {
            match direction {
                MoveDirection::Left => {
                    let tetromino = self.tetromino.as_mut().unwrap();
                    if tetromino.position.x - 1.0 >= 0.0 {
                        tetromino.position.x -= 1.0;
                    }
                }
                MoveDirection::Right => {
                    let tetromino = self.tetromino.as_mut().unwrap();
                    if tetromino.position.x + 1.0 < grid.width as f32 {
                        tetromino.position.x += 1.0;
                    }
                }
                MoveDirection::Up => {
                    let tetromino = self.tetromino.as_mut().unwrap();
                    tetromino.rotation.rotate_cw();
                }
                MoveDirection::Down => {
                    let tetromino = self.tetromino.as_mut().unwrap();
                    tetromino.position.y += 1.0;
                }
            }
        }

        let offset = Tetris::calculate_offset(&self.grid, &self.tetromino.as_ref().unwrap());
        let tetromino = self.tetromino.as_mut().unwrap();
        tetromino.position.x += offset.x;
        tetromino.position.y += offset.y;

        if tetromino.position.y == GRID_HEIGHT as f32 - 4.0 as f32 {
            //todo: We need to fix this so it calculates according to all blocks in tetromino
            self.lock_tetromino();
            self.generate_tetromino();
        }
        Ok(())
    }

    pub fn lock_tetromino(&mut self) {
        self.tetromino
            .as_ref()
            .unwrap()
            .blocks()
            .iter()
            .for_each(|block| {
                let abs_block_x = self.tetromino.as_ref().unwrap().position.x + block.position.x;
                let abs_block_y = self.tetromino.as_ref().unwrap().position.y + block.position.y;

                self.grid.blocks[abs_block_x as usize][abs_block_y as usize] = Block {
                    position: Point2::new(abs_block_x, abs_block_y),
                    color: block.color,
                    state: BlockState::Filled,
                };
            });
    }

    pub fn draw_grid(&self, ctx: &mut Context) -> GameResult<()> {
        let mut grid = graphics::MeshBuilder::new();
        self.grid.blocks.iter().for_each(|x| {
            x.iter().for_each(|y| {
                grid.rectangle(
                    match y.state {
                        BlockState::Filled => DrawMode::fill(),
                        BlockState::Empty => DrawMode::Stroke(StrokeOptions::default()),
                    },
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
        tet.blocks().iter().for_each(|x| {
            tetromino.rectangle(
                DrawMode::fill(),
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
                    self.tetromino.as_mut().unwrap().rotation.rotate_cw();
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
