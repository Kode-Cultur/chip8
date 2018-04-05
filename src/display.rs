use sdl2::Sdl;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

// Scaling the old screen a little bit
static SCALE_FACTOR: u32 = 10;

pub struct Display {
    pub gfx: [[u8; 64]; 32],
    canvas: WindowCanvas,
    draw_flag: bool,
}

impl Display {
    // new returns a new display instance
    pub fn new(sdl: &Sdl) -> Display {
        let video_sub = sdl.video().unwrap();
        let window = video_sub
            .window("chipr8", 64 * SCALE_FACTOR, 32 * SCALE_FACTOR)
            .position_centered()
            .build()
            .unwrap();

        // let mut canvas = window.into_canvas().build().unwrap();
        let mut canvas: WindowCanvas = window.into_canvas().present_vsync().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Display {
            gfx: [[0; 64]; 32],
            canvas: canvas,
            draw_flag: true,
        }
    }

    // Clear clears the display
    pub fn clear(&mut self) {
        self.gfx = [[0; 64]; 32];
        self.draw_on_screen();
        self.draw_flag = true;
    }

    // Draws current sprite on display
    pub fn draw_on_screen(&mut self) {
        for (y, line) in self.gfx.iter().enumerate() {
            for (x, &index) in line.iter().enumerate() {
                let x = (x as u32) * SCALE_FACTOR;
                let y = (y as u32) * SCALE_FACTOR;

                self.canvas.set_draw_color(set_color(index));
                self.canvas
                    .fill_rect(Rect::new(x as i32, y as i32, SCALE_FACTOR, SCALE_FACTOR))
                    .unwrap();
            }
        }
        self.canvas.present();
        self.draw_flag = true;
        // self.canvas.clear();
    }
}

// Returns color
fn set_color(pixel: u8) -> pixels::Color {
    if pixel == 0 {
        pixels::Color::RGB(0, 0, 0)
    } else {
        pixels::Color::RGB(0, 255, 255)
    }
}
