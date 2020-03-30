mod engine;
mod runtime_logic;

use runtime_logic::menu;
use engine::{render,text::Text};
use sdl2::pixels::Color;

fn main() -> Result<(), String> {
    menu::run();
    Ok(())
}