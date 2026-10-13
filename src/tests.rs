use super::*;

#[test]
fn op_2nnn_pushes_return_address_and_jumps() {
    let mut chip = Chip::new();
    chip.opcode = 0x2A3B;
    chip.pc = 0x202; // cycle() has already advanced past the CALL instruction.

    chip.op_2nnn();

    assert_eq!(chip.stack[0], 0x202);
    assert_eq!(chip.sp, 1);
    assert_eq!(chip.pc, 0x0A3B);
}

#[test]
fn op_3xnn_skips_when_register_equals_byte() {
    let mut chip = Chip::new();
    chip.opcode = 0x3255;
    chip.pc = 0x202;
    chip.registers[2] = 0x55;
    chip.op_3xnn();

    assert_eq!(chip.pc, 0x204);
}

#[test]
fn op_3xnn_does_not_skip_when_register_differs() {
    let mut chip = Chip::new();
    chip.opcode = 0x3255;
    chip.pc = 0x202;
    chip.registers[2] = 0x54;
    chip.op_3xnn();

    assert_eq!(chip.pc, 0x202);
}

#[test]
fn op_4nnn_pushes_return_address_and_jumps() {
    let mut chip = Chip::new();
    chip.opcode = 0x4255;
    chip.registers[2] = 0x55;
    chip.pc = 0x202;

    chip.op_4xnn();

    assert_eq!(chip.pc, 0x202);
}

#[test]
fn op_4xnn_skips_when_register_equals_byte() {
    let mut chip = Chip::new();
    chip.opcode = 0x4255;
    chip.registers[2] = 0x56;
    chip.pc = 0x202;

    chip.op_4xnn();

    assert_eq!(chip.pc, 0x204);
}

#[test]
fn op_5xy0_test_0() {
    let mut chip = Chip::new();
    chip.opcode = 0x5010;
    chip.registers[0] = 0x56;
    chip.registers[1] = 0x56;
    chip.pc = 0x202;

    chip.op_5xy0();

    assert_eq!(chip.pc, 0x204);
}

#[test]
fn op_5xy0_test_1() {
    let mut chip = Chip::new();
    chip.opcode = 0x5010;
    chip.registers[0] = 0x56;
    chip.registers[1] = 0x57;
    chip.pc = 0x202;

    chip.op_5xy0();

    assert_eq!(chip.pc, 0x202);
}

#[test]
fn op_6xnn_test_0() {
    let mut chip = Chip::new();
    chip.opcode = 0x6010;
    chip.registers[0] = 0x10;

    chip.op_6xnn();

    assert_eq!(chip.registers[0], 0x10);
}

#[test]
fn op_7xnn_test_0() {
    let mut chip = Chip::new();
    chip.opcode = 0x7033;
    chip.registers[0] = 0x10;

    chip.op_7xnn();

    assert_eq!(chip.registers[0], 0x43);
}

#[test]
fn op_8xy0_test_1() {
    let mut chip = Chip::new();
    chip.opcode = 0x8120;
    chip.registers[1] = 0x10;
    chip.registers[2] = 0x30;

    chip.op_8xy0();

    assert_eq!(chip.registers[1], 0x30);
}

#[test]
fn op_8xy1_test_0() {
    let mut chip = Chip::new();
    chip.opcode = 0x8121;
    chip.registers[1] = 0x13;
    chip.registers[2] = 0x56;

    chip.op_8xy1();

    assert_eq!(chip.registers[1], 0x57);
}

#[test]
fn op_8xy2_test_0() {
    let mut chip = Chip::new();
    chip.opcode = 0x8122;
    chip.registers[1] = 0x13;
    chip.registers[2] = 0x56;

    chip.op_8xy2();

    assert_eq!(chip.registers[1], 0x12);
}

#[test]
fn op_8xy3_test_1() {
    let mut chip = Chip::new();
    chip.opcode = 0x8123;
    chip.registers[1] = 0x13;
    chip.registers[2] = 0x56;

    chip.op_8xy3();

    assert_eq!(chip.registers[1], 0x45);
}

#[test]
fn op_8xy4_test_1() {
    let mut chip = Chip::new();
    chip.opcode = 0x8124;
    chip.registers[1] = 0xff;
    chip.registers[2] = 0xee;

    chip.op_8xy4();

    assert_eq!(chip.registers[1], 0xed);
    assert_eq!(chip.registers[15], 0x01);
}

