use std::collections::HashMap;

use lib_rv32_common::{constants::*, csr::*, parse_int};

use crate::error::AssemblerError;
use crate::InstructionFormat;

/// Convert an instruction to it's tokens, stripping out whitespace,
/// parenthesis, and commas.
#[macro_export]
macro_rules! tokenize {
    ($s:expr) => {
        $s.replace(",", " ")
            .replace("\n", " ")
            .replace("(", " ")
            .replace(")", " ")
            .to_ascii_lowercase()
            .split_whitespace()
            .map(|s| s.to_owned())
            .collect()
    };
}

// /// Match an operation to the correct opcode.
// pub fn match_opcode(op: &str) -> Result<u8, AssemblerError> {
//     let opcode = match op {
//         "add" | "sub" | "sll" | "srl" | "slt" | "sltu" | "xor" | "sra" | "or" | "and" => OPCODE_ARITHMETIC,
//         "addi" | "slli" | "srli" | "slti" | "sltiu" | "xori" | "srai" | "ori" | "andi" => OPCODE_ARITHMETIC_IMM,
//         "lui" => OPCODE_LUI,
//         "auipc" => OPCODE_AUIPC,
//         "jal" => OPCODE_JAL,
//         "jalr" => OPCODE_JALR,
//         "beq" | "bne" | "blt" | "bge" | "bgeu" => OPCODE_BRANCH,
//         "lb" | "lbu" | "lh" | "lhu" | "lw" => OPCODE_LOAD,
//         "sb" | "sh" | "sw" => OPCODE_STORE,
//         "mul" | "mulh" | "mulhsu" | "mulhu" | "div" | "divu" | "rem" | "remu" => OPCODE_ARITHMETIC,
//         "csrrw" | "csrrs" | "csrrc" | "csrrwi" | "csrrsi" | "csrrci" => OPCODE_SYSTEM,
//         _ => return Err(AssemblerError::InvalidOperationError),
//     };
//     Ok(opcode)
// }

/// Match an operation to the correct opcode.
pub fn match_opcode_and_format(op: &str) -> Result<(u8, InstructionFormat), AssemblerError> {
    let opcode = match op {
        "add" | "sub" | "sll" | "srl" | "slt" | "sltu" | "xor" | "sra" | "or" | "and" => {
            OPCODE_ARITHMETIC
        }
        "addi" | "slli" | "srli" | "slti" | "sltiu" | "xori" | "srai" | "ori" | "andi" => {
            OPCODE_ARITHMETIC_IMM
        }
        "lui" => OPCODE_LUI,
        "auipc" => OPCODE_AUIPC,
        "jal" => OPCODE_JAL,
        "jalr" => OPCODE_JALR,
        "beq" | "bne" | "blt" | "bge" | "bgeu" => OPCODE_BRANCH,
        "lb" | "lbu" | "lh" | "lhu" | "lw" => OPCODE_LOAD,
        "sb" | "sh" | "sw" => OPCODE_STORE,
        "mul" | "mulh" | "mulhsu" | "mulhu" | "div" | "divu" | "rem" | "remu" => OPCODE_ARITHMETIC,
        "csrrw" | "csrrs" | "csrrc" => {
            return Ok((OPCODE_SYSTEM, InstructionFormat::SystemQuasiRType));
        }
        "csrrwi" | "csrrsi" | "csrrci" => {
            return Ok((OPCODE_SYSTEM, InstructionFormat::SystemQuasiIType));
        }
        _ => return Err(AssemblerError::InvalidOperationError),
    };

    // Use the opcode to identify the instruction format.
    let format = match opcode {
        OPCODE_ARITHMETIC_IMM | OPCODE_JALR | OPCODE_LOAD => InstructionFormat::Itype,
        OPCODE_ARITHMETIC => InstructionFormat::Rtype,
        OPCODE_JAL => InstructionFormat::Jtype,
        OPCODE_LUI | OPCODE_AUIPC => InstructionFormat::Utype,
        OPCODE_BRANCH => InstructionFormat::Btype,
        OPCODE_STORE => InstructionFormat::Stype,
        // OPCODE_FENCE => FENCE.I
        _ => unreachable!("encountered 0b{:b}", opcode),
    };

    Ok((opcode, format))
}

