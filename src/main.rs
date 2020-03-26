mod tetris;
mod engine;
mod menu;

use engine::render;
use sdl2::pixels::Color;

fn main() -> Result<(), String> {
    println!("Hello, world!");
    let mut window = render::Window::new(800, 600);
    // window.place_text("Ris", "resources/AmazDooMRight2.ttf", 200, Color::RGB(255,0,0), 20)?;
    window.run();

    Ok(())
}
