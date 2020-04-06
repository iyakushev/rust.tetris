use std::borrow::Borrow;
use std::path::Path;

pub struct Text {
    content: String,
    pub pos_x: u32,
    pub pos_y: u32,
    hint: sdl2::ttf::Hinting,
    color: sdl2::pixels::Color,
    font: String,
    size: u16
}

/// Returns a value that was cast to a string filled with leading zeros to match capacity.
pub fn cast_with_capacity(value: u16, capacity: usize) -> String {
    let mut string = value.to_string();
    if string.len() < capacity {
        string.insert_str(0, &"0".repeat(capacity - string.len()));
    }
    string
}


impl Text {
    pub fn new(content: &str, x: u32, y: u32, size: u16, color: Option<sdl2::pixels::Color>) -> Self {
        let clr = match color {
            Some(color) => color,
            _ => sdl2::pixels::Color::RGBA(0,0,0,255),
        };
        Text {
            content: content.to_string(),
            pos_x: x,
            pos_y: y,
            hint: sdl2::ttf::Hinting::Normal,
            font: "data/fonts/sansation.ttf".to_string(),
            color: clr,
            size: size
        }
    }

    pub fn change_text(&mut self, new_text: &str) {
        self.content = new_text.to_string();
    }

    pub fn contents(&self) -> &str {
        &self.content
    }

    pub fn get_color(&self) -> sdl2::pixels::Color {
        self.color
    }

    pub fn set_hinting(&mut self, hint: sdl2::ttf::Hinting) { self.hint = hint }

    pub fn set_font(&mut self, font: &str) {
        self.font = format!("data/fonts/{}",font.to_string());
    }

    pub fn set_size(&mut self, size: u16) {
        self.size = size;
    }

    pub fn set_color(&mut self, clr: sdl2::pixels::Color) {
        self.color = clr;
    }

    pub fn get_surface(&self, styles: Vec<sdl2::ttf::FontStyle>) -> Result<sdl2::surface::Surface, String> {
        let ctx = sdl2::ttf::init().map_err(|e| e.to_string())?;
        let mut font = ctx.load_font(Path::new(&self.font), self.size)?;
        for el in styles {
            font.set_style(el);
        }
        font.set_hinting(self.hint.clone());
        let surface  = font.render(&self.content)
            .blended(self.color)
            .map_err(|e| e.to_string())?;
        Ok(surface)
    }
}