#[test]
fn op_8xy4_test_2() {
    let mut chip = Chip::new();
    chip.opcode = 0x8124;
    chip.registers[1] = 0x10;
    chip.registers[2] = 0x20;
    chip.registers[0xF] = 1;

    chip.op_8xy4();

    assert_eq!(chip.registers[1], 0x30);
    assert_eq!(chip.registers[0xF], 0);
}

#[test]
fn op_8xy4_test_3() {
    let mut chip = Chip::new();
    chip.opcode = 0x8F24;
    chip.registers[0xF] = 0xFF;
    chip.registers[2] = 1;

    chip.op_8xy4();

    assert_eq!(chip.registers[0xF], 1);
}

#[test]
fn op_8xy5_test_0() {
    let mut chip = Chip::new();
    chip.opcode = 0x8125;
    chip.registers[1] = 0x50;
    chip.registers[2] = 0x20;

    chip.op_8xy5();

    assert_eq!(chip.registers[1], 0x30);
    assert_eq!(chip.registers[0xF], 1);
}

#[test]
fn op_8xy5_test_1() {
    let mut chip = Chip::new();
    chip.opcode = 0x8125;
    chip.registers[1] = 0x10;
    chip.registers[2] = 0x20;
    chip.registers[0xF] = 1;

    chip.op_8xy5();

    assert_eq!(chip.registers[1], 0xF0);
    assert_eq!(chip.registers[0xF], 0);
}

#[test]
fn op_8xy5_test_2() {
    let mut chip = Chip::new();
    chip.opcode = 0x8125;
    chip.registers[1] = 0x20;
    chip.registers[2] = 0x20;

    chip.op_8xy5();

    assert_eq!(chip.registers[1], 0);
    assert_eq!(chip.registers[0xF], 1);
}

#[test]
fn op_8xy6_test_0() {
    let mut chip = Chip::new();
    chip.opcode = 0x8126;
    chip.registers[1] = 0x03;
    chip.registers[2] = 0xFF;

    chip.op_8xy6();

    assert_eq!(chip.registers[1], 0x01);
    assert_eq!(chip.registers[0xF], 1);
}

#[test]
fn op_8xy6_test_1() {
    let mut chip = Chip::new();
    chip.opcode = 0x8126;
    chip.registers[1] = 0x80;
    chip.registers[0xF] = 1;

    chip.op_8xy6();

    assert_eq!(chip.registers[1], 0x40);
    assert_eq!(chip.registers[0xF], 0);
}

#[test]
fn op_8xy7_test_0() {
    let mut chip = Chip::new();
    chip.opcode = 0x8127;
    chip.registers[1] = 0x20;
    chip.registers[2] = 0x50;

    chip.op_8xy7();

    assert_eq!(chip.registers[1], 0x30);
    assert_eq!(chip.registers[0xF], 1);
}

#[test]
fn op_8xy7_test_1() {
    let mut chip = Chip::new();
    chip.opcode = 0x8127;
    chip.registers[1] = 0x20;
    chip.registers[2] = 0x10;
    chip.registers[0xF] = 1;

    chip.op_8xy7();

    assert_eq!(chip.registers[1], 0xF0);
    assert_eq!(chip.registers[0xF], 0);
}

#[test]
fn op_8xy7_test_2() {
    let mut chip = Chip::new();
    chip.opcode = 0x8127;
    chip.registers[1] = 0x20;
    chip.registers[2] = 0x20;

    chip.op_8xy7();

    assert_eq!(chip.registers[1], 0);
    assert_eq!(chip.registers[0xF], 1);
}

#[test]
fn op_8xye_test_0() {
    let mut chip = Chip::new();
    chip.opcode = 0x812E;
    chip.registers[1] = 0x81;
    chip.registers[2] = 0x00;

    chip.op_8xye();

    assert_eq!(chip.registers[1], 0x02);
    assert_eq!(chip.registers[0xF], 1);
}

#[test]
fn op_8xye_test_1() {
    let mut chip = Chip::new();
    chip.opcode = 0x812E;
    chip.registers[1] = 0x40;
    chip.registers[0xF] = 1;

    chip.op_8xye();

    assert_eq!(chip.registers[1], 0x80);
    assert_eq!(chip.registers[0xF], 0);
}

