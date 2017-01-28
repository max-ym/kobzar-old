use super::LoggerTrait;

pub struct Logger {
    /// Index of a cell being updated.
    index   : i16
}

impl Logger {

    pub fn new() -> Self {
        Logger { index: 0 }
    }

    /// Shift all symbols up by one line.
    fn shift(&mut self) {
        // Point to second row on the screen.
        let mut i = 0xB8000 + 80 * 2;
        while i < 0xB8000 + 80 * 2 * 25 {
            let cell_src = (i + 80 * 2) as *mut i16;
            let cell_dst = (i         ) as *mut i16;

            unsafe { *cell_dst = *cell_src; }

            i = i + 2;
        }
    }

    fn set(&self, c: char) {
        let cell = (self.index as isize * 2 + 0xB8000) as *mut i16;
        unsafe { *cell = 0x0700 | (c as i16); }
    }
}

impl LoggerTrait for Logger {

    fn newline(&mut self) {
        // Get index of a cell in the new line.
        let i = self.index + 80;
        if i >= 80 * 25 {
            // Shift all symbols up.
            self.shift();
            self.index = self.index % 80;
        } else {
            self.index = i - i % 80;
        }
    }

    fn print(&mut self, s: &str) {
        let chars = s.chars();
        for ch in chars {
            if ch == '\n' {
                self.newline();
            } else {
                self.set(ch);

                self.index += 1;
                if self.index >= 80 * 25 {
                    self.shift();
                    self.index -= 80;
                }
            }
        }
    }
}
