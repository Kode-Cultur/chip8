use rand::random;
use sdl2::Sdl;
use std::fs::File;
use std::io::prelude::*;

use display::Display;
use keypad::Keypad;

/// Chip8 fontset
/// Each bit that is 1 represents a pixel turned on, (ie. white), and each bit
/// that is 0 represents a pixel that is off.
static CHIP8_FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

// This stuct describes a chip 8 vm
pub struct Chip8 {
    // Holds current opcode
    opcode: u16,
    // Index
    pub i: u16,
    // Program counter
    pub pc: u16,
    // Memory, size 4K
    pub mem: [u8; 4096],
    // Stack, 16-levels
    pub stack: [u16; 16],
    // Stack pointer
    pub sp: u8,
    // V register
    pub v: [u8; 16],

    // Delay timer
    pub dt: u8,

    pub keypad: Keypad,
    pub display: Display,

    pub draw_flag: bool,
}

impl Chip8 {
    pub fn new(sdl: &Sdl) -> Chip8 {
        let mut chip = Chip8 {
            opcode: 0,
            i: 0,
            pc: 0x200,
            mem: [0; 4096],
            stack: [0; 16],
            sp: 0,
            v: [0; 16],
            dt: 0,
            keypad: Keypad::new(),
            display: Display::new(sdl),
            draw_flag: true,
        };
        // Load fontset into memory
        for (i, v) in CHIP8_FONTSET.iter().enumerate().take(80) {
            chip.mem[i] = *v;
        }
        chip
    }

    pub fn load_application(&mut self, filepath: &str) {
        let mut file = File::open(&filepath).expect("Failed to open file");
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf).expect("Failed to fill buffer");