#[test]
fn op_9xy0_test_0() {
    let mut chip = Chip::new();
    chip.opcode = 0x9120;
    chip.pc = 0x202;
    chip.registers[1] = 0x50;
    chip.registers[2] = 0x20;

    chip.op_9xy0();

    assert_eq!(chip.pc, 0x204);
}

#[test]
fn op_9xy0_test_1() {
    let mut chip = Chip::new();
    chip.opcode = 0x9120;
    chip.pc = 0x202;
    chip.registers[1] = 0x50;
    chip.registers[2] = 0x50;

    chip.op_9xy0();

    assert_eq!(chip.pc, 0x202);
}

#[test]
fn op_annn_test_0() {
    let mut chip = Chip::new();
    chip.opcode = 0xAABC;
    chip.index = 0x123;

    chip.op_annn();

    assert_eq!(chip.index, 0x0ABC);
}

#[test]
fn op_bnnn_test_0() {
    let mut chip = Chip::new();
    chip.opcode = 0xB200;
    chip.pc = 0x202;
    chip.registers[0] = 0x15;

    chip.op_bnnn();

    assert_eq!(chip.pc, 0x215);
}

#[test]
fn op_bnnn_test_1() {
    let mut chip = Chip::new();
    chip.opcode = 0xB345;
    chip.pc = 0x202;

    chip.op_bnnn();

    assert_eq!(chip.pc, 0x345);
}

#[test]
fn op_cxnn_test_0() {
    let mut chip = Chip::new();
    chip.opcode = 0xC3F0;

    chip.op_cxnn_with_byte(0xAB);

    assert_eq!(chip.registers[3], 0xA0);
}

#[test]
fn op_cxnn_test_1() {
    let mut chip = Chip::new();
    chip.opcode = 0xC300;
    chip.registers[3] = 0xFF;

    chip.op_cxnn();

    assert_eq!(chip.registers[3], 0);
}

#[test]
fn op_dxyn_test_0() {
    let mut chip = Chip::new();
    chip.opcode = 0xD122;
    chip.registers[1] = 3;
    chip.registers[2] = 4;
    chip.index = 0x300;
    chip.memory[0x300] = 0x81;
    chip.memory[0x301] = 0x40;

    chip.op_dxyn().unwrap();

    assert_eq!(chip.video[4 * Chip::VIDEO_WIDTH + 3], u32::MAX);
    assert_eq!(chip.video[4 * Chip::VIDEO_WIDTH + 10], u32::MAX);
    assert_eq!(chip.video[5 * Chip::VIDEO_WIDTH + 4], u32::MAX);
    assert_eq!(chip.video[4 * Chip::VIDEO_WIDTH + 4], 0);
    assert_eq!(chip.video.iter().filter(|&&pixel| pixel != 0).count(), 3);
    assert_eq!(chip.registers[0xF], 0);
}

#[test]
fn op_dxyn_test_1() {
    let mut chip = Chip::new();
    chip.opcode = 0xD121;
    chip.index = 0x300;
    chip.memory[0x300] = 0x80;

    chip.op_dxyn().unwrap();
    assert_eq!(chip.video[0], u32::MAX);
    assert_eq!(chip.registers[0xF], 0);

    chip.op_dxyn().unwrap();
    assert_eq!(chip.video[0], 0);
    assert_eq!(chip.registers[0xF], 1);

    chip.op_dxyn().unwrap();
    assert_eq!(chip.video[0], u32::MAX);
    assert_eq!(chip.registers[0xF], 0);
}

#[test]
fn op_dxyn_test_2() {
    let mut chip = Chip::new();
    chip.opcode = 0xD121;
    chip.registers[1] = 65;
    chip.registers[2] = 33;
    chip.index = 0x300;
    chip.memory[0x300] = 0x80;

    chip.op_dxyn().unwrap();

    assert_eq!(chip.video[Chip::VIDEO_WIDTH + 1], u32::MAX);
    assert_eq!(chip.video.iter().filter(|&&pixel| pixel != 0).count(), 1);
}

