#![allow(dead_code)]
use crate::types::TetrominoType::{I, J, L, O, S, T, Z};
use ggez::graphics::Color;
use ggez::nalgebra::{self as na, geometry::Point2};
use na::Rotation2;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub const BLOCK_WIDTH: f32 = 45.0;
pub const BLOCK_HEIGHT: f32 = 45.0;

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

    pub fn rotate(&mut self, deg: f32) -> &Block {
        self.position = Rotation2::new(deg.to_radians()) * self.position;
        self
    }

    pub fn coords_rounded(&self) -> Point2<f32> {
        Point2::new(
            self.position.coords.x.round(),
            self.position.coords.y.round(),
        )
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
    Random,
}

impl Distribution<TetrominoType> for Standard {
    fn sample<R>(&self, rng: &mut R) -> TetrominoType
    where
        R: rand::Rng + ?std::marker::Sized,
    {
        match rng.gen_range(0, 7) {
            0 => TetrominoType::I,
            1 => TetrominoType::O,
            2 => TetrominoType::T,
            3 => TetrominoType::S,
            4 => TetrominoType::Z,
            5 => TetrominoType::J,
            6 => TetrominoType::L,
            _ => TetrominoType::Random,
        }
    }
}

#[derive(Debug)]
pub struct Tetromino {
    pub position: Point2<f32>,
    pub t_tetromino: TetrominoType,
    pub color: Color,
    pub blocks: Vec<Block>,
}

impl Tetromino {
    pub fn new(position: Point2<f32>, t_tetromino: TetrominoType, color: Color) -> Tetromino {
        let tt = match t_tetromino {
            TetrominoType::Random => rand::random(),
            _ => t_tetromino,
        };

        Tetromino {
            position: position,
            t_tetromino: tt,
            color: color,
            blocks: match tt {
                I => Tetromino::tetromino_i(&color),
                O => Tetromino::tetromino_o(&color),
                T => Tetromino::tetromino_t(&color),
                S => Tetromino::tetromino_s(&color),
                Z => Tetromino::tetromino_z(&color),
                J => Tetromino::tetromino_j(&color),
                L => Tetromino::tetromino_l(&color),
                _ => vec![Block::new(Point2::new(0.0, 0.0), color, BlockState::Empty); 4],
            },
        }
    }

    pub fn rotate(&mut self) {
        self.blocks.iter_mut().for_each(|p: &mut Block| {
            p.rotate(90.0);
        });
    }

    fn tetromino_i(color: &Color) -> Vec<Block> {
        vec![
            Block {
                position: Point2::new(0.0, 0.0),
                color: *color,
                state: BlockState::Filled,
            },
            Block {
                position: Point2::new(0.0, 1.0),
                color: *color,
                state: BlockState::Filled,
            },
            Block {
                position: Point2::new(0.0, 2.0),
                color: *color,
                state: BlockState::Filled,
            },
            Block {
                position: Point2::new(0.0, 3.0),
                color: *color,
                state: BlockState::Filled,
            },
        ]
    }

    fn tetromino_o(color: &Color) -> Vec<Block> {
        vec![
            Block {
                position: Point2::new(0.0, 0.0),
                color: *color,
                state: BlockState::Filled,
            },
            Block {
                position: Point2::new(1.0, 0.0),
                color: *color,
                state: BlockState::Filled,
            },
            Block {
                position: Point2::new(0.0, 1.0),
                color: *color,
                state: BlockState::Filled,
            },
            Block {
                position: Point2::new(1.0, 1.0),
                color: *color,
                state: BlockState::Filled,
            },
        ]
    }

