pub struct Canvas {
    pub buffer: *mut u8,
    pub width: usize,
    pub height: usize,
    pub pitch: usize,
    pub bpp: usize,
}

impl Canvas {
    #[inline]
    #[allow(arithmetic_overflow)]
    pub fn pixel(&self, x: usize, y: usize, color: u32) {
        if x >= self.width && y >= self.height {
            return;
        }

        unsafe {
            let pixpos = self.pixel_pos(x, y) as isize;

            *self.buffer.offset(pixpos).as_mut().unwrap_unchecked() = color as u8;
            *self.buffer.offset(pixpos + 1).as_mut().unwrap_unchecked() = (color >> 8) as u8;
            *self.buffer.offset(pixpos + 2).as_mut().unwrap_unchecked() = (color >> 16) as u8;
        }
    }

    #[inline]
    fn pixel_pos(&self, x: usize, y: usize) -> usize {
        x * (self.bpp / 8) + y * self.pitch
    }
}
