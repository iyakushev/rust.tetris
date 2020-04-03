use crate::engine::{render, text::Text};
use sdl2::{
    pixels::Color,
    event::Event,
    keyboard::Keycode,
    rect::Rect
};
use rand::{distributions::{Distribution, Standard}, Rng};
use std::path::Path;

//Todo scale interface
//Todo background animation
//Todo ghosting/shadow
//Todo rotation

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Some(Rect::new($x as i32, $y as i32, $w as u32, $h as u32))
    )
);

#[derive(Debug)]
enum Shape { I, T, L, J, S, Z, O}

impl Shape {
    ///Returns a corresponding matrix shape of a figure
    pub fn matrix(&self) -> [u8; 4] {
        match self {
            Shape::I => [1,3,5,7],
            Shape::J => [3,5,7,6],
            Shape::L => [2,3,5,7],
            Shape::O => [2,3,4,5],
            Shape::S => [3,5,4,6],
            Shape::Z => [2,4,5,7],
            Shape::T => [3,4,5,7],
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
    pos_x: u32,
    pos_y: u32,
    t_size: u8,
    m_shape: [u8; 4],
    color_offset: u8,
}

impl Tetromino {
    pub fn new(shape: Shape) -> Self {
        Tetromino {
            m_shape: shape.matrix(),
            color_offset: shape.texture_offset(),
            shape: shape,
            pos_x: 160,
            pos_y: 100,
            t_size: 18
        }
    }

    pub fn id(&self) -> &Shape {
        &self.shape
    }

    pub fn rotate(&mut self, r: u8) {
        // match angle {
        //     0 => ,
        //     1 => ,
        //     2 => ,
        //     3 => ,
        // }
    }

    pub fn make_move(&mut self, direction: i8, lborder: u32, rborder: u32) {
        if self.pos_x > lborder && self.pos_x < rborder {
            self.pos_x = ((direction * self.t_size as i8) as i32 + self.pos_x as i32) as u32;
        }
    }

    pub fn draw(&self, window: &mut render::Window) {
        for offset in self.m_shape.iter() {
            let x = self.pos_x + ((offset%2) * self.t_size) as u32;
            let y = self.pos_y+ ((offset/2)*self.t_size) as u32;
            window.load_texture(Path::new("data/art/tiles.png"),
                            rect!(self.color_offset, 0, self.t_size, self.t_size),
                            rect!(x, y, self.t_size, self.t_size)).unwrap();
        }
    }
}

pub fn run(window: &mut render::Window, event_pump: &mut sdl2::EventPump) -> Result<(), String> {
    let mut tetromino = Tetromino::new(rand::random());
    let ui_bottom_offset = (window.height - 54) as i32;
    const WHITE: Color = Color::RGBA(255, 255,255,255);
    let border_left: u32 = 18*4;
    let border_right: u32 = window.width - 18*4;
    let ui = vec!(Text::new("Score:", 10, 10, 15, Some(WHITE)),
                  Text::new("000000", 55, 11, 15, Some(WHITE)),
                  Text::new("Level:", 14, 30, 15, Some(WHITE)),
                  Text::new("01", 55, 31 , 15, Some(WHITE)),
                  Text::new("NEXT:", 165, 10, 15, Some(WHITE)),
                  Text::new("APM:", 15, ui_bottom_offset as u32 + 10, 15, Some(WHITE)),
                  Text::new("000", 55, ui_bottom_offset as u32 + 11, 15, Some(WHITE)),
                  Text::new("Lines:", 10, ui_bottom_offset as u32 + 30, 15, Some(WHITE)),
                  Text::new("000", 55, ui_bottom_offset as u32 + 31, 15, Some(WHITE)),
                  Text::new("Time:", 128, ui_bottom_offset as u32 + 10, 15, Some(WHITE)),
                  Text::new("000", 130, ui_bottom_offset as u32 + 31, 15, Some(WHITE)),
                  Text::new("Pocket:", 190, ui_bottom_offset as u32 + 10, 15, Some(WHITE)));

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
                }
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    tetromino.make_move(-1, border_left, border_right);
                }
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    tetromino.make_move(1, border_left, border_right);
                }
                _ => {}
            }
        }

        window.draw_line(WHITE, (0,54), (window.width as i32, 54));
        window.draw_line(WHITE, (145,0), (145, 54));
        window.draw_line(WHITE, (18*4-3,ui_bottom_offset), (18*4-3, 54));
        window.draw_line(WHITE, ((window.width - 18*4+2) as i32,ui_bottom_offset), ((window.width - 18*4+2) as i32, 54));
        window.draw_line(WHITE, (120,ui_bottom_offset), (120, window.height as i32));
        window.draw_line(WHITE, (170,ui_bottom_offset), (170, window.height as i32));
        window.draw_text(&ui, 0)?;
        window.draw_line(WHITE, (0, ui_bottom_offset), (window.width as i32, ui_bottom_offset));
        tetromino.draw(window);
        
        window.present();
    }

    Ok(())
}