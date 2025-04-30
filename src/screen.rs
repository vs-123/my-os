const VGA_ADDRESS: usize = 0xb8000;

#[derive(Clone)]
pub struct Screen {
    cursor_x: usize,
    cursor_y: usize,
    width: usize,
    height: usize,
    dimension: usize,
}

impl Screen {
    pub fn new(cursor_x: usize, cursor_y: usize, width: usize, height: usize) -> Self {
        Screen {
            cursor_x,
            cursor_y,
            width,
            height,
            dimension: width * height,
        }
    }

    pub fn print_char(&mut self, c: char) {
        let mut video_memory: &mut [u8] =
            unsafe { core::slice::from_raw_parts_mut(VGA_ADDRESS as *mut u8, self.dimension) };
        match c {
            '\x08' => {
                self.cursor_x -= 2;
                video_memory[self.cursor_y * self.width + self.cursor_x] = ' ' as u8;
            }

            '\x09' => {
                self.cursor_x = (self.cursor_x + 8) & !(8 - 1);
            }

            '\n' => {
                self.cursor_x = 0;
                self.cursor_y += 2;
            }

            '\r' => {
                self.cursor_x = 0;
            }

            _ => {
                if self.cursor_x >= self.width {
                    self.print_char('\n');
                }

                unsafe {
                    // CHARACTER //
                    video_memory[self.cursor_y * self.width + self.cursor_x] = c as u8;
                    // COLOR //
                    self.cursor_x += 1;
                    // 0x0f is the color of white on black, 0 is black and f is white
                    video_memory[self.cursor_y * self.width + self.cursor_x] = 0x0f;
                }
                self.cursor_x += 1;
            }
        }
        if self.cursor_x >= self.width {
            self.cursor_x = 0;
            self.cursor_y += 1;
        }
    }

    pub fn print(&mut self, s: &str) {
        for c in s.chars() {
            self.print_char(c);
        }
    }
}
