#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate rand;
extern crate sdl2;

use sdl2::event::Event;
use std::thread;
use std::time::Duration;

use docopt::Docopt;

mod cpu;
mod display;
mod keypad;

use cpu::Chip8;

// Command line arguments
const USAGE: &'static str = "
    chip8 - A Chip-8 emulator.

    Usage:
        chipr8 load <game>
        chipr8 ( -h | --help)
        chipr8 --version

    Options:
        -h --help    Show this screen.
        --version    Show version information text.
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_load: bool,
    arg_game: String,
}

fn main() {
    let sleep_dur = Duration::from_millis(2);

    // Parsing the command line args
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    // Initializing sdl2
    let sdl_context = sdl2::init().unwrap();

    let mut chip = Chip8::new(&sdl_context);

    if args.cmd_load {
        chip.load_application(&args.arg_game);
    }
    let mut event_pump = sdl_context.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main, // Exits main loop
                Event::KeyDown { keycode: key, .. } => chip.keypad.map_key(key.unwrap(), true),
                Event::KeyUp { keycode: key, .. } => chip.keypad.map_key(key.unwrap(), false),
                _ => {}
            }
        }
        chip.emulate_cycle();
        chip.display.draw_on_screen();
        thread::sleep(sleep_dur);
    }
}
