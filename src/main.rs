use sdl2::pixels::Color;

use runtime_logic::menu;

mod engine;
mod runtime_logic;

fn main() -> Result<(), String> {
    menu::run();
    Ok(())
}