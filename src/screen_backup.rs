const VGA_ADDRESS: usize = 0xb8000;

pub struct Screen {
    cursor_x: usize,
    cursor_y: usize,
    width: usize,
    height: usize,
    dimension: usize,
}

impl Screen {
    pub fn new(
        cursor_x: usize,
        cursor_y: usize,
        width: usize,
        height: usize,
        dimension: usize,
    ) -> Self {
        Screen {
            cursor_x,
            cursor_y,
            width,
            height,
            dimension,
        }
    }

    fn new_line(&mut self) {
        self.cursor_x = 0;
        self.cursor_y += 1;
    }

    fn carriage_return(&mut self) {
        self.cursor_x = 0;
    }

    pub fn print_char(&mut self, c: char) {
        let mut video_memory: &mut [u8] =
            unsafe { core::slice::from_raw_parts_mut(VGA_ADDRESS as *mut u8, self.dimension) };
        match c {
            '\n' => self.new_line(),
            '\r' => self.carriage_return(),
            _ => {
                if self.cursor_x >= self.width {
                    self.new_line();
                }
                let index = self.cursor_y * self.width + self.cursor_x;
                unsafe {
                    video_memory[index] = c as u8;
                }
                self.cursor_x += 1;
            }
        }
    }
}