    fn tetromino_t(color: &Color) -> Vec<Block> {
        vec![
            Block {
                position: Point2::new(0.0, 0.0),
                color: *color,
                state: BlockState::Filled,
            },
            Block {
                position: Point2::new(-1.0, 0.0),
                color: *color,
                state: BlockState::Filled,
            },
            Block {
                position: Point2::new(1.0, 0.0),
                color: *color,
                state: BlockState::Filled,
            },
            Block {
                position: Point2::new(0.0, 1.0),
                color: *color,
                state: BlockState::Filled,
            },
        ]
    }

    fn tetromino_s(color: &Color) -> Vec<Block> {
        vec![
            Block {
                position: Point2::new(0.0, 0.0),
                color: *color,
                state: BlockState::Filled,
            },
            Block {
                position: Point2::new(-1.0, 0.0),
                color: *color,
                state: BlockState::Filled,
            },
            Block {
                position: Point2::new(0.0, 1.0),
                color: *color,
                state: BlockState::Filled,
            },
            Block {
                position: Point2::new(1.0, 1.0),
                color: *color,
                state: BlockState::Filled,
            },
        ]
    }
    fn tetromino_z(color: &Color) -> Vec<Block> {
        vec![
            Block {
                position: Point2::new(0.0, 0.0),
                color: *color,
                state: BlockState::Filled,
            },
            Block {
                position: Point2::new(1.0, 0.0),
                color: *color,
                state: BlockState::Filled,
            },
            Block {
                position: Point2::new(0.0, 1.0),
                color: *color,
                state: BlockState::Filled,
            },
            Block {
                position: Point2::new(-1.0, 1.0),
                color: *color,
                state: BlockState::Filled,
            },
        ]
    }

    fn tetromino_j(color: &Color) -> Vec<Block> {
        vec![
            Block {
                position: Point2::new(0.0, 0.0),
                color: *color,
                state: BlockState::Filled,
            },
            Block {
                position: Point2::new(0.0, 1.0),
                color: *color,
                state: BlockState::Filled,
            },
            Block {
                position: Point2::new(0.0, 2.0),
                color: *color,
                state: BlockState::Filled,
            },
            Block {
                position: Point2::new(-1.0, 0.0),
                color: *color,
                state: BlockState::Filled,
            },
        ]
    }

    fn tetromino_l(color: &Color) -> Vec<Block> {
        vec![
            Block {
                position: Point2::new(0.0, 0.0),
                color: *color,
                state: BlockState::Filled,
            },
            Block {
                position: Point2::new(0.0, 1.0),
                color: *color,
                state: BlockState::Filled,
            },
            Block {
                position: Point2::new(0.0, 2.0),
                color: *color,
                state: BlockState::Filled,
            },
            Block {
                position: Point2::new(1.0, 0.0),
                color: *color,
                state: BlockState::Filled,
            },
        ]
    }
}

#[derive(Debug)]
pub struct Grid {
    width: u32,
    height: u32,
    pub blocks: Vec<Vec<Block>>,
}

impl Grid {
    pub fn new(width: u32, height: u32, color: &Color) -> Grid {
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
                            color: *color,
                            state: BlockState::Empty,
                        })
                        .collect::<Vec<Block>>()
                })
                .collect::<Vec<Vec<Block>>>(),
        }
    }

    pub fn reset(&mut self) {
        self.blocks
            .iter_mut()
            .for_each(|x| x.iter_mut().for_each(|y| y.state = BlockState::Empty));
    }

    pub fn print(self) {
        for x in self.blocks {
            for y in x {
                print!("{}", format!("{} {}", y.position.x, y.position.y));
            }
            println!("");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_rotation() {
        let mut x = Block::new(
            Point2::new(-1.0, 0.0),
            Color::new(1.0, 1.0, 1.0, 1.0),
            BlockState::Filled,
        );
        let y = Block::new(
            Point2::new(0.0, 1.0),
            Color::new(1.0, 1.0, 1.0, 1.0),
            BlockState::Filled,
        );

        println!("{:?}\n{:?}", x, y);
        assert_eq!(x.rotate(-90.0f32).coords_rounded(), y.position);
    }
}
