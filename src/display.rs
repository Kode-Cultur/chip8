
pub struct Display {
    gfx: [[u8; 64]; 32],
    draw_flag: bool,
}

impl Display {
    // new returns a new display instance
    pub fn new() -> Display {
        Display {
            gfx: [[0; 64]; 32],
            draw_flag: true,
        }
    }

    // Clear clears the display
    pub fn clear(&mut self) {
        self.gfx = [[0; 64]; 32];
        self.draw_flag = true;
    }
}
