use crate::tetromino::TetrominoType::{I, J, L, O, S, T, Z};
use crate::util::{Block, Point2D};
use ggez::graphics::Color;

#[derive(Copy, Clone, Debug)]
pub enum TetrominoType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

#[derive(Debug)]
pub struct Tetromino {
    position: Point2D<u32>,
    t_tetromino: TetrominoType,
    color: Color,
    blocks: Vec<Block>,
}

impl Tetromino {
    pub fn new(position: &Point2D<u32>, t_tetromino: &TetrominoType, color: &Color) -> Tetromino {
        Tetromino {
            position: *position,
            t_tetromino: *t_tetromino,
            color: *color,
            blocks: match t_tetromino {
                I => Tetromino::tetromino_i(color),
                O => Tetromino::tetromino_o(color),
                T => Tetromino::tetromino_t(color),
                S => Tetromino::tetromino_s(color),
                Z => Tetromino::tetromino_z(color),
                J => Tetromino::tetromino_j(color),
                L => Tetromino::tetromino_l(color),
            },
        }
    }

    pub fn rotate(&mut self) {
        self.blocks.iter_mut().for_each(|p: &mut Block| {
            p.rotate(90.0f32);
        });
    }

    fn tetromino_i(color: &Color) -> Vec<Block> {
        vec![
            Block {
                position: Point2D::new(0, 0),
                color: *color,
            },
            Block {
                position: Point2D::new(0, 1),
                color: *color,
            },
            Block {
                position: Point2D::new(0, 2),
                color: *color,
            },
            Block {
                position: Point2D::new(0, 3),
                color: *color,
            },
        ]
    }

    fn tetromino_o(color: &Color) -> Vec<Block> {
        vec![
            Block {
                position: Point2D::new(0, 0),
                color: *color,
            },
            Block {
                position: Point2D::new(1, 0),
                color: *color,
            },
            Block {
                position: Point2D::new(0, 1),
                color: *color,
            },
            Block {
                position: Point2D::new(1, 1),
                color: *color,
            },
        ]
    }

    fn tetromino_t(color: &Color) -> Vec<Block> {
        vec![
            Block {
                position: Point2D::new(0, 0),
                color: *color,
            },
            Block {
                position: Point2D::new(-1, 0),
                color: *color,
            },
            Block {
                position: Point2D::new(1, 0),
                color: *color,
            },
            Block {
                position: Point2D::new(0, 1),
                color: *color,
            },
        ]
    }

    fn tetromino_s(color: &Color) -> Vec<Block> {
        vec![
            Block {
                position: Point2D::new(0, 0),
                color: *color,
            },
            Block {
                position: Point2D::new(-1, 0),
                color: *color,
            },
            Block {
                position: Point2D::new(0, 1),
                color: *color,
            },
            Block {
                position: Point2D::new(1, 1),
                color: *color,
            },
        ]
    }
    fn tetromino_z(color: &Color) -> Vec<Block> {
        vec![
            Block {
                position: Point2D::new(0, 0),
                color: *color,
            },
            Block {
                position: Point2D::new(1, 0),
                color: *color,
            },
            Block {
                position: Point2D::new(0, 1),
                color: *color,
            },
            Block {
                position: Point2D::new(-1, 1),
                color: *color,
            },
        ]
    }

    fn tetromino_j(color: &Color) -> Vec<Block> {
        vec![
            Block {
                position: Point2D::new(0, 0),
                color: *color,
            },
            Block {
                position: Point2D::new(0, 1),
                color: *color,
            },
            Block {
                position: Point2D::new(0, 2),
                color: *color,
            },
            Block {
                position: Point2D::new(-1, 0),
                color: *color,
            },
        ]
    }

    fn tetromino_l(color: &Color) -> Vec<Block> {
        vec![
            Block {
                position: Point2D::new(0, 0),
                color: *color,
            },
            Block {
                position: Point2D::new(0, 1),
                color: *color,
            },
            Block {
                position: Point2D::new(0, 2),
                color: *color,
            },
            Block {
                position: Point2D::new(1, 0),
                color: *color,
            },
        ]
    }
}
