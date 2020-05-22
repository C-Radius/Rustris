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
const MILLS_PER_LOCK: u64 = 500;

//Struct to keep track of where our tetromino is going.
pub enum MoveDirection {
    Left,
    Right,
    Up,
    Down,
}

//Struct to hold the entire game state.
pub struct Rustris {
    grid: Grid,
    tetromino: Option<Tetromino>,
    tetromino_next: Option<Tetromino>,
    game_running: bool,
    move_tetromino_down: std::time::Duration,
    last_update: Instant,
    to_lock: bool,
    lock_timer: Duration,
    score: u64,
}

impl Rustris {
    pub fn new(_ctx: &mut Context) -> Rustris {
        //Create new game state.
        Rustris {
            grid: Grid::new(GRID_WIDTH, GRID_HEIGHT, Color::new(0.5, 0.5, 0.5, 1.0)),
            tetromino: None,
            tetromino_next: None,
            game_running: false,
            move_tetromino_down: Duration::new(0, 0),
            last_update: Instant::now(),
            to_lock: false,
            lock_timer: Duration::new(0, 0),
            score: 0,
        }
    }

    //Draw intro text here, this happens while game is not palyed.
    pub fn draw_intro(&mut self, ctx: &mut Context) -> GameResult<()> {
        let info_text = graphics::Text::new("Press any key to start.");
        info_text.draw(ctx, DrawParam::new().dest(Point2::new(30.0, 30.0)))
    }

    //Generates a random tetromino.
    pub fn generate_tetromino(&mut self) {
        if self.tetromino_next.is_none() {
            self.tetromino_next = Some(Tetromino::random(Point2::new(5.0, 0.0), Rotation::_0));
        }
        self.tetromino = self.tetromino_next.take();
        self.tetromino_next = Some(Tetromino::random(Point2::new(5.0, 0.0), Rotation::_0));
    }

    //Calculates the offset that should be applied on the tetromino in order for its
    //position to be a valid one in the grid
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
                if abs_tetromino_x.abs() > offset.x {
                    offset.x = abs_tetromino_x.abs();
                }
            }
        });

        offset
    }

    //Validates if the incoming move is a proper one. If it is it updates our tetromino
    //with its new values.
    pub fn validate_move(&mut self, direction: &MoveDirection) -> Option<Tetromino> {
        let mut next_pos_empty = true;
        let mut tetromino = self.tetromino.unwrap().clone();

        match direction {
            MoveDirection::Left => {
                tetromino.position.x -= 1.0;
            }
            MoveDirection::Right => {
                tetromino.position.x += 1.0;
            }
            MoveDirection::Down => {
                tetromino.position.y += 1.0;
            }
            MoveDirection::Up => {
                tetromino.rotation.rotate_cw();
                let offset = Rustris::calculate_offset(&self.grid, &tetromino);
                tetromino.position.x += offset.x;
                tetromino.position.y += offset.y;
            }
        };

        tetromino.blocks().iter().for_each(|block| {
            if self.grid.check_occupied(
                (tetromino.position.x + block.position.x) as u32,
                (tetromino.position.y + block.position.y) as u32,
            ) {
                next_pos_empty = false;
            }
        });

        if next_pos_empty && self.to_lock {
            self.to_lock = false;
            self.lock_timer = Duration::from_millis(0);
        }

        match next_pos_empty {
            true => Some(tetromino),
            false => None,
        }
    }

    //Moves our tetromino to the its new position.
    pub fn move_tetromino(&mut self, direction: &MoveDirection) -> GameResult<()> {
        //Check if incoming move is valid. If yes, swap current tetromino with new one.
        //If not, keep current tetromino
        self.tetromino = self.validate_move(direction).or(self.tetromino);

        //Check if tetromino reached the lowest point of our grid. If yes, lock it up and
        //generate a new one.
        if self.validate_move(&MoveDirection::Down).is_none() && self.to_lock == false {
            self.to_lock = true;
        }

        Ok(())
    }

    //Locks tetromino on the grid.
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

    //Draws the grid for our game.
    pub fn draw_grid(&self, ctx: &mut Context) -> GameResult<()> {
        let mut grid = graphics::MeshBuilder::new();
        self.grid.blocks.iter().for_each(|x| {
            x.iter().for_each(|y| {
                match y.state {
                    BlockState::Filled => {
                        grid.rectangle(
                            DrawMode::fill(),
                            Rect::new(
                                y.position.x * BLOCK_WIDTH,
                                y.position.y * BLOCK_HEIGHT,
                                BLOCK_WIDTH,
                                BLOCK_HEIGHT,
                            ),
                            y.color,
                        );
                        //Draw outline
                        grid.rectangle(
                            DrawMode::stroke(2.0),
                            Rect::new(
                                y.position.x * BLOCK_WIDTH,
                                y.position.y * BLOCK_HEIGHT,
                                BLOCK_WIDTH,
                                BLOCK_HEIGHT,
                            ),
                            Color::new(0.5, 0.5, 0.5, 1.0),
                        );
                    }
                    BlockState::Empty => {}
                };
            })
        });

        grid.rectangle(
            DrawMode::stroke(2.0),
            Rect::new(
                0.0,
                0.0,
                GRID_WIDTH as f32 * BLOCK_WIDTH,
                GRID_HEIGHT as f32 * BLOCK_HEIGHT,
            ),
            Color::new(0.5, 0.5, 0.5, 1.0),
        );

        let d_param = DrawParam::default().dest(Point2::new(
            (graphics::size(&ctx).0 / 2.0) - (BLOCK_WIDTH * GRID_WIDTH as f32) as f32 / 2.0,
            20.0,
        ));

        let mesh = grid.build(ctx).unwrap();
        graphics::draw(ctx, &mesh, d_param)
    }

    ///Draws our tetromino in the grid on its current position.
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
            tetromino.rectangle(
                DrawMode::stroke(2.0),
                Rect::new(
                    (tet.position.x + x.position.x) * BLOCK_WIDTH,
                    (tet.position.y + x.position.y) * BLOCK_HEIGHT,
                    BLOCK_WIDTH,
                    BLOCK_HEIGHT,
                ),
                Color::new(0.5, 0.5, 0.5, 1.0),
            );
        });

        let d_param = DrawParam::default().dest(Point2::new(
            (graphics::size(&ctx).0 / 2.0) - (BLOCK_WIDTH * GRID_WIDTH as f32) as f32 / 2.0,
            20.0,
        ));
        let mesh = tetromino.build(ctx).unwrap();
        graphics::draw(ctx, &mesh, d_param)
    }

    pub fn draw_next_tetromino(&self, ctx: &mut Context) -> GameResult<()> {
        let mut tetromino = graphics::MeshBuilder::new();
        let tet = self.tetromino_next.as_ref().unwrap();

        tet.blocks().iter().for_each(|x| {
            tetromino.rectangle(
                DrawMode::fill(),
                Rect::new(
                    (2.0 + x.position.x) * BLOCK_WIDTH,
                    (2.0 + x.position.y) * BLOCK_HEIGHT,
                    BLOCK_WIDTH,
                    BLOCK_HEIGHT,
                ),
                x.color,
            );
            tetromino.rectangle(
                DrawMode::stroke(2.0),
                Rect::new(
                    (2.0 + x.position.x) * BLOCK_WIDTH,
                    (2.0 + x.position.y) * BLOCK_HEIGHT,
                    BLOCK_WIDTH,
                    BLOCK_HEIGHT,
                ),
                Color::new(0.5, 0.5, 0.5, 1.0),
            );
        });

        let d_param = DrawParam::default().dest(Point2::new(
            ((graphics::size(&ctx).0 / 2.0) + (BLOCK_WIDTH * GRID_WIDTH as f32) as f32 / 2.0)
                + 20.0,
            20.0,
        ));
        let mesh = tetromino.build(ctx).unwrap();
        graphics::draw(ctx, &mesh, d_param)
    }

    fn update_score(&mut self, clears: &LineClears) {
        //Update the score
        match clears {
            LineClears::NoClear => {}
            LineClears::Single => {
                self.score += 1000;
            }
            LineClears::Double => {
                self.score += 4000;
            }
            LineClears::Tripple => {
                self.score += 6000;
            }
            LineClears::Rustris => {
                self.score += 12000;
            }
        }
    }

    fn draw_score(&mut self, ctx: &mut Context) -> GameResult<()> {
        let info_text = graphics::Text::new(format!("Score: {}", self.score));
        info_text.draw(ctx, DrawParam::new().dest(Point2::new(30.0, 30.0)))?;

        Ok(())
    }
}

