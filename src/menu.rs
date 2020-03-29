use sdl2::pixels::Color;
use crate::engine::{render, text::Text};

//Todo scale and positioning
//Todo settings .ini
//Todo cursor/selector indicator
//Todo sounds


pub fn run() -> Result<(), String>{
    let mut window = render::Window::new(800, 600);
    let menu_text = vec!(Text::new("Tetris", 250, 50, 100)?, 
                         Text::new("New game", 300, 150, 40)?,
                         Text::new("High score", 300, 200, 40)?,
                         Text::new("Settings", 300, 250, 40)?,
                         Text::new("Quit", 300, 300, 40)?);
    window.draw_bg(Color::RGBA(195, 217, 255, 255));
    window.draw_text(menu_text, 100)?;
    window.run();
    Ok(())
}

