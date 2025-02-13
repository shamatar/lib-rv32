pub const OPCODE_LUI: u8 = 0b0110111;
pub const OPCODE_AUIPC: u8 = 0b0010111;
pub const OPCODE_JAL: u8 = 0b1101111;
pub const OPCODE_JALR: u8 = 0b1100111;
pub const OPCODE_BRANCH: u8 = 0b1100011;
pub const OPCODE_LOAD: u8 = 0b0000011;
pub const OPCODE_STORE: u8 = 0b0100011;
pub const OPCODE_ARITHMETIC_IMM: u8 = 0b0010011;
pub const OPCODE_ARITHMETIC: u8 = 0b0110011;
pub const OPCODE_SYSTEM: u8 = 0b1110011;
pub const OPCODE_MISC_MEM: u8 = 0b0001111;

pub const FUNC3_BEQ: u8 = 0b000;
pub const FUNC3_BNE: u8 = 0b001;
pub const FUNC3_BLT: u8 = 0b100;
pub const FUNC3_BGE: u8 = 0b101;
pub const FUNC3_BLTU: u8 = 0b110;
pub const FUNC3_BGEU: u8 = 0b111;
pub const FUNC3_LB: u8 = 0b000;
pub const FUNC3_LH: u8 = 0b001;
pub const FUNC3_LW: u8 = 0b010;
pub const FUNC3_LBU: u8 = 0b100;
pub const FUNC3_LHU: u8 = 0b101;
pub const FUNC3_SB: u8 = 0b000;
pub const FUNC3_SH: u8 = 0b001;
pub const FUNC3_SW: u8 = 0b010;
pub const FUNC3_ADD_SUB: u8 = 0b000;
pub const FUNC3_SLL: u8 = 0b001;
pub const FUNC3_SLT: u8 = 0b010;
pub const FUNC3_SLTU: u8 = 0b011;
pub const FUNC3_XOR: u8 = 0b100;
pub const FUNC3_SR: u8 = 0b101;
pub const FUNC3_OR: u8 = 0b110;
pub const FUNC3_AND: u8 = 0b111;
pub const FUNC3_MUL: u8 = 0b000;
pub const FUNC3_MULH: u8 = 0b001;
pub const FUNC3_MULHSU: u8 = 0b010;
pub const FUNC3_MULHU: u8 = 0b011;
pub const FUNC3_DIV: u8 = 0b100;
pub const FUNC3_DIVU: u8 = 0b101;
pub const FUNC3_REM: u8 = 0b110;
pub const FUNC3_REMU: u8 = 0b111;
pub const FUNC3_CSRRW: u8 = 0b001;
pub const FUNC3_CSRRS: u8 = 0b010;
pub const FUNC3_CSRRC: u8 = 0b011;
pub const FUNC3_CSRRWI: u8 = 0b101;
pub const FUNC3_CSRRSI: u8 = 0b110;
pub const FUNC3_CSRRCI: u8 = 0b111;
pub const FUNC3_FENCE: u8 = 0b000;
pub const FUNC3_FENCEI: u8 = 0b001;
pub const FUNC3_PRIV: u8 = 0b000;
pub const FUNC3_JALR: u8 = 0b000;

pub const FUNC7_ADD: u8 = 0b0000000;
pub const FUNC7_SUB: u8 = 0b0100000;
pub const FUNC7_SLT: u8 = 0b0000000;
pub const FUNC7_SLTU: u8 = 0b0000000;
pub const FUNC7_SLL: u8 = 0b0000000;
pub const FUNC7_SRL: u8 = 0b0000000;
pub const FUNC7_SRA: u8 = 0b0100000;
pub const FUNC7_AND: u8 = 0b0000000;
pub const FUNC7_OR: u8 = 0b0000000;
pub const FUNC7_XOR: u8 = 0b0000000;
pub const FUNC7_MUL: u8 = 0b0000001;
pub const FUNC7_DIV: u8 = 0b0000001;

pub const FUNC12_MRET: u16 = 0b001100000010;
pub const FUNC12_WFI: u16 = 0b000100000101;
pub const FUNC12_ECALL: u16 = 0b000000000000;
pub const FUNC12_EBREAK: u16 = 0b000000000001;

/// Array to match register numbers to their common names.
pub static REG_NAMES: &[&str] = &[
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6",
];