impl EventHandler for Rustris {
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
                if self.to_lock == true {
                    self.lock_timer = self.lock_timer.add(self.last_update.elapsed());
                }

                if self.lock_timer >= Duration::from_millis(MILLS_PER_LOCK) && self.to_lock == true
                {
                    self.to_lock = false;
                    self.lock_timer = Duration::new(0, 0);
                    self.lock_tetromino();
                    self.generate_tetromino();
                } else {
                    if self.move_tetromino_down.as_millis() >= MOVE_TETROMINO_EVERY {
                        self.move_tetromino(&MoveDirection::Down)?;
                        self.move_tetromino_down = Duration::new(0, 0);
                    }
                }
            }
            self.last_update = Instant::now();
        }

        if self.lock_timer != Duration::new(0, 0) {}
        //Clear filled lines and get the number of lines cleared back.
        let cleared = self.grid.clear_lines();
        self.update_score(&cleared);

        GameResult::Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        if !self.game_running {
            self.draw_intro(ctx).unwrap();
        } else {
            self.draw_score(ctx).unwrap();
            self.draw_grid(ctx).unwrap();
            self.draw_tetromino(ctx)?;
            self.draw_next_tetromino(ctx)?;
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
        repeat: bool,
    ) {
        if self.game_running {
            //Todo
            //Make this more ... Professional?
            self.lock_timer = Duration::from_millis(0);
            match keycode {
                KeyCode::Up => {
                    if !repeat {
                        self.move_tetromino(&MoveDirection::Up).unwrap();
                    }
                }
                KeyCode::Down => {
                    self.move_tetromino(&MoveDirection::Down).unwrap();
                }
                KeyCode::Left => {
                    self.move_tetromino(&MoveDirection::Left).unwrap();
                }
                KeyCode::Right => {
                    self.move_tetromino(&MoveDirection::Right).unwrap();
                }
                KeyCode::Escape => {
                    ggez::event::quit(ctx);
                }
                _ => (),
            }
        }
    }
}
