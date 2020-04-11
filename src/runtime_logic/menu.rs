use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::engine::{render, text::Text};

use super::tetris;

//Todo scale interface
    // |-> Todo settings .ini
//Todo sounds
//Todo background animation
//Todo name
//Todo highscore save

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub fn save_name(text: &str, pos_x: u32, pos_y: u32) {
    // render::rect!();
}


pub fn run() -> Result<(), String>{
    let mut window = render::Window::new(320, 480);
    let mut menu_text = vec!(
                            Text::new("Rust.Tetris!", 10, 20, 100, None),
                            Text::new("New game", 70, 150, 40, None),
                            Text::new("High score", 70, 200, 40, None),
                            Text::new("Settings", 70, 250, 40, None),
                            Text::new("Quit", 70, 300, 40, None));
    window.draw_bg(Color::RGBA(195, 217, 255, 255));
    window.draw_text(&menu_text, 70)?;
    window.present();

    let mut event_pump = window.create_event_pump();
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
                Event::KeyDown { keycode: Some(Keycode::Space), ..} => {
                    match cursor {
                        1 => break 'running,
                        4 => break 'running,
                        _ => (),
                    }
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        menu_text[cursor].set_color(Color::RGBA(255, 255, 255, 255));
        window.draw_text(&menu_text, 100)?;
        window.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    match cursor {
        1 => tetris::run(&mut window, &mut event_pump)?,
        _ => (),
    }

    Ok(())
}

