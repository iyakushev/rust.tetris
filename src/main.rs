mod tetris;
mod engine;
mod menu;

use engine::{render,text::Text};
use sdl2::pixels::Color;

fn main() -> Result<(), String> {
    menu::run();
    Ok(())
}