#[test]
fn op_dxyn_test_3() {
    let mut chip = Chip::new();
    chip.opcode = 0xD122;
    chip.registers[1] = 63;
    chip.registers[2] = 31;
    chip.index = 0x300;
    chip.memory[0x300] = 0xFF;
    chip.memory[0x301] = 0xFF;

    chip.op_dxyn().unwrap();

    assert_eq!(chip.video[Chip::VIDEO_WIDTH * Chip::VIDEO_HEIGHT - 1], u32::MAX);
    assert_eq!(chip.video.iter().filter(|&&pixel| pixel != 0).count(), 1);
    assert_eq!(chip.registers[0xF], 0);
}

#[test]
fn op_dxyn_test_4() {
    let mut chip = Chip::new();
    chip.opcode = 0xD121;
    chip.index = 4096;

    assert_eq!(chip.op_dxyn(), Err("Sprite is outside CHIP-8 memory"));
    assert!(chip.video.iter().all(|&pixel| pixel == 0));
}

#[test]
fn op_dxyn_test_5() {
    let mut chip = Chip::new();
    chip.opcode = 0xD120;
    chip.index = 4096;
    chip.registers[0xF] = 1;
    chip.video[0] = u32::MAX;

    chip.op_dxyn().unwrap();

    assert_eq!(chip.video[0], u32::MAX);
    assert_eq!(chip.registers[0xF], 0);
}

#[test]
fn op_dxyn_test_6() {
    let mut chip = Chip::new();
    chip.opcode = 0xDF11;
    chip.registers[0xF] = 2;
    chip.index = 0x300;
    chip.memory[0x300] = 0x80;
    chip.video[2] = u32::MAX;

    chip.op_dxyn().unwrap();

    assert_eq!(chip.video[2], 0);
    assert_eq!(chip.registers[0xF], 1);
}

#[test]
fn op_ex9e_test_0() {
    let mut chip = Chip::new();
    chip.opcode = 0xE39E;
    chip.pc = 0x202;
    chip.registers[3] = 0xA;
    chip.keypad[0xA] = 1;
    chip.op_ex9e();
    assert_eq!(chip.pc, 0x204);
}

#[test]
fn op_ex9e_test_1() {
    let mut chip = Chip::new();
    chip.opcode = 0xE39E;
    chip.pc = 0x202;
    chip.registers[3] = 0xA;
    chip.op_ex9e();
    assert_eq!(chip.pc, 0x202);

    chip.registers[3] = 16;
    chip.op_ex9e();
    assert_eq!(chip.pc, 0x202);
}

#[test]
fn op_exa1_test_0() {
    let mut chip = Chip::new();
    chip.opcode = 0xE3A1;
    chip.pc = 0x202;
    chip.registers[3] = 0xA;
    chip.op_exa1();
    assert_eq!(chip.pc, 0x204);
}

#[test]
fn op_exa1_test_1() {
    let mut chip = Chip::new();
    chip.opcode = 0xE3A1;
    chip.pc = 0x202;
    chip.registers[3] = 0xA;
    chip.keypad[0xA] = 1;
    chip.op_exa1();
    assert_eq!(chip.pc, 0x202);

    chip.registers[3] = 16;
    chip.op_exa1();
    assert_eq!(chip.pc, 0x204);
}

#[test]
fn op_fx07_test_0() {
    let mut chip = Chip::new();
    chip.opcode = 0xF307;
    chip.delay_timer = 37;
    chip.op_fx07();
    assert_eq!(chip.registers[3], 37);
    assert_eq!(chip.delay_timer, 37);
}

#[test]
fn op_fx0a_test_0() {
    let mut chip = Chip::new();
    chip.opcode = 0xF30A;
    chip.pc = 0x202;
    chip.registers[3] = 0xFF;
    chip.op_fx0a();
    assert_eq!(chip.pc, 0x200);
    assert_eq!(chip.registers[3], 0xFF);
}

#[test]
fn op_fx0a_test_1() {
    let mut chip = Chip::new();
    chip.opcode = 0xF30A;
    chip.pc = 0x202;
    chip.keypad[0xA] = 1;
    chip.keypad[2] = 1;
    chip.op_fx0a();
    assert_eq!(chip.registers[3], 2);
    assert_eq!(chip.pc, 0x202);
}

#[test]
fn op_fx15_test_0() {
    let mut chip = Chip::new();
    chip.opcode = 0xF315;
    chip.registers[3] = 42;
    chip.op_fx15();
    assert_eq!(chip.delay_timer, 42);
}