        for (i, &v) in buf.iter().enumerate() {
            let addr = 0x200 + i;
            if addr < 4096 {
                self.mem[addr] = v;
            } else {
                break;
            }
        }
    }

    // Emulates a execution cycle
    pub fn emulate_cycle(&mut self) {
        let opcode = self.read_opcode();
        self.opcode = opcode;
        self.process_opcode(opcode);

        if self.dt > 0 {
            self.dt -= 1;
        };
    }

    // Fetches and reads current opcode from indexed memory.
    fn read_opcode(&self) -> u16 {
        (u16::from(self.mem[self.pc as usize])) << 8 | (u16::from(self.mem[(self.pc + 1) as usize]))
    }

    // Processes opcode
    fn process_opcode(&mut self, opcode: u16) {
        // Increment the program counter
        self.pc += 2;

        // Decodig opcodes into nibbles
        let op_0 = (opcode & 0xF000) >> 12;
        let op_1 = (opcode & 0x0F00) >> 8;
        let op_2 = (opcode & 0x00F0) >> 4;
        let op_3 = opcode & 0x000F;

        self.opcode = opcode;

        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let y = ((self.opcode & 0x00F0) >> 4) as usize;
        let n = (self.opcode & 0x000F) as u8;
        let nn = (self.opcode & 0x00FF) as u8;
        let nnn = self.opcode & 0x0FFF;

        // matching on those opcode-nibbles
        match (op_0, op_1, op_2, op_3) {
            // 0x00E0: Clear display
            (0, 0, 0xE, 0) => {
                self.display.clear();
                self.draw_flag = true;
            }
            // 00EE: Returns from a subroutine
            (0, 0, 0xE, 0xE) => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }
            // 1NNN: Jumps to address nnn
            (0x1, _, _, _) => self.pc = nnn,
            // 2NNN: Calls subroutine at nnn
            (0x2, _, _, _) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }
            // Skips next instruction if ...
            // 3XNN: ...vx == nn
            (0x3, _, _, _) => {
                self.pc += if self.v[x] == nn { 2 } else { 0 };
            }
            // 4XNN: ...vx != nn
            (0x4, _, _, _) => {
                self.pc += if self.v[x] != nn { 2 } else { 0 };
            }
            // 5XY0: .. vx == vy
            (0x5, _, _, _) => {
                self.pc = if self.v[x] == self.v[y] { 2 } else { 0 };
            }
            // Sets vx to...
            // 6XNN: ...nn
            (0x6, _, _, _) => self.v[x] = nn,
            // 7XNN: ...vx += nn
            (0x7, _, _, _) => self.v[x] += nn,
            // 8XY0: ...vy
            (0x8, _, _, 0x0) => self.v[x] = self.v[y],
            // 8XY1: ...vx OR vy
            (0x8, _, _, 0x1) => self.v[x] |= self.v[y],
            // 8XY2: ...vx AND vy
            (0x8, _, _, 0x2) => self.v[x] &= self.v[y],
            // 8XY3: ...vx XOR vy
            (0x8, _, _, 0x3) => self.v[x] ^= self.v[y],
            // 8XY4: ...vx + vy
            (0x8, _, _, 0x4) => {
                let result = u16::from(self.v[x]) + u16::from(self.v[y]);
                // if the result is bigger than 8 bit (255) the carry flag is
                // set to 1 otherwise 0.
                self.v[0xF] = if result > 0xFF { 1 } else { 0 };
                // The lowest 8 bits of the result are kept and stored.
                self.v[x] >>= 1;
            }
            // 8XY5: ...vx-vy
            (0x8, _, _, 0x5) => {
                let result = self.v[x] as i8 - self.v[y] as i8;
                self.v[x] = result as u8;
                self.v[0xF] = if result < 0 { 1 } else { 0 };
            }
            // 8XY6: ...
            // Sets VF to 1 if the least significant bit of vx is 1 otherwise 0.
            // Then vx is divided by 2.
            (0x8, _, _, 0x6) => {
                self.v[0xF] = self.v[x] & 0x1;
                self.v[x] >>= 1;
            }
            // 8XY7: ...vy-vx
            (0x8, _, _, 0x7) => {
                let result = self.v[y] as i8 - self.v[x] as i8;
                self.v[x] = result as u8;
                self.v[0xF] = if result < 0 { 1 } else { 0 };
            }
            // 8XYE: ...
            // Sets VF to 1 if the most significant bit of vx is 1 otherwise 0.
            // Then vx is multiplied by 2.
            (0x8, _, _, 0xE) => {
                self.v[0xF] = self.v[x] & 0x80;
                self.v[x] <<= 1;
            }
            // 9XY0: Skips next instruction if vx != vf.
            (0x9, _, _, _) => self.pc += if self.v[x] != self.v[0xF] { 2 } else { 0 },
            // ANNN: The value of register i is set to nnn.
            (0xA, _, _, _) => self.i = nnn,
            // BNNN: Jumps to location nnn + v0.
            (0xB, _, _, _) => self.pc = nnn + u16::from(self.v[0]),
            // CXNN: Sets vx to the rsult of a random bitwise operation on a
            // random number and NN.
            (0xC, _, _, _) => self.v[x] = random::<u8>() & nn,
            // DXNN: Display n-byte sprite starting at memory location I at
            // (VX, VY), set VF = collision.
            (0xD, _, _, _) => {
                self.v[0xF] = 0;
                for byte in 0..n {
                    let y = (self.v[y] as usize + byte as usize) % 32;
                    for bit in 0..8 {
                        let x = (self.v[x] as usize + bit as usize) % 64;
                        let color = (self.mem[self.i as usize + byte as usize] >> (7 - bit)) & 1;
                        self.v[0xF] |= color & self.display.gfx[y][x];
                        self.display.gfx[y][x] ^= color;
                    }
                }
                self.display.draw_on_screen();
            }
            // EX9E: Skip next instruction if key with the value of vx is
            // pressed.
            (0xE, _, 0x9, 0xE) => {
                self.pc += if self.keypad.get_status(self.v[x] as usize) {
                    2
                } else {
                    0
                };
            }
            // EXA1: Skip next instruction if key with the value of vx is not
            // pressed.
            (0xE, _, 0xA, 0x1) => {
                self.pc += if self.keypad.get_status(self.v[x] as usize) {
                    0
                } else {
                    2
                };
            }
            // FX07: Set vx to the value of the delay timer
            (0xF, _, 0x0, 0x7) => self.v[x] = self.dt,
            // FX0A: Wait for a key press, then store the value to vx
            (0xF, _, 0x0, 0xA) => {
                // self.pc -= 2;
                for (i, v) in self.keypad.pad.iter().enumerate() {
                    if *v {
                        // If key is pressed
                        self.v[x] = i as u8;
                        self.pc += 2;
                        break;
                    }
                }
            }
            // FX15: Set delay timer to vx
            (0xF, _, 0x1, 0x5) => self.dt = self.v[x],
            // FX18: Set sound timer to vx
            (0xF, _, 0x1, 0x8) => { /* Not implemented */ }
            // FX1E: Set i to i + vx
            (0xF, _, 0x1, 0xE) => self.i += u16::from(self.v[x]),
            // FX29: Set i to the location of sprite for digit vx
            (0xF, _, 0x2, 0x9) => self.i = u16::from(self.v[x]),
            // FX33: Store the bcd representation of vx in memory locations
            // i, i+1 and i+2.
            (0xF, _, 0x3, 0x3) => {
                self.mem[self.i as usize] = self.v[x] / 100;
                self.mem[self.i as usize + 1] = (self.v[x] / 10) % 10;
                self.mem[self.i as usize + 2] = (self.v[x] % 100) % 10;
            }
            // FX55: Store registers v0 through vx in memory starting at
            // location I
            (0xF, _, 0x5, 0x5) => {
                for i in 0..x {
                    self.mem[(self.i + i as u16) as usize] = self.v[i];
                }
                self.i += (x + 1) as u16;
            }
            // FX65: Read registers v0 through vx from memory starting at
            // location I
            (0xF, _, 0x6, 0x5) => {
                for i in 0..x {
                    self.v[i] = self.mem[(self.i + i as u16) as usize];
                }
                self.i += (x + 1) as u16;
            }
            _ => println!("Unkown opcode: {:?}", opcode),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate sdl2;
    use super::*;

    #[test]
    fn opcode_jump() {
        let sdl_context = sdl2::init().unwrap();
        let mut chip = Chip8::new(&sdl_context);
        chip.process_opcode(0x1A2A);
        assert_eq!(chip.pc, 0x0A2A);
    }
}
