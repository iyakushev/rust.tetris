use crate::engine::{render, text::Text};
use sdl2::{
    pixels::Color,
    event::Event,
    keyboard::Keycode,
    rect::Rect
};
use rand::{distributions::{Distribution, Standard}, Rng};
use std::path::Path;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Some(Rect::new($x as i32, $y as i32, $w as u32, $h as u32))
    )
);


#[derive(Debug)]
enum Shape {
    I,
    T, 
    L, 
    J, 
    S, 
    Z, 
    O 
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

}

impl Tetromino {
    pub fn new(shape: Shape) -> Self {
        Tetromino {
            shape: shape,
            pos_x: 0,
            pos_y: 0
        }
    }

    pub fn id(&self) -> &Shape {
        &self.shape
    }

    pub fn draw(&self, window: &mut render::Window) {
        window.load_texture(Path::new("data/art/tiles.png"), None, rect!(100,50,140,18)).unwrap();
    }
}

pub fn run(window: &mut render::Window, event_pump: &mut sdl2::EventPump) -> Result<(), String> {
    let tetromino = Tetromino::new(rand::random());
    const WHITE: Color = Color::RGBA(255, 255,255,255);
    let ui = vec!(Text::new("Score:", 10, 10, 20, Some(WHITE)));
    
    'running: loop {
        window.draw_bg(Color::RGBA(0, 0, 0, 255));
        window.draw_line(Color::RGBA(255,255,255,255), (0,50), (window.width as i32, 50));
        window.draw_text(&ui, 10)?;
        tetromino.draw(window);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        
        window.present();
    }
    println!("PIECE: {:?}", tetromino.id());
    
    Ok(())
}