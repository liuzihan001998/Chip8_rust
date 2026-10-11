extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use rand::random;
use std::{io, fs, path::Path};

struct Chip {
    registers: [u8; 16],
    memory: [u8; 4096],
    index : u16,
    pc : u16,
    stack: [u16; 16],
    sp: u8,
    sound_timer: u8,
    delay_timer: u8,
    keypad: [u8; 16],
    video : [u32; 64 * 32],
    opcode : u16,
}

impl Chip {

    // The constants of the memory
    const FONT_SET_SIZE : u32 = 80;
    const FONT_START_ADDR : u32 = 0x50;
    const START_ADDRESS : u32 = 0x200;

    fn load_rom(&mut self, path: impl AsRef<Path>) -> io::Result<()> {
        let rom = fs::read(path)?;
        let start : usize = Self::FONT_START_ADDR as usize;

        if rom.is_empty() || rom.len() > self.memory.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "ROM 为空或超过 3584 字节",
            ));
        }

        self.memory[start..start + rom.len()].copy_from_slice(&rom);

        Ok(())
    }

    fn new() -> Self {
        Self {
            registers: [0; 16],
            memory: [0; 4096],
            index: 0,
            pc: Self::START_ADDRESS as u16,
            stack: [0; 16],
            sp: 0,
            sound_timer: 0,
            delay_timer: 0,
            keypad: [0; 16],
            video: [0; 64 * 32],
            opcode: 0,
        }
    }

    fn cycle(&mut self) {
        let pc : usize = self.pc as usize;
        self.opcode = u16::from_be_bytes(
            [self.memory[pc],
            self.memory[pc+1]]
        );

        self.pc += 2;

        // TODO: decode and execute opcode

    }

    fn tick_timers(&mut self) {
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
    }
}


// 实现指令集 0
impl Chip {

    // clear the display
    fn op_00e0(&mut self) {
        self.video.fill(0);
    }

    // Return from a subroutine
    fn op_00ee(&mut self) {
        self.sp -=1;
        self.pc = self.stack[self.sp as usize];
    }

}

// 实现指令集 1~7
impl Chip {

    // Jump to location nnn
    fn op_1nnn(&mut self) {
        let nnn = self.opcode & 0x0FFF;
        self.pc = nnn;
    }

    // Call subroutine at nnn
    fn op_2nnn(&mut self) {
        let nnn = self.opcode & 0x0FFF;
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = nnn;
    }

    // Skip next instruction if register[Vx] = nn
    fn op_3xnn(&mut self) {
        let vk = ((self.opcode & 0x0F00) >> 8) as usize;
        let nn = (self.opcode & 0x00FF) as u8;
        if self.registers[vk] == nn{
            self.pc += 2;
        }
    }

}



struct PlatformLayer{

}

impl PlatformLayer{

    const WINDOW_WIDTH : u32 = 64;
    const WINDOW_HEIGHT : u32 = 32;

}


pub fn main() {

    let mut chip = Chip::new();

    match chip.load_rom("test_opcode.ch8") {
        Ok(_) => {}
        Err(e) => {
            println!("Error loading test opcode.ch8: {}", e);
        }
    }

}

#[cfg(test)]
mod tests;
