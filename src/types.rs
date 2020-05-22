use crate::types::TetrominoType::{I, J, L, O, S, T, Z};
use ggez::graphics::Color;
use ggez::nalgebra::geometry::Point2;
use rand::distributions::{Distribution, Standard};

//I
const COLOR_I: Color = Color::new(42.0 / 255.0, 80.0 / 255.0, 230.0 / 255.0, 255.0 / 255.0);
//O
const COLOR_O: Color = Color::new(255.0 / 255.0, 242.0 / 255.0, 117.0 / 255.0, 255.0 / 255.0);
//T
const COLOR_T: Color = Color::new(230.0 / 255.0, 46.0 / 255.0, 187.0 / 255.0, 255.0 / 255.0);
//S
const COLOR_S: Color = Color::new(51.0 / 255.0, 243.0 / 255.0, 115.0 / 255.0, 255.0 / 255.0);
//Z
const COLOR_Z: Color = Color::new(255.0 / 255.0, 136.0 / 255.0, 16.0 / 255.0, 255.0 / 255.0);
//J
const COLOR_J: Color = Color::new(31.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0);
//L
const COLOR_L: Color = Color::new(208.0 / 255.0, 54.0 / 255.0, 54.0 / 255.0, 255.0 / 255.0);

pub const BLOCK_WIDTH: f32 = 25.0;
pub const BLOCK_HEIGHT: f32 = 25.0;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlockState {
    Empty,
    Filled,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Block {
    pub position: Point2<f32>,
    pub color: Color,
    pub state: BlockState,
}

impl Block {
    pub fn new(position: Point2<f32>, color: Color, state: BlockState) -> Block {
        Block {
            position: position,
            color: color,
            state: state,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Rotation {
    _0,
    _90,
    _180,
    _270,
}

impl Rotation {
    pub fn rotate_cw(&mut self) -> &Rotation {
        match self {
            Rotation::_0 => *self = Rotation::_90,
            Rotation::_90 => *self = Rotation::_180,
            Rotation::_180 => *self = Rotation::_270,
            Rotation::_270 => *self = Rotation::_0,
        }
        self
    }
    #[allow(dead_code)]
    pub fn rotate_ccw(&mut self) -> &Rotation {
        match self {
            Rotation::_0 => *self = Rotation::_270,
            Rotation::_270 => *self = Rotation::_180,
            Rotation::_180 => *self = Rotation::_90,
            Rotation::_90 => *self = Rotation::_0,
        }
        self
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TetrominoType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl Distribution<TetrominoType> for Standard {
    fn sample<R>(&self, rng: &mut R) -> TetrominoType
    where
        R: rand::Rng + ?std::marker::Sized,
    {
        match rng.gen_range(0, 7) {
            0 => I,
            1 => O,
            2 => T,
            3 => S,
            4 => Z,
            5 => J,
            6 | _ => L,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Tetromino {
    pub position: Point2<f32>,
    pub rotation: Rotation,
    pub t_type: TetrominoType,
}

impl Tetromino {
    pub fn random(position: Point2<f32>, rotation: Rotation) -> Tetromino {
        Tetromino {
            position: position,
            rotation: rotation,
            t_type: rand::random(),
        }
    }

    pub fn blocks(&self) -> Vec<Block> {
        match self.t_type {
            TetrominoType::I => match self.rotation {
                Rotation::_0 => vec![
                    Block::new(Point2::new(-2.0, 0.0), COLOR_I, BlockState::Filled),
                    Block::new(Point2::new(-1.0, 0.0), COLOR_I, BlockState::Filled),
                    Block::new(Point2::new(0.0, 0.0), COLOR_I, BlockState::Filled),
                    Block::new(Point2::new(1.0, 0.0), COLOR_I, BlockState::Filled),
                ],
                Rotation::_90 => vec![
                    Block::new(Point2::new(0.0, 2.0), COLOR_I, BlockState::Filled),
                    Block::new(Point2::new(0.0, 1.0), COLOR_I, BlockState::Filled),
                    Block::new(Point2::new(0.0, 0.0), COLOR_I, BlockState::Filled),
                    Block::new(Point2::new(0.0, -1.0), COLOR_I, BlockState::Filled),
                ],
                Rotation::_180 => vec![
                    Block::new(Point2::new(-2.0, 0.0), COLOR_I, BlockState::Filled),
                    Block::new(Point2::new(-1.0, 0.0), COLOR_I, BlockState::Filled),
                    Block::new(Point2::new(0.0, 0.0), COLOR_I, BlockState::Filled),
                    Block::new(Point2::new(1.0, 0.0), COLOR_I, BlockState::Filled),
                ],
                Rotation::_270 => vec![
                    Block::new(Point2::new(0.0, 2.0), COLOR_I, BlockState::Filled),
                    Block::new(Point2::new(0.0, 1.0), COLOR_I, BlockState::Filled),
                    Block::new(Point2::new(0.0, 0.0), COLOR_I, BlockState::Filled),
                    Block::new(Point2::new(0.0, -1.0), COLOR_I, BlockState::Filled),
                ],
            },
            TetrominoType::O => match self.rotation {
                Rotation::_0 => vec![
                    Block::new(Point2::new(0.0, 0.0), COLOR_O, BlockState::Filled),
                    Block::new(Point2::new(1.0, 0.0), COLOR_O, BlockState::Filled),
                    Block::new(Point2::new(0.0, 1.0), COLOR_O, BlockState::Filled),
                    Block::new(Point2::new(1.0, 1.0), COLOR_O, BlockState::Filled),
                ],
                Rotation::_90 => vec![
                    Block::new(Point2::new(0.0, 0.0), COLOR_O, BlockState::Filled),
                    Block::new(Point2::new(1.0, 0.0), COLOR_O, BlockState::Filled),
                    Block::new(Point2::new(0.0, 1.0), COLOR_O, BlockState::Filled),
                    Block::new(Point2::new(1.0, 1.0), COLOR_O, BlockState::Filled),
                ],
                Rotation::_180 => vec![
                    Block::new(Point2::new(0.0, 0.0), COLOR_O, BlockState::Filled),
                    Block::new(Point2::new(1.0, 0.0), COLOR_O, BlockState::Filled),
                    Block::new(Point2::new(0.0, 1.0), COLOR_O, BlockState::Filled),
                    Block::new(Point2::new(1.0, 1.0), COLOR_O, BlockState::Filled),
                ],
                Rotation::_270 => vec![
                    Block::new(Point2::new(0.0, 0.0), COLOR_O, BlockState::Filled),
                    Block::new(Point2::new(1.0, 0.0), COLOR_O, BlockState::Filled),
                    Block::new(Point2::new(0.0, 1.0), COLOR_O, BlockState::Filled),
                    Block::new(Point2::new(1.0, 1.0), COLOR_O, BlockState::Filled),
                ],
            },
            TetrominoType::T => match self.rotation {
                Rotation::_0 => vec![
                    Block::new(Point2::new(0.0, 0.0), COLOR_T, BlockState::Filled),
                    Block::new(Point2::new(-1.0, 0.0), COLOR_T, BlockState::Filled),
                    Block::new(Point2::new(1.0, 0.0), COLOR_T, BlockState::Filled),
                    Block::new(Point2::new(0.0, 1.0), COLOR_T, BlockState::Filled),
                ],
                Rotation::_90 => vec![
                    Block::new(Point2::new(0.0, 0.0), COLOR_T, BlockState::Filled),
                    Block::new(Point2::new(0.0, 1.0), COLOR_T, BlockState::Filled),
                    Block::new(Point2::new(0.0, -1.0), COLOR_T, BlockState::Filled),
                    Block::new(Point2::new(1.0, 0.0), COLOR_T, BlockState::Filled),
                ],
                Rotation::_180 => vec![
                    Block::new(Point2::new(0.0, 0.0), COLOR_T, BlockState::Filled),
                    Block::new(Point2::new(-1.0, 0.0), COLOR_T, BlockState::Filled),
                    Block::new(Point2::new(1.0, 0.0), COLOR_T, BlockState::Filled),
                    Block::new(Point2::new(0.0, -1.0), COLOR_T, BlockState::Filled),
                ],
                Rotation::_270 => vec![
                    Block::new(Point2::new(0.0, 0.0), COLOR_T, BlockState::Filled),
                    Block::new(Point2::new(0.0, 1.0), COLOR_T, BlockState::Filled),
                    Block::new(Point2::new(0.0, -1.0), COLOR_T, BlockState::Filled),
                    Block::new(Point2::new(-1.0, 0.0), COLOR_T, BlockState::Filled),
                ],
            },
            TetrominoType::S => match self.rotation {
                Rotation::_0 => vec![
                    Block::new(Point2::new(0.0, 0.0), COLOR_S, BlockState::Filled),
                    Block::new(Point2::new(1.0, 0.0), COLOR_S, BlockState::Filled),
                    Block::new(Point2::new(0.0, -1.0), COLOR_S, BlockState::Filled),
                    Block::new(Point2::new(-1.0, -1.0), COLOR_S, BlockState::Filled),
                ],
                Rotation::_90 => vec![
                    Block::new(Point2::new(0.0, 0.0), COLOR_S, BlockState::Filled),
                    Block::new(Point2::new(0.0, -1.0), COLOR_S, BlockState::Filled),
                    Block::new(Point2::new(-1.0, 0.0), COLOR_S, BlockState::Filled),
                    Block::new(Point2::new(-1.0, 1.0), COLOR_S, BlockState::Filled),
                ],
                Rotation::_180 => vec![
                    Block::new(Point2::new(0.0, 0.0), COLOR_S, BlockState::Filled),
                    Block::new(Point2::new(1.0, 0.0), COLOR_S, BlockState::Filled),
                    Block::new(Point2::new(0.0, -1.0), COLOR_S, BlockState::Filled),
                    Block::new(Point2::new(-1.0, -1.0), COLOR_S, BlockState::Filled),
                ],
                Rotation::_270 => vec![
                    Block::new(Point2::new(0.0, 0.0), COLOR_S, BlockState::Filled),
                    Block::new(Point2::new(0.0, -1.0), COLOR_S, BlockState::Filled),
                    Block::new(Point2::new(-1.0, 0.0), COLOR_S, BlockState::Filled),
                    Block::new(Point2::new(-1.0, 1.0), COLOR_S, BlockState::Filled),
                ],
            },
            TetrominoType::Z => match self.rotation {
                Rotation::_0 => vec![
                    Block::new(Point2::new(0.0, 0.0), COLOR_Z, BlockState::Filled),
                    Block::new(Point2::new(-1.0, 0.0), COLOR_Z, BlockState::Filled),
                    Block::new(Point2::new(0.0, -1.0), COLOR_Z, BlockState::Filled),
                    Block::new(Point2::new(1.0, -1.0), COLOR_Z, BlockState::Filled),
                ],
                Rotation::_90 => vec![
                    Block::new(Point2::new(0.0, 0.0), COLOR_Z, BlockState::Filled),
                    Block::new(Point2::new(0.0, 1.0), COLOR_Z, BlockState::Filled),
                    Block::new(Point2::new(-1.0, 0.0), COLOR_Z, BlockState::Filled),
                    Block::new(Point2::new(-1.0, -1.0), COLOR_Z, BlockState::Filled),
                ],
                Rotation::_180 => vec![
                    Block::new(Point2::new(0.0, 0.0), COLOR_Z, BlockState::Filled),
                    Block::new(Point2::new(-1.0, 0.0), COLOR_Z, BlockState::Filled),
                    Block::new(Point2::new(0.0, -1.0), COLOR_Z, BlockState::Filled),
                    Block::new(Point2::new(1.0, -1.0), COLOR_Z, BlockState::Filled),
                ],
                Rotation::_270 => vec![
                    Block::new(Point2::new(0.0, 0.0), COLOR_Z, BlockState::Filled),
                    Block::new(Point2::new(0.0, 1.0), COLOR_Z, BlockState::Filled),
                    Block::new(Point2::new(-1.0, 0.0), COLOR_Z, BlockState::Filled),
                    Block::new(Point2::new(-1.0, -1.0), COLOR_Z, BlockState::Filled),
                ],
            },
            TetrominoType::J => match self.rotation {
                Rotation::_0 => vec![
                    Block::new(Point2::new(0.0, 0.0), COLOR_J, BlockState::Filled),
                    Block::new(Point2::new(-1.0, 0.0), COLOR_J, BlockState::Filled),
                    Block::new(Point2::new(-1.0, 1.0), COLOR_J, BlockState::Filled),
                    Block::new(Point2::new(1.0, 0.0), COLOR_J, BlockState::Filled),
                ],
                Rotation::_90 => vec![
                    Block::new(Point2::new(0.0, 0.0), COLOR_J, BlockState::Filled),
                    Block::new(Point2::new(0.0, 1.0), COLOR_J, BlockState::Filled),
                    Block::new(Point2::new(1.0, 1.0), COLOR_J, BlockState::Filled),
                    Block::new(Point2::new(0.0, -1.0), COLOR_J, BlockState::Filled),
                ],
                Rotation::_180 => vec![
                    Block::new(Point2::new(0.0, 0.0), COLOR_J, BlockState::Filled),
                    Block::new(Point2::new(1.0, 0.0), COLOR_J, BlockState::Filled),
                    Block::new(Point2::new(1.0, -1.0), COLOR_J, BlockState::Filled),
                    Block::new(Point2::new(-1.0, 0.0), COLOR_J, BlockState::Filled),
                ],
                Rotation::_270 => vec![
                    Block::new(Point2::new(0.0, 0.0), COLOR_J, BlockState::Filled),
                    Block::new(Point2::new(0.0, -1.0), COLOR_J, BlockState::Filled),
                    Block::new(Point2::new(-1.0, -1.0), COLOR_J, BlockState::Filled),
                    Block::new(Point2::new(0.0, 1.0), COLOR_J, BlockState::Filled),
                ],
            },
            TetrominoType::L => match self.rotation {
                Rotation::_0 => vec![
                    Block::new(Point2::new(0.0, 0.0), COLOR_L, BlockState::Filled),
                    Block::new(Point2::new(-1.0, 0.0), COLOR_L, BlockState::Filled),
                    Block::new(Point2::new(-1.0, -1.0), COLOR_L, BlockState::Filled),
                    Block::new(Point2::new(1.0, 0.0), COLOR_L, BlockState::Filled),
                ],
                Rotation::_90 => vec![
                    Block::new(Point2::new(0.0, 0.0), COLOR_L, BlockState::Filled),
                    Block::new(Point2::new(0.0, 1.0), COLOR_L, BlockState::Filled),
                    Block::new(Point2::new(-1.0, 1.0), COLOR_L, BlockState::Filled),
                    Block::new(Point2::new(0.0, -1.0), COLOR_L, BlockState::Filled),
                ],
                Rotation::_180 => vec![
                    Block::new(Point2::new(0.0, 0.0), COLOR_L, BlockState::Filled),
                    Block::new(Point2::new(1.0, 0.0), COLOR_L, BlockState::Filled),
                    Block::new(Point2::new(1.0, 1.0), COLOR_L, BlockState::Filled),
                    Block::new(Point2::new(-1.0, 0.0), COLOR_L, BlockState::Filled),
                ],
                Rotation::_270 => vec![
                    Block::new(Point2::new(0.0, 0.0), COLOR_L, BlockState::Filled),
                    Block::new(Point2::new(0.0, -1.0), COLOR_L, BlockState::Filled),
                    Block::new(Point2::new(1.0, -1.0), COLOR_L, BlockState::Filled),
                    Block::new(Point2::new(0.0, 1.0), COLOR_L, BlockState::Filled),
                ],
            },
        }
    }
}

#[derive(Debug)]
pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub blocks: Vec<Vec<Block>>,
}

impl Grid {
    pub fn new(width: u32, height: u32, color: Color) -> Grid {
        Grid {
            width: width,
            height: height,
            blocks: (0..width)
                .into_iter()
                .map(|x| {
                    (0..height)
                        .into_iter()
                        .map(|y| Block {
                            position: Point2::new(x as f32, y as f32),
                            color: color,
                            state: BlockState::Empty,
                        })
                        .collect::<Vec<Block>>()
                })
                .collect::<Vec<Vec<Block>>>(),
        }
    }

    pub fn find_line_clears(&mut self) -> Vec<u32> {
        let mut empty_found: bool = false;
        let mut lines: Vec<u32> = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if self.blocks[x as usize][y as usize].state == BlockState::Empty {
                    empty_found = true;
                }
            }
            if !empty_found {
                lines.push(y);
            }
            empty_found = false;
        }
        lines
    }

    pub fn clear_line(&mut self, line_index: u32) -> bool {
        if line_index >= self.height {
            return false;
        }
        for (index, row) in self.blocks.iter_mut().enumerate() {
            row.remove(line_index as usize);
            row.insert(
                0,
                Block::new(
                    Point2::new(index as f32, 0 as f32),
                    Color::new(0.5, 0.5, 0.5, 1.0),
                    BlockState::Empty,
                ),
            );
        }
        true
    }

    pub fn clear_lines(&mut self) -> bool {
        let lines = self.find_line_clears();

        if !lines.is_empty() {
            lines.iter().for_each(|line| {
                self.clear_line(*line);
            });
        }

        for (x, row) in self.blocks.iter_mut().enumerate() {
            for (y, col) in row.iter_mut().enumerate() {
                col.position.x = x as f32;
                col.position.y = y as f32;
            }
        }
        true
    }

    pub fn check_occupied(&self, x: u32, y: u32) -> bool {
        if x < self.width && y < self.height {
            if self.blocks[x as usize][y as usize].state == BlockState::Empty {
                return false;
            }
        }
        true
    }

    pub fn reset(&mut self) {
        self.blocks
            .iter_mut()
            .for_each(|x| x.iter_mut().for_each(|y| y.state = BlockState::Empty));
    }
}