/// Match a register number or name to its integer number.
pub fn match_register(reg: &str) -> Result<u8, AssemblerError> {
    if reg.starts_with('x') {
        match reg.strip_prefix('x').unwrap().parse() {
            Ok(n) => Ok(n),
            Err(_) => Err(AssemblerError::NoSuchRegisterError),
        }
    } else {
        match REG_NAMES.iter().position(|e| *e == reg) {
            Some(n) => Ok(n as u8),
            None => Err(AssemblerError::NoSuchRegisterError),
        }
    }
}

/// Parse a label or an immediate literal into an integer.
pub fn parse_imm(s: &str, labels: &HashMap<String, u32>, pc: u32) -> Result<u32, AssemblerError> {
    let num = parse_int!(i64, s);
    match num {
        Err(_) => {
            let label = labels.get(s);
            if let Some(v) = label {
                Ok((*v).wrapping_sub(pc))
            } else {
                Err(AssemblerError::InvalidImmediateError)
            }
        }
        Ok(d) => Ok(d as u32),
    }
}

pub fn parse_uimm(s: &str, labels: &HashMap<String, u32>, pc: u32) -> Result<u32, AssemblerError> {
    let num = parse_int!(u64, s);
    match num {
        Err(_) => {
            let label = labels.get(s);
            if let Some(v) = label {
                Ok((*v).wrapping_sub(pc))
            } else {
                Err(AssemblerError::InvalidImmediateError)
            }
        }
        Ok(d) => Ok(d as u32),
    }
}

/// Match an operation to the correct func3.
pub fn match_func3(t: &str) -> u8 {
    match t {
        "beq" => FUNC3_BEQ,
        "bne" => FUNC3_BNE,
        "blt" => FUNC3_BLT,
        "bge" => FUNC3_BGE,
        "bltu" => FUNC3_BLTU,
        "bgeu" => FUNC3_BGEU,
        "lb" => FUNC3_LB,
        "lbu" => FUNC3_LBU,
        "lh" => FUNC3_LH,
        "lhu" => FUNC3_LHU,
        "lw" => FUNC3_LW,
        "sb" => FUNC3_SB,
        "sh" => FUNC3_SH,
        "sw" => FUNC3_SW,
        "add" | "addi" | "sub" => FUNC3_ADD_SUB,
        "sll" | "slli" => FUNC3_SLL,
        "slt" | "slti" => FUNC3_SLT,
        "sltu" | "sltiu" => FUNC3_SLTU,
        "xor" | "xori" => FUNC3_XOR,
        "sra" | "srai" | "srl" | "srli" => FUNC3_SR,
        "or" | "ori" => FUNC3_OR,
        "and" | "andi" => FUNC3_AND,
        "mul" => FUNC3_MUL,
        "mulh" => FUNC3_MULH,
        "mulhsu" => FUNC3_MULHSU,
        "mulhu" => FUNC3_MULHU,
        "div" => FUNC3_DIV,
        "divu" => FUNC3_DIVU,
        "rem" => FUNC3_REM,
        "remu" => FUNC3_REMU,
        "csrrw" => FUNC3_CSRRW,
        "csrrs" => FUNC3_CSRRS,
        "csrrc" => FUNC3_CSRRC,
        "csrrwi" => FUNC3_CSRRWI,
        "csrrsi" => FUNC3_CSRRSI,
        "csrrci" => FUNC3_CSRRCI,
        "fence.i" => FUNC3_FENCEI,
        _ => unreachable!("encountered {}", t),
    }
}

/// Match an operation to the correct func7.
pub fn match_func7(t: &str) -> u8 {
    match t {
        "add" => FUNC7_ADD,
        "sub" => FUNC7_SUB,
        "sra" => FUNC7_SRA,
        "srl" => FUNC7_SRL,
        "sll" => FUNC7_SLL,
        "slt" => FUNC7_SLT,
        "sltu" => FUNC7_SLTU,
        "and" => FUNC7_AND,
        "or" => FUNC7_OR,
        "xor" => FUNC7_XOR,
        "mul" | "mulh" | "mulhsu" | "mulhu" => FUNC7_MUL,
        "div" | "divu" | "rem" | "remu" => FUNC7_MUL,
        _ => unreachable!("encountered {}", t),
    }
}

pub fn parse_csr(t: &str) -> u32 {
    match t {
        "cycle" => CSR_RDCYCLE,
        "time" => CSR_RDTIME,
        "instret" => CSR_RDINSTRET,
        "cycleh" => CSR_RDCYCLEH,
        "timeh" => CSR_RDTIMEH,
        "instreth" => CSR_RDINSTRETH,
        _ => unreachable!("encountered {}", t),
    }
}
