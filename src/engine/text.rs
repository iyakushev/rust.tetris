use std::path::Path;


struct Text {
    ctx: sdl2::ttf::Sdl2TtfContext,
    content: String,
    pos_x: u32,
    pos_y: u32,
    color: sdl2::pixels::Color,
    font: sdl2::ttf::Font,
    size: u16
}


impl Text {
    pub fn new(content: &str, x: u32, y: u32) -> Result<Self, String> {
        let ctx = sdl2::ttf::init().map_err(|e| e.to_string())?;
        let font = ctx.load_font(Path::new("data/Bruzh.ttf"), 10)?;
        Ok(Text {
            ctx: ctx,
            content: content.to_string(),
            pos_x: x,
            pos_y: y,
            font: font,
            color: sdl2::pixels::Color::RGB(0, 0, 0),
            size: 10
        })
    }

    pub fn set_font(&mut self, font: &str) -> Result<(), String> {
        self.font = self.ctx.load_font(Path::new(font), self.size)?;
        Ok(())
    }

    pub fn apply_style(&mut self, style: sdl2::ttf::FontStyle) {
        self.font.set_style(style);
    }

    pub fn mut_size(&mut self, size: u16) {
        self.size = size;
    }


    pub fn get_surface(&self) {
    }

    pub fn place_text(&mut self, padding: u32) -> Result<(), String> {        
            // render a surface, and convert it to a texture bound to the canvas
            let surface = font.render(text)
                .solid(clr).map_err(|e| e.to_string())?;
            let texture_creator = self.canvas.texture_creator();
            let texture = texture_creator.create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;
    
            self.canvas.set_draw_color(Color::RGBA(195, 217, 255, 255));
            self.canvas.clear();
    
            let TextureQuery { width, height, .. } = texture.query();
    
            // If the example text is too big for the screen, downscale it (and center irregardless)
            let target = self.get_centered_rect(width, height, self.width - padding, self.height - padding);
    
            self.canvas.copy(&texture, None, Some(target))?;
            self.canvas.present();
            Ok(())
        }
}
