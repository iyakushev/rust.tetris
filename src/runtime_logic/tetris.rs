use std::env::var;
use std::path::Path;

use rand::{distributions::{Distribution, Standard}, Rng};
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    timer::Timer,
};

use crate::engine::{render, text::Text};

//Todo scale interface
//Todo background animation
//Todo ghosting/shadow
//Todo rotation
//Todo APM
//Todo Timer

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Some(Rect::new($x as i32, $y as i32, $w as u32, $h as u32))
    )
);

#[derive(Debug)]
enum Shape { I, T, L, J, S, Z, O}
enum Rotation {Right, Left}

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


#[derive(Debug)]
struct Tetromino {
    shape: Shape,
    pos_x: u32, // Coordinates with respect to the tile size: 0 -> t_size
    pos_y: u32, // Coordinates with respect to the tile size: 0 -> t_size
    t_size: u8,
    m_shape: [u8; 4],
    r_angle: usize,
    color_offset: u8,
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
            t_size: 18
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

    pub fn make_move(&mut self, direction: i8) {
        let new_pos = (direction as i32 + self.pos_x as i32) as u32;
        if new_pos < 4 || new_pos > 12 { // Hardcoded borders
            ()
        } else {
            self.pos_x = new_pos
        }
    }

    pub fn draw(&self, window: &mut render::Window) {
        for offset in self.m_shape.iter() {
            let x = self.pos_x + (offset%4*1) as u32;
            let y = self.pos_y + (offset/4*1) as u32;
            window.load_texture(Path::new("data/art/tiles.png"),
                            rect!(self.color_offset, 0, self.t_size, self.t_size),
                            rect!(x * self.t_size as u32, y * self.t_size as u32, self.t_size, self.t_size)).unwrap();
        }
    }
}


pub fn run(window: &mut render::Window, event_pump: &mut sdl2::EventPump) -> Result<(), String> {
    let mut tetromino = Tetromino::new(rand::random());
    let mut next_tetr = Tetromino::new(rand::random());

    // let mut pocket_tm = None;

    let mut level = 1;
    let mut score = 0;
    let mut timer = 0;
    let mut field:Vec<Tetromino> = vec![];

    let ui_bottom_offset = (window.height - 54) as i32;
    const WHITE: Color = Color::RGBA(255, 255,255,255);
    const UI_H: i32 = 54;
    let border_left: u32 = 18*4-3;
    let border_right: u32 = window.width - 18*4+4;
    let ui = vec!(Text::new("Score:", 10, 10, 15, Some(WHITE)),
                  Text::new("000000", 55, 11, 15, Some(WHITE)),
                  Text::new("Level:", 14, 30, 15, Some(WHITE)),
                  Text::new(&level.to_string(), 55, 31 , 15, Some(WHITE)),
                  Text::new("NEXT:", 165, 10, 15, Some(WHITE)),
                  Text::new("APM:", 15, ui_bottom_offset as u32 + 10, 15, Some(WHITE)),
                  Text::new("000", 55, ui_bottom_offset as u32 + 11, 15, Some(WHITE)),
                  Text::new("Lines:", 10, ui_bottom_offset as u32 + 30, 15, Some(WHITE)),
                  Text::new("000", 55, ui_bottom_offset as u32 + 31, 15, Some(WHITE)),
                  Text::new("Time:", 128, ui_bottom_offset as u32 + 10, 15, Some(WHITE)),
                  Text::new("000", 130, ui_bottom_offset as u32 + 31, 15, Some(WHITE)),
                  Text::new("Pocket:", 190, ui_bottom_offset as u32 + 10, 15, Some(WHITE)));
    next_tetr.pos_x = 12;
    next_tetr.pos_y = 0;

    'running: loop {
        //Todo miami mode
        window.draw_bg(Color::RGBA(0, 0, 0, 255));
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::R), ..} => {
                    tetromino = Tetromino::new(rand::random());
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    tetromino.make_move(-1);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    tetromino.make_move(1);
                },
                Event::KeyDown { keycode: Some(Keycode::E), ..} => {
                    tetromino.rotate(Rotation::Right);
                },
                Event::KeyDown { keycode: Some(Keycode::Q), ..} => {
                    tetromino.rotate(Rotation::Left);
                }
                _ => {}
            }
        }

        window.draw_line(WHITE, (0,UI_H), (window.width as i32, UI_H));
        window.draw_line(WHITE, (145,0), (145, UI_H));
        window.draw_line(WHITE, (border_left as i32, ui_bottom_offset), (border_left as i32, UI_H));
        window.draw_line(WHITE, (border_right as i32,ui_bottom_offset), (border_right as i32, UI_H));
        window.draw_line(WHITE, (120,ui_bottom_offset), (120, window.height as i32));
        window.draw_line(WHITE, (170,ui_bottom_offset), (170, window.height as i32));
        window.draw_line(WHITE, (0, ui_bottom_offset), (window.width as i32, ui_bottom_offset));
        window.draw_text(&ui, 0)?;
        tetromino.draw(window);
        next_tetr.draw(window);
        
        window.present();
        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60)); // 1 tick
        if timer >= 60 {
            tetromino.pos_y += 1;
            timer = 0;
        } else {timer += 1}
    }

    Ok(())
}