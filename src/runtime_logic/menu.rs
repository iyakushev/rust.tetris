use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::rect::Rect;
use std::time::Duration;
use crate::engine::{render, text::Text};
use super::tetris;

//Todo scale and positioning
//Todo settings .ini
//Todo sounds
//Todo background animation
//Todo text animation

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub fn pop_in_box(text: &str, pos_x: u32, pos_y: u32) {
    // render::rect!();
}


pub fn run() -> Result<(), String>{
    let mut window = render::Window::new(800, 600);
    let mut menu_text = vec!(Text::new("Tetris", 260, 20, 100)?, 
                         Text::new("New game", 300, 150, 40)?,
                         Text::new("High score", 300, 200, 40)?,
                         Text::new("Settings", 300, 250, 40)?,
                         Text::new("Quit", 300, 300, 40)?);
    window.draw_bg(Color::RGBA(195, 217, 255, 255));
    window.text_to_buf(&menu_text, 100)?;
    window.present();

    let mut event_pump = window.get_event_pump();
    let mut cursor = 1;
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        window.draw_bg(Color::RGB(i, 64, 255 - i));
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                    if cursor > 1 {
                        menu_text[cursor].set_color(Color::RGBA(0, 0, 0, 255));
                        cursor -= 1;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    if cursor < menu_text.len() - 1 {
                        menu_text[cursor].set_color(Color::RGBA(0, 0, 0, 255));
                        cursor += 1;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::KpEnter), ..} => {
                    match cursor {
                        1 => tetris::run(),
                        _ => (),
                    }
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        menu_text[cursor].set_color(Color::RGBA(255, 255, 255, 255));
        window.text_to_buf(&menu_text, 100)?;
        window.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }


    Ok(())
}

