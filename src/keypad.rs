use sdl2::keyboard::Keycode;

// Represents a compatible keypad
pub struct Keypad {
    pub pad: [bool; 16],
}

impl Default for Keypad {
    fn default() -> Keypad {
        Keypad { pad: [false; 16] }
    }
}

impl Keypad {
    // new() returns a new empty keypad
    pub fn new() -> Keypad {
        Keypad { pad: [false; 16] }
    }

    // get_status fetches the current status from an index.
    // Returns true if index is pressed
    pub fn get_status(&mut self, index: usize) -> bool {
        self.pad[index]
    }

    // The Chip 8 system uses a simple HEX keypad that allows users to interact
    // with the system.
    pub fn map_key(&mut self, key: Keycode, state: bool) {
        // Mapping keys like this:
        // Keypad             Keyboard
        // +-+-+-+-+          +-+-+-+-+
        // |1|2|3|C|          |1|2|3|4|
        // |4|5|6|D|          |Q|W|E|R|
        // +-+-+-+-+    =>    +-+-+-+-+
        // |7|8|9|E|          |A|S|D|F|
        // |A|0|B|F|          |Z|X|C|V|
        // +-+-+-+-+          +-+-+-+-+
        match key {
            Keycode::Num1 => self.set_key(0x1, state),
            Keycode::Num2 => self.set_key(0x2, state),
            Keycode::Num3 => self.set_key(0x3, state),
            Keycode::Num4 => self.set_key(0xC, state),

            Keycode::Q => self.set_key(0x4, state),
            Keycode::W => self.set_key(0x5, state),
            Keycode::E => self.set_key(0x6, state),
            Keycode::R => self.set_key(0xD, state),

            Keycode::A => self.set_key(0x7, state),
            Keycode::S => self.set_key(0x8, state),
            Keycode::D => self.set_key(0x9, state),
            Keycode::F => self.set_key(0xE, state),

            Keycode::Z => self.set_key(0xA, state),
            Keycode::X => self.set_key(0x0, state),
            Keycode::C => self.set_key(0xB, state),
            Keycode::V => self.set_key(0xF, state),

            _ => println!("Unmapped key pressed: {}", key.name()),
        }
    }

    // Sets key
    pub fn set_key(&mut self, index: usize, state: bool) {
        self.pad[index] = state;
    }
}
