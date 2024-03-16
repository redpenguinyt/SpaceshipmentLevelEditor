use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::WindowCanvas,
    ttf::Sdl2TtfContext,
};

pub struct FontHandler {
    ttf: Sdl2TtfContext,
    pub current_font: String,
}

impl FontHandler {
    pub fn new(font: &str) -> Result<Self, String> {
        Ok(Self {
            ttf: sdl2::ttf::init().map_err(|e| e.to_string())?,
            current_font: String::from(font),
        })
    }

    pub fn draw_text(
        &mut self,
        canvas: &mut WindowCanvas,
        text: &str,
        pos: Point,
        font_size: u16,
    ) -> Result<(), String> {
        let font = self.ttf.load_font(&self.current_font, font_size)?;

        let text = font
            .render(text)
            .solid(Color::WHITE)
            .expect("Failed to render");

        let texture_creator = canvas.texture_creator();

        let text_texture = text
            .as_texture(&texture_creator)
            .map_err(|e| e.to_string())?;

        canvas.copy(
            &text_texture,
            Rect::new(0, 0, text.width(), text.height()),
            Rect::new(pos.x, pos.y, text.width(), text.height()),
        )?;

        Ok(())
    }
}
