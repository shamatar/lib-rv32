const REG_MASK: u8 = 0b1_1111;
const FUNC3_MASK: u8 = 0b111;
const FUNC7_MASK: u8 = 0b111_1111;
const FUNC12_MASK: u16 = 0b1111_1111_1111;
const OPCODE_MASK: u8 = 0b111_1111;

const RD_OFFSET: u32 = 7;
const RS1_OFFSET: u32 = 15;
const RS2_OFFSET: u32 = 20;
const OPCODE_OFFSET: u32 = 0;

const FUNC3_OFFSET: u32 = 12;
const FUNC7_OFFSET: u32 = 25;
const FUNC12_OFFSET: u32 = 20;

const OVERRIDE_RD_MASK: u32 = !((REG_MASK as u32) << RD_OFFSET);
const OVERRIDE_RS1_MASK: u32 = !((REG_MASK as u32) << RS1_OFFSET);
const OVERRIDE_RS2_MASK: u32 = !((REG_MASK as u32) << RS2_OFFSET);
const OVERRIDE_OPCODE_MASK: u32 = !((OPCODE_MASK as u32) << OPCODE_OFFSET);
const OVERRIDE_FUNC3_MASK: u32 = !((FUNC3_MASK as u32) << FUNC3_OFFSET);
const OVERRIDE_FUNC7_MASK: u32 = !((FUNC7_MASK as u32) << FUNC7_OFFSET);
const OVERRIDE_FUNC12_MASK: u32 = !((FUNC12_MASK as u32) << FUNC12_OFFSET);

pub fn override_rd(opcode: u32, new_rd: u8) -> u32 {
    (opcode & OVERRIDE_RD_MASK) | (((new_rd & REG_MASK) as u32) << RD_OFFSET)
}

pub fn override_rs1(opcode: u32, new_rs1: u8) -> u32 {
    (opcode & OVERRIDE_RS1_MASK) | (((new_rs1 & REG_MASK) as u32) << RS1_OFFSET)
}

pub fn override_rs2(opcode: u32, new_rs2: u8) -> u32 {
    (opcode & OVERRIDE_RS2_MASK) | (((new_rs2 & REG_MASK) as u32) << RS2_OFFSET)
}

pub fn override_opcode(encoding: u32, new: u8) -> u32 {
    (encoding & OVERRIDE_OPCODE_MASK) | (((new & OPCODE_MASK) as u32) << OPCODE_OFFSET)
}

pub fn override_func3(opcode: u32, new: u8) -> u32 {
    (opcode & OVERRIDE_FUNC3_MASK) | (((new & FUNC3_MASK) as u32) << FUNC3_OFFSET)
}

pub fn override_func7(opcode: u32, new: u8) -> u32 {
    (opcode & OVERRIDE_FUNC7_MASK) | (((new & FUNC7_MASK) as u32) << FUNC7_OFFSET)
}

pub fn override_func12(opcode: u32, new: u16) -> u32 {
    (opcode & OVERRIDE_FUNC12_MASK) | (((new & FUNC12_MASK) as u32) << FUNC12_OFFSET)
}
