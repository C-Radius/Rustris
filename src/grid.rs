use crate::util::*;
use crate::util::{Block, Point2D};
use ggez::graphics::Color;

pub struct Grid {
    width: u32,
    height: u32,
    blocks: Vec<Vec<Block>>,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Grid {
        Grid {
            width: width,
            height: height,
            blocks: (0..width)
                .into_iter()
                .map(|x: u32| {
                    (0..height)
                        .into_iter()
                        .map(|y: u32| Block {
                            position: Point2D::new(x as i32, y as i32),
                            color: Color::new(0.0, 0.0, 0.0, 1.0),
                        })
                        .collect::<Vec<Block>>()
                })
                .collect::<Vec<Vec<Block>>>(),
        }
    }
}