#[test]
fn op_fx18_test_0() {
    let mut chip = Chip::new();
    chip.opcode = 0xF318;
    chip.registers[3] = 42;
    chip.op_fx18();
    assert_eq!(chip.sound_timer, 42);
}

#[test]
fn op_fx1e_test_0() {
    let mut chip = Chip::new();
    chip.opcode = 0xF31E;
    chip.index = 0x300;
    chip.registers[3] = 0x40;
    chip.registers[0xF] = 1;
    chip.op_fx1e();
    assert_eq!(chip.index, 0x340);
    assert_eq!(chip.registers[0xF], 1);
}

#[test]
fn op_fx1e_test_1() {
    let mut chip = Chip::new();
    chip.opcode = 0xF31E;
    chip.index = 0xFFFF;
    chip.registers[3] = 2;
    chip.op_fx1e();
    assert_eq!(chip.index, 1);
}

#[test]
fn op_fx29_test_0() {
    let mut chip = Chip::new();
    chip.opcode = 0xF329;
    chip.registers[3] = 0xA;
    chip.op_fx29();
    assert_eq!(chip.index, 0x50 + 10 * 5);
    assert_eq!(&chip.memory[chip.index as usize..chip.index as usize + 5], &[0xF0, 0x90, 0xF0, 0x90, 0x90]);
}

#[test]
fn op_fx33_test_0() {
    let mut chip = Chip::new();
    chip.opcode = 0xF329;
    chip.registers[3] = 0x7D;
    chip.index = 0x000;
    chip.op_fx33();
    assert_eq!(chip.memory[chip.index as usize],1);
    assert_eq!(chip.memory[chip.index as usize + 1],2);
    assert_eq!(chip.memory[chip.index as usize + 2],5);
}

#[test]
fn op_fx55_test_0() {
    let mut chip = Chip::new();
    chip.opcode = 0xF255;
    chip.registers[0] = 0x70;
    chip.registers[1] = 0x71;
    chip.registers[2] = 0x72;
    chip.index = 0x001;
    chip.op_fx55();
    assert_eq!(chip.memory[0x001],0x70);
    assert_eq!(chip.memory[0x002],0x71);
    assert_eq!(chip.memory[0x003],0x72);
}

#[test]
fn op_fx65_test_0() {
    let mut chip = Chip::new();
    chip.opcode = 0xF265;
    chip.memory[0x001] = 0x70;
    chip.memory[0x002] = 0x71;
    chip.memory[0x003] = 0x72;
    chip.index = 0x001;
    chip.op_fx65();
    assert_eq!(chip.registers[0],0x70);
    assert_eq!(chip.registers[1],0x71);
    assert_eq!(chip.registers[2],0x72);

}

#[test]
fn cycle_test_0() {
    let mut chip = Chip::new();
    chip.memory[0x200..0x206].copy_from_slice(&[
        0x61, 0xFF, // V1 = 255
        0x71, 0x02, // V1 wraps to 1
        0xF1, 0x15, // delay timer = V1
    ]);

    chip.cycle().unwrap();
    chip.cycle().unwrap();
    chip.cycle().unwrap();

    assert_eq!(chip.registers[1], 1);
    assert_eq!(chip.delay_timer, 1);
    assert_eq!(chip.pc, 0x206);
}

#[test]
fn cycle_test_1() {
    let mut chip = Chip::new();
    chip.memory[0x200..0x202].copy_from_slice(&[0x81, 0x28]);

    assert_eq!(chip.cycle(), Err("Unknown opcode: 0x8128".to_owned()));
}

#[test]
fn cycle_test_2() {
    let mut chip = Chip::new();
    chip.pc = 4095;

    assert!(chip.cycle().unwrap_err().contains("Program counter"));
}

#[test]
fn tick_timers_test_0() {
    let mut chip = Chip::new();
    chip.delay_timer = 2;
    chip.sound_timer = 1;

    chip.tick_timers();
    assert_eq!(chip.delay_timer, 1);
    assert_eq!(chip.sound_timer, 0);

    chip.tick_timers();
    assert_eq!(chip.delay_timer, 0);
    assert_eq!(chip.sound_timer, 0);
}
