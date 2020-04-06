use std::path::Path;
use std::process::exit;

use rand::{distributions::{Distribution, Standard}, Rng};
use sdl2::rect::Rect;

use crate::engine::render::Window;

//Todo scale interface
//Todo background animation
//Todo ghosting/shadow
//Todo APM

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Some(Rect::new($x as i32, $y as i32, $w as u32, $h as u32))
    )
);

#[derive(Debug, Copy, Clone)]
pub enum Shape { I, T, L, J, S, Z, O}
pub enum Rotation {Right, Left}

impl Shape {
    ///Returns a corresponding matrix shape of a figure
    pub fn matrix(&self) -> [u8; 4] {
        match self {
            Shape::I => [4,5,6,7],    // 0,  1,  2,  3
            Shape::J => [0,4,5,6],    // 4,  5,  6,  7
            Shape::L => [3,5,6,7],    // 8,  9, 10, 11
            Shape::O => [2,3,6,7],    //12, 13, 14, 15
            Shape::S => [2,3,5,6],
            Shape::Z => [0,1,5,6],
            Shape::T => [1,4,5,6],
        }
    }

    ///Returns a corresponding shift in texture
    pub fn texture_offset(&self) -> u8 {
        match self {
            Shape::I => 5*18,
            Shape::J => 0,
            Shape::L => 6*18,
            Shape::O => 4*18,
            Shape::S => 3*18,
            Shape::Z => 2*18,
            Shape::T => 18,
        }
    }
}

impl Distribution<Shape> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Shape {
        match rng.gen_range(0, 6) {
            0 => Shape::I,
            1 => Shape::T,
            2 => Shape::L,
            3 => Shape::J,
            4 => Shape::S,
            5 => Shape::Z,
            _ => Shape::O
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Tetromino {
    shape: Shape,
    pos_x: u32, // Coordinates with respect to the tile size: 0 -> t_size
    pos_y: u32, // Coordinates with respect to the tile size: 0 -> t_size
    t_size: u8,
    m_shape: [u8; 4],
    r_angle: usize,
    color_offset: u8,
    active: bool
}

impl Tetromino {
    pub fn new(shape: Shape) -> Self {
        Tetromino {
            r_angle: 0,
            m_shape: shape.matrix(),
            color_offset: shape.texture_offset(),
            shape: shape,
            pos_x: 10,
            pos_y: 0,
            t_size: 18,
            active: true
        }
    }

    pub fn id(&self) -> &Shape {
        &self.shape
    }

    // does fixed angle rotation to acknowledge T-spin/wall kicks, save on performance
    pub fn rotate(&mut self, r: Rotation) {
        let variations: [[u8;4];4] = match self.shape {
            Shape::I => [[4,5,6,7], [2,6,10,14], [8,9,10,11], [1,5,9,13]],  // 0,  1,  2,  3
            Shape::J => [[0,4,5,6], [1,2,5,9], [4,5,6,10], [1,5,8,9]],      // 4,  5,  6,  7
            Shape::L => [[3,5,6,7], [2,6,10,11], [5,6,7,9], [1,2,6,10]],    // 8,  9, 10, 11
            Shape::O => [[2,3,6,7], [2,3,6,7], [2,3,6,7], [2,3,6,7]],       //12, 13, 14, 15
            Shape::S => [[2,3,5,6], [2,6,7,11], [6,7,9,10], [1,5,6,10]],
            Shape::Z => [[0,1,5,6], [2,5,6,9], [4,5,9,10], [1,4,5,8]],
            Shape::T => [[1,4,5,6], [1,5,6,9], [4,5,6,9], [1,4,5,9]],
        };
        match r {
            Rotation::Right => {
                if self.r_angle < 3 {self.r_angle += 1}
                else {self.r_angle = 0}
            },
            Rotation::Left => {
                if self.r_angle > 0 {self.r_angle -= 1}
                else {self.r_angle = 3}
            }
        }
        self.m_shape = variations[self.r_angle];
    }

    pub fn set_default_pos(&mut self) {
        self.pos_x = 9;
        self.pos_y = 0;
        self.r_angle = 0;
        self.m_shape = self.shape.matrix();
    }

    pub fn set_for_next(&mut self) {
        self.pos_x = 12;
        self.pos_y = 0;
        self.r_angle = 0;
        self.m_shape = self.shape.matrix();
    }

    pub fn set_to_pocket(&mut self) {
        self.pos_x = 14;
        self.pos_y = 24;
        self.r_angle = 0;
        self.m_shape = self.shape.matrix();
    }

    pub fn stops_falling(&self) -> bool { !self.active }

    pub fn make_move(&mut self, direction: i8, axis: u8) {
        let new_pos = (direction as i32 + self.pos_x as i32) as u32;
        if new_pos < 4 || new_pos > 12 { // Hardcoded borders
            ()
        } else {
            match axis {
                0 => self.pos_x = new_pos,
                1 => {
                    if self.pos_y < 20 {
                        self.pos_y += 1
                    } else {
                        self.active = false
                    }
                },
                _ => exit(12),
            }
        }
    }

    pub fn draw(&self, window: &mut Window) -> Result<(), String> {
        for offset in self.m_shape.iter() {
            let x = self.pos_x + (offset%4*1) as u32;
            let y = self.pos_y + (offset/4*1) as u32;
            window.load_texture(Path::new("data/art/tiles.png"),
                                rect!(self.color_offset, 0, self.t_size, self.t_size),
                                rect!(x * self.t_size as u32, y * self.t_size as u32, self.t_size, self.t_size))?;
        }
        Ok(())
    }
}