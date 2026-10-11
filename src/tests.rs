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

