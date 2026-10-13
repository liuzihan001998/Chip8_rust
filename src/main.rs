extern crate sdl2;

use sdl2::{
    event::Event,
    keyboard::Keycode,
    render::{Canvas, Texture},
    video::Window,
    EventPump, Sdl,
    pixels::PixelFormatEnum,
};

use rand::random;
use std::{io, fs, path::Path};
use std::time::{Duration, Instant};

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
    video : [u32; Chip::VIDEO_WIDTH * Chip::VIDEO_HEIGHT],
    opcode : u16,
    font_set : [u8; Chip::FONT_SET_SIZE],
}

impl Chip {

    // The constants of the memory
    const FONT_SET_SIZE : usize = 80;
    const FONT_START_ADDR : usize = 0x50;
    const START_ADDRESS : usize = 0x200;
    // The constants of the video
    const VIDEO_WIDTH : usize = 64;
    const VIDEO_HEIGHT : usize = 32;

    fn load_rom(&mut self, path: impl AsRef<Path>) -> io::Result<()> {
        let rom = fs::read(path)?;
        let start = Self::START_ADDRESS;

        if rom.is_empty() || rom.len() > self.memory.len() - start {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "ROM 为空或超过 3584 字节",
            ));
        }

        self.memory[start..start + rom.len()].copy_from_slice(&rom);

        Ok(())
    }

    fn new() -> Self {
        let mut chip = Self {
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
            font_set: [
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
            ]
        };
        chip.memory[Self::FONT_START_ADDR..Self::FONT_START_ADDR + Self::FONT_SET_SIZE]
            .copy_from_slice(&chip.font_set);
        chip
    }

    fn cycle(&mut self) -> Result<(), String> {
        let pc = usize::from(self.pc);
        let instruction = self.memory.get(pc..pc + 2)
            .ok_or_else(|| format!("Program counter is outside CHIP-8 memory: {:#05X}", self.pc))?;
        self.opcode = u16::from_be_bytes([instruction[0], instruction[1]]);
        self.pc = self.pc.wrapping_add(2);

        match self.opcode & 0xF000 {
            0x0000 => match self.opcode {
                0x00E0 => self.op_00e0(),
                0x00EE => {
                    if self.sp == 0 {
                        return Err("Return with an empty stack".into());
                    }
                    self.op_00ee();
                }
                _ => return Err(format!("Unknown opcode: {:#06X}", self.opcode)),
            },
            0x1000 => self.op_1nnn(),
            0x2000 => {
                if usize::from(self.sp) >= self.stack.len() {
                    return Err("CHIP-8 stack overflow".into());
                }
                self.op_2nnn();
            }
            0x3000 => self.op_3xnn(),
            0x4000 => self.op_4xnn(),
            0x5000 if self.opcode & 0x000F == 0 => self.op_5xy0(),
            0x6000 => self.op_6xnn(),
            0x7000 => self.op_7xnn(),
            0x8000 => match self.opcode & 0x000F {
                0x0 => self.op_8xy0(),
                0x1 => self.op_8xy1(),
                0x2 => self.op_8xy2(),
                0x3 => self.op_8xy3(),
                0x4 => self.op_8xy4(),
                0x5 => self.op_8xy5(),
                0x6 => self.op_8xy6(),
                0x7 => self.op_8xy7(),
                0xE => self.op_8xye(),
                _ => return Err(format!("Unknown opcode: {:#06X}", self.opcode)),
            },
            0x9000 if self.opcode & 0x000F == 0 => self.op_9xy0(),
            0xA000 => self.op_annn(),
            0xB000 => self.op_bnnn(),
            0xC000 => self.op_cxnn(),
            0xD000 => self.op_dxyn().map_err(str::to_owned)?,
            0xE000 => match self.opcode & 0x00FF {
                0x9E => self.op_ex9e(),
                0xA1 => self.op_exa1(),
                _ => return Err(format!("Unknown opcode: {:#06X}", self.opcode)),
            },
            0xF000 => match self.opcode & 0x00FF {
                0x07 => self.op_fx07(),
                0x0A => self.op_fx0a(),
                0x15 => self.op_fx15(),
                0x18 => self.op_fx18(),
                0x1E => self.op_fx1e(),
                0x29 => self.op_fx29(),
                0x33 => {
                    if usize::from(self.index) + 3 > self.memory.len() {
                        return Err("BCD is outside CHIP-8 memory".into());
                    }
                    self.op_fx33();
                }
                0x55 | 0x65 => {
                    let vx = ((self.opcode & 0x0F00) >> 8) as usize;
                    if usize::from(self.index) + vx + 1 > self.memory.len() {
                        return Err("Register transfer is outside CHIP-8 memory".into());
                    }
                    if self.opcode & 0x00FF == 0x55 {
                        self.op_fx55();
                    } else {
                        self.op_fx65();
                    }
                }
                _ => return Err(format!("Unknown opcode: {:#06X}", self.opcode)),
            },
            _ => return Err(format!("Unknown opcode: {:#06X}", self.opcode)),
        }

        Ok(())
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

    // Skip next instruction if register[Vx] != nn
    fn op_4xnn(&mut self) {
        let vk = ((self.opcode & 0x0F00) >> 8) as usize;
        let nn = (self.opcode & 0x00FF) as u8;
        if self.registers[vk] != nn{
            self.pc += 2;
        }
    }

    // Skip next instruction if register[Vx] = register[Vy]
    fn op_5xy0(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;

        if self.registers[vx] == self.registers[vy] {
            self.pc += 2;
        }
    }

    // Set register[Vx] to nn
    fn op_6xnn(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let nn = (self.opcode & 0x00FF) as u8;
        self.registers[vx] = nn;
    }

    // Add nn to register[Vx]
    fn op_7xnn(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let nn = (self.opcode & 0x00FF) as u8;
        self.registers[vx] = self.registers[vx].wrapping_add(nn);
    }
}

// 实现指令集 8
impl Chip {

    // Set register[Vx] to the value of register[Vy]
    fn op_8xy0(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;
        self.registers[vx] = self.registers[vy];
    }

    // Set register[Vx] to register[Vx] OR register[Vy]
    fn op_8xy1(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;
        self.registers[vx] |= self.registers[vy];
    }

    // Set register[Vx] to register[Vx] AND register[Vy]
    fn op_8xy2(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;
        self.registers[vx] &= self.registers[vy];
    }

    // Set register[Vx] to register[Vx] XOR register[Vy]
    fn op_8xy3(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;
        self.registers[vx] ^= self.registers[vy];
    }

    // Set register[Vx] to register[Vx] + register[Vy]
    // set VF = carry
    fn op_8xy4(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;
        let (sum, carry) = self.registers[vx].overflowing_add(self.registers[vy]);
        self.registers[vx] = sum;
        self.registers[0xF] = carry as u8;
    }

    // Set Vx to Vx - Vy; VF is 1 when no borrow occurs.
    fn op_8xy5(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;
        let (difference, borrow) = self.registers[vx].overflowing_sub(self.registers[vy]);
        self.registers[vx] = difference;
        self.registers[0xF] = (!borrow) as u8;
    }

    // Shift Vx right by one; VF gets the original least significant bit.
    fn op_8xy6(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let value = self.registers[vx];
        self.registers[vx] = value >> 1;
        self.registers[0xF] = value & 1;
    }

    // Set Vx to Vy - Vx; VF is 1 when no borrow occurs.
    fn op_8xy7(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;
        let (difference, borrow) = self.registers[vy].overflowing_sub(self.registers[vx]);
        self.registers[vx] = difference;
        self.registers[0xF] = (!borrow) as u8;
    }

    // Shift Vx left by one; VF gets the original most significant bit.
    fn op_8xye(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let value = self.registers[vx];
        self.registers[vx] = value << 1;
        self.registers[0xF] = value >> 7;
    }

}

// 实现指令集 9 A B C
impl Chip {

    // Skip the next instruction when Vx and Vy differ.
    fn op_9xy0(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;
        if self.registers[vx] != self.registers[vy] {
            self.pc += 2;
        }
    }

    // Set the index register to nnn.
    fn op_annn(&mut self) {
        self.index = self.opcode & 0x0FFF;
    }

    // Jump to nnn plus V0.
    fn op_bnnn(&mut self) {
        self.pc = (self.opcode & 0x0FFF) + u16::from(self.registers[0]);
    }

    // Store a random byte masked by nn in Vx.
    fn op_cxnn(&mut self) {
        self.op_cxnn_with_byte(random());
    }

    fn op_cxnn_with_byte(&mut self, random_byte: u8) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let nn = self.opcode as u8;
        self.registers[vx] = random_byte & nn;
    }
}

// 实现指令集 D
impl Chip {

    fn op_dxyn(&mut self) -> Result<(), &'static str> {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;
        let height = (self.opcode & 0x000F) as usize;

        let x_pos = usize::from(self.registers[vx]) % Self::VIDEO_WIDTH;
        let y_pos = usize::from(self.registers[vy]) % Self::VIDEO_HEIGHT;

        self.registers[0xF] = 0;

        for row in 0..height {
            if y_pos + row >= Self::VIDEO_HEIGHT {
                break;
            }

            let sprite_byte = *self.memory
                .get(usize::from(self.index) + row)
                .ok_or("Sprite is outside CHIP-8 memory")?;

            for col in 0..8 {
                if x_pos + col >= Self::VIDEO_WIDTH {
                    break;
                }

                if sprite_byte & (0x80u8 >> col) != 0 {
                    let screen_index = (y_pos + row) * Self::VIDEO_WIDTH + x_pos + col;
                    let screen_pixel = &mut self.video[screen_index];

                    if *screen_pixel == u32::MAX {
                        self.registers[0xF] = 1;
                    }

                    *screen_pixel ^= u32::MAX;
                }
            }
        }

        Ok(())
    }

}

// 实现指令集 E F
impl Chip {
    // Skip the next instruction if the key in Vx is pressed.
    fn op_ex9e(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let key = self.registers[vx] as usize;
        if self.keypad.get(key).is_some_and(|&pressed| pressed != 0) {
            self.pc = self.pc.wrapping_add(2);
        }
    }

    // Skip the next instruction if the key in Vx is not pressed.
    fn op_exa1(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let key = self.registers[vx] as usize;
        if !self.keypad.get(key).is_some_and(|&pressed| pressed != 0) {
            self.pc = self.pc.wrapping_add(2);
        }
    }

    fn op_fx07(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        self.registers[vx] = self.delay_timer;
    }

    // Repeat FX0A until a key is pressed; use the lowest pressed key.
    fn op_fx0a(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        if let Some(key) = self.keypad.iter().position(|&pressed| pressed != 0) {
            self.registers[vx] = key as u8;
        } else {
            self.pc = self.pc.wrapping_sub(2);
        }
    }

    // Set the delay timer to register[Vx]
    fn op_fx15(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        self.delay_timer = self.registers[vx];
    }

    // Set the sound timer to register[Vx]
    fn op_fx18(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        self.sound_timer = self.registers[vx];
    }

    // Set index register to index register + register[Vx]
    fn op_fx1e(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        self.index = self.index.wrapping_add(u16::from(self.registers[vx]));
    }

    // Set index register to the location of the sprite for the character in register[Vx]
    fn op_fx29(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        self.index = (Self::FONT_START_ADDR + usize::from(self.registers[vx]) * 5) as u16;
    }

    // Store the binary-coded decimal representation of register[Vx] at index register, index register + 1, and index register + 2
    fn op_fx33(&mut self) {
        let vx  = ((self.opcode & 0x0F00) >> 8) as usize;
        let mut number = self.registers[vx];
        let index = self.index as usize;
        self.memory[index + 2] = number % 10;
        number /= 10;
        self.memory[index + 1] = number % 10;
        number /= 10;
        self.memory[index] = number % 10;
    }

    // Store registers V0 through Vx in memory starting at index register
    fn op_fx55(&mut self) {

        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        for i in 0..=vx {
            self.memory[self.index as usize + i] = self.registers[i];
        }
    }

    // Read registers V0 through Vx from memory starting at index register
    fn op_fx65(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        for i in 0..=vx {
            self.registers[i] = self.memory[self.index as usize + i];
        }
    }
}

struct Platform{
    canvas : Canvas<Window>,
    events : EventPump,
    _sdl : Sdl,
}

impl Platform{

    fn key_index(key: Keycode) -> Option<usize> {
        match key {
            Keycode::X    => Some(0x0),
            Keycode::Num1 => Some(0x1),
            Keycode::Num2 => Some(0x2),
            Keycode::Num3 => Some(0x3),
            Keycode::Q    => Some(0x4),
            Keycode::W    => Some(0x5),
            Keycode::E    => Some(0x6),
            Keycode::A    => Some(0x7),
            Keycode::S    => Some(0x8),
            Keycode::D    => Some(0x9),
            Keycode::Z    => Some(0xA),
            Keycode::C    => Some(0xB),
            Keycode::Num4 => Some(0xC),
            Keycode::R    => Some(0xD),
            Keycode::F    => Some(0xE),
            Keycode::V    => Some(0xF),
            _ => None,
        }
    }

}

impl Platform{

    const WINDOW_WIDTH : u32 = 64;
    const WINDOW_HEIGHT : u32 = 32;

    // 初始化 SDL、创建窗口和 canvas
    fn new(scale: u32) -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;

        let window = video
            .window("Chip8", Self::WINDOW_WIDTH * scale, Self::WINDOW_HEIGHT * scale)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;

        let events = sdl.event_pump()?;

        Ok(Self {
            canvas,
            events,
            _sdl: sdl,
        })
    }

    fn process_input(&mut self, keys: &mut [u8; 16]) -> bool {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return true,

                Event::KeyDown {
                    keycode: Some(key),
                    ..
                } => {
                    if let Some(index) = Self::key_index(key) {
                        keys[index] = 1;
                    }
                }

                Event::KeyUp {
                    keycode: Some(key),
                    ..
                } => {
                    if let Some(index) = Self::key_index(key) {
                        keys[index] = 0;
                    }
                }

                _ => {}
            }
        }

        false
    }

    fn update(
        &mut self,
        texture: &mut Texture<'_>,
        screen: &[u32; Chip::VIDEO_WIDTH * Chip::VIDEO_HEIGHT],
    ) -> Result<(), String> {
        let pixels: Vec<u8> = screen
            .iter()
            .flat_map(|pixel| pixel.to_ne_bytes())
            .collect();

        texture
            .update(None, &pixels, Chip::VIDEO_WIDTH * size_of::<u32>())
            .map_err(|e| e.to_string())?;

        self.canvas.clear();
        self.canvas.copy(texture, None, None)?;
        self.canvas.present();

        Ok(())
    }

}


pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut chip = Chip::new();
    let rom_path = std::env::args().nth(1).unwrap_or_else(|| "test_opcode.ch8".to_owned());
    chip.load_rom(&rom_path)?;

    let mut platform = Platform::new(10)?;
    let texture_creator = platform.canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::RGBA8888,
            Platform::WINDOW_WIDTH,
            Platform::WINDOW_HEIGHT,
        )?;

    platform.update(&mut texture, &chip.video)?;

    let instruction_interval = Duration::from_millis(2);
    let timer_interval = Duration::from_secs_f64(1.0 / 60.0);
    let mut instruction_elapsed = Duration::ZERO;
    let mut timer_elapsed = Duration::ZERO;
    let mut last_time = Instant::now();

    loop {
        if platform.process_input(&mut chip.keypad) {
            break;
        }

        let now = Instant::now();
        let elapsed = now.duration_since(last_time);
        last_time = now;
        instruction_elapsed += elapsed;
        timer_elapsed += elapsed;

        let mut drew_frame = false;
        while instruction_elapsed >= instruction_interval {
            chip.cycle()?;
            instruction_elapsed -= instruction_interval;
            drew_frame = true;
        }

        while timer_elapsed >= timer_interval {
            chip.tick_timers();
            timer_elapsed -= timer_interval;
        }

        if drew_frame {
            platform.update(&mut texture, &chip.video)?;
        }

        std::thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}

#[cfg(test)]
mod tests;
