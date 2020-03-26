mod tetris;
mod render;
mod menu;

fn main() -> Result<(), String> {
    println!("Hello, world!");
    let mut window = render::Window::new(800, 600);
    window.place_text("SUPER MARIO")?;
    window.run();

    Ok(())
}
