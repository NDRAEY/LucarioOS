use crate::{Canvas, log::log};

pub struct TTY<'a> {
    pub canvas: &'a Canvas,
    pub x: usize,
    pub y: usize,
    pub color: usize
}

impl TTY<'_> {
    pub fn new(canvas: &Canvas) -> TTY<'_> {
        TTY {
            canvas,
            x: 0,
            y: 0,
            color: 0xffffff,
        }
    }

    pub fn putchar(&mut self, character: u8) {
        if character == '\n' as u8 {
            self.y += crate::display::font::FONT_HEIGHT as usize;
            self.x = 0;
            return;
        }

        if character == '\r' as u8 {
            self.x = 0;
            return;
        }

        if self.x >= self.canvas.width {
            self.y += crate::display::font::FONT_HEIGHT as usize;
            self.x = 0;
        }

        let pos: usize = character as usize * crate::display::font::FONT_HEIGHT as usize;
        let font = crate::display::font::FONT;

        for y in 0..crate::display::font::FONT_HEIGHT {
            let line = font[pos + y as usize];

            for x in 0..crate::display::font::FONT_WIDTH {
                if (line >> x) & 1 == 1 {
                    self.canvas.pixel(self.x + x as usize, self.y + y as usize, self.color as u32);
                }
            }
        }

        self.x += crate::display::font::FONT_WIDTH as usize;
    }

    pub fn puts(&mut self, strng: &str) {
        for i in strng.as_bytes() {
            self.putchar(*i);
        }
    }
}
