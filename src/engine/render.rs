extern crate sdl2; 

use sdl2::{
    pixels::Color,
    event::Event,
    keyboard::Keycode,
    render::TextureQuery,
    rect::Rect 
};
use std::time::Duration;
use std::path::Path;


// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub struct Window {
    canvas: sdl2::render::WindowCanvas,
    width: u32,
    height: u32,
    ctx: sdl2::Sdl
}

impl Window {
    pub fn new(width: u32, height: u32) -> Self {
        let ctx = sdl2::init().unwrap();

        let video_subsystem = ctx.video().unwrap();
     
        let window = video_subsystem.window("Tetris", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        
        Window {
            canvas: window.into_canvas().build().unwrap(),
            width: width,
            height: height,
            ctx: ctx
        }
    }
    
    // Scale fonts to a reasonable size when they're too big (though they might look less smooth)
    fn get_centered_rect(&self, rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
        let wr = rect_width as f32 / cons_width as f32;
        let hr = rect_height as f32 / cons_height as f32;

        let (w, h) = if wr > 1f32 || hr > 1f32 {
            if wr > hr {
                println!("Scaling down! The text will look worse!");
                let h = (rect_height as f32 / wr) as i32;
                (cons_width as i32, h)
            } else {
                println!("Scaling down! The text will look worse!");
                let w = (rect_width as f32 / hr) as i32;
                (w, cons_height as i32)
            }
        } else {
            (rect_width as i32, rect_height as i32)
        };

        let cx = (self.width as i32 - w) / 2;
        let cy = (self.height as i32 - h) / 2;
        rect!(cx, cy, w, h)
    }

    pub fn set_color(&mut self, clr: Color) {
        self.canvas.set_draw_color(clr);
    }

    pub fn run(&mut self) {     
        // self.canvas.set_draw_color(Color::RGB(0, 255, 255));
        // self.canvas.clear();
        // self.canvas.present();
        let mut event_pump = self.ctx.event_pump().unwrap();
        // let mut i = 0;
        'running: loop {
            // i = (i + 1) % 255;
            // self.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            // self.canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }
            // The rest of the game loop goes here...
    
            // self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}

