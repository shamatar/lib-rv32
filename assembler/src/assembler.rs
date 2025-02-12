use std::collections::HashMap;
#[cfg(not(target_arch = "wasm32"))]
use std::io::prelude::*;

use log::info;

use lib_rv32_common::constants::*;

use crate::{
    encode_b_imm, encode_func3, encode_func7, encode_i_imm, encode_j_imm, encode_opcode, encode_rd,
    encode_rs1, encode_rs2, encode_s_imm, encode_u_imm, error::AssemblerError, parse::*, tokenize,
};

pub enum InstructionFormat {
    Itype,
    Rtype,
    Jtype,
    Utype,
    Stype,
    Btype,
    SpecialCaseParamtric(
        Box<
            dyn FnOnce(
                &[String],
                &mut HashMap<String, u32>,
                u32,
                String,
            ) -> Result<u32, AssemblerError>,
        >,
    ),
}

/// Assemble a single instruction.
///
/// Parameters:
///     `ir_string: &str`: The instruction
///     `labels: &mut std::collections::HashMap<String, u32>`: Map of labels
///     `pc: u32` Current location of the program
///
/// Returns:
///     `Result<Option<u32>>`: The assembled binary instruction, an error, or nothing.
pub fn assemble_ir(
    ir_string: &str,
    labels: &mut HashMap<String, u32>,
    pc: u32,
) -> Result<Option<u32>, AssemblerError> {
    let mut msg = String::new();
    let mut ir: u32 = 0;

    let mut tokens: Vec<String> = tokenize!(ir_string);

    if tokens.is_empty() {
        return Ok(None);
    } else if tokens.len() > 5 {
        return Err(AssemblerError::TooManyTokensError);
    }

    // Add and remove leading label.
    if tokens[0].ends_with(':') {
        labels.insert(tokens[0].strip_suffix(':').unwrap().to_owned(), pc);
        tokens.remove(0);
    }

    if tokens.is_empty() {
        return Ok(None);
    }

    msg += &format!("{:18} -> [{:02x}] ", ir_string, pc);

    let op = &tokens[0];
    let (opcode, format) = match_opcode_and_format(op)?;
    ir |= encode_opcode!(opcode);

    // special cases we do immediatelly
    if let InstructionFormat::SpecialCaseParamtric(parsing_fn) = format {
        let ir = (parsing_fn)(&tokens, labels, pc, msg)?;
        return Ok(Some(ir));
    }

    // otherwise simpler cases

    // Use the destination register field.
    if let InstructionFormat::Rtype | InstructionFormat::Itype | InstructionFormat::Utype = format {
        let rd = match_register(&tokens[1])?;
        ir |= encode_rd!(rd);
    }

    // Use the first register operand and func3 fields.
    if let InstructionFormat::Itype
    | InstructionFormat::Rtype
    | InstructionFormat::Btype
    | InstructionFormat::Stype = format
    {
        let rs1 = match_register(
            &tokens[match opcode {
                OPCODE_LOAD => 3,
                OPCODE_BRANCH => 1,
                OPCODE_STORE => 3,
                OPCODE_JALR => 3,
                _ => 2,
            }],
        )?;
        ir |= encode_rs1!(rs1);
        ir |= encode_func3!(match_func3(op));
    }

    // Use the second register operand field.
    if let InstructionFormat::Rtype | InstructionFormat::Stype | InstructionFormat::Btype = format {
        let rs2 = match_register(
            &tokens[match opcode {
                OPCODE_STORE => 1,
                OPCODE_BRANCH => 2,
                OPCODE_SYSTEM => 2,
                _ => 3,
            }],
        )?;
        ir |= encode_rs2!(rs2);
    }

    // Use the func7 field.
    if matches!(format, InstructionFormat::Rtype) || matches!(format, InstructionFormat::Itype if opcode == OPCODE_ARITHMETIC_IMM && matches!(match_func3(op), FUNC3_SLL | FUNC3_SR)) {
        ir |= encode_func7!(match_func7(op));
    }

    match format {
        InstructionFormat::Itype => {
            let imm = parse_imm(
                &tokens[match opcode {
                    OPCODE_LOAD => 2,
                    OPCODE_STORE => 2,
                    OPCODE_JALR => 2,
                    _ => 3,
                }],
                labels,
                pc,
            )?;
            ir |= encode_i_imm!(imm);
        }
        InstructionFormat::Utype => {
            let imm = parse_imm(&tokens[2], labels, pc)?;
            ir |= encode_u_imm!(imm);
        }
        InstructionFormat::Jtype => {
            let imm = parse_imm(&tokens[2], labels, pc)?;
            ir |= encode_j_imm!(imm);
        }
        InstructionFormat::Btype => {
            let imm = parse_imm(&tokens[3], labels, pc)?;
            ir |= encode_b_imm!(imm);
        }
        InstructionFormat::Stype => {
            let imm = parse_imm(&tokens[2], labels, pc)?;
            ir |= encode_s_imm!(imm);
        }
        InstructionFormat::Rtype => (),
        InstructionFormat::SpecialCaseParamtric(..) => {
            unreachable!()
        }
    }

    msg += &format!("{:08x}", ir);
    info!("{}", msg);

    Ok(Some(ir))
}

/// Assemble a `BufRead` down to a vector of words. The input should contain
/// the entire program.
#[cfg(not(target_arch = "wasm32"))]
pub fn assemble_program_buf<R>(reader: &mut R) -> Result<Vec<u32>, AssemblerError>
where
    R: BufRead,
{
    let mut prog = Vec::new();
    let mut buf = String::new();
    let mut labels = HashMap::new();
    let mut pc: u32 = 0;

    loop {
        let bytes_rd = reader.read_line(&mut buf);

        if bytes_rd.is_err() {
            return Err(AssemblerError::IOError);
        }

        if bytes_rd.unwrap() == 0 {
            break;
        }

        let ir = assemble_ir(buf.trim_end(), &mut labels, pc);

        if let Err(why) = ir {
            return Err(why);
        }

        if let Some(i) = ir.unwrap() {
            prog.push(i);
            pc += 4;
        }
        buf.clear();
    }

    Ok(prog)
}

/// Assemble a full program of newline-separated instructions.
pub fn assemble_program(program: &str) -> Result<Vec<u32>, AssemblerError> {
    let mut prog = Vec::new();
    let mut labels = HashMap::new();
    let mut pc: u32 = 0;

    for line in program.split("\n") {
        let ir = assemble_ir(line, &mut labels, pc);

        if let Err(why) = ir {
            return Err(why);
        }

        if let Some(i) = ir.unwrap() {
            prog.push(i);
            pc += 4;
        }
    }

    Ok(prog)
}
