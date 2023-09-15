use crate::{encode_opcode, encode_rd, encode_rs1, error::AssemblerError, parse::*};

use crate::{encode::*, encode_func3};
use lib_rv32_common::constants::{FUNC3_FENCE, FUNC3_FENCEI};
use log::info;
use std::collections::HashMap;

pub fn encode_fence(
    opcode: u8,
    _tokens: &[String],
    _labels: &mut HashMap<String, u32>,
    _pc: u32,
    mut msg: String,
) -> Result<u32, AssemblerError> {
    let mut ir = 0u32;

    ir |= encode_opcode!(opcode);

    ir |= encode_func3!(FUNC3_FENCE);

    msg += &format!("{:08x}", ir);
    info!("{}", msg);

    Ok(ir)
}

pub fn encode_fencei(
    opcode: u8,
    _tokens: &[String],
    _labels: &mut HashMap<String, u32>,
    _pc: u32,
    mut msg: String,
) -> Result<u32, AssemblerError> {
    let mut ir = 0u32;

    ir |= encode_opcode!(opcode);

    ir |= encode_func3!(FUNC3_FENCEI);

    msg += &format!("{:08x}", ir);
    info!("{}", msg);

    Ok(ir)
}

pub fn parse_csr_with_reg(
    opcode: u8,
    tokens: &[String],
    _labels: &mut HashMap<String, u32>,
    _pc: u32,
    mut msg: String,
) -> Result<u32, AssemblerError> {
    let mut ir = 0u32;

    ir |= encode_opcode!(opcode);

    let rd = match_register(&tokens[1])?;
    ir |= encode_rd!(rd);

    let csr_index = parse_csr(&tokens[2]);
    ir = encode_csr_index(ir, csr_index)?;

    let rs1 = match_register(&tokens[3])?;
    ir |= encode_rs1!(rs1);

    msg += &format!("{:08x}", ir);
    info!("{}", msg);

    Ok(ir)
}

pub fn parse_csr_with_imm(
    opcode: u8,
    tokens: &[String],
    labels: &mut HashMap<String, u32>,
    pc: u32,
    mut msg: String,
) -> Result<u32, AssemblerError> {
    let mut ir = 0u32;

    ir |= encode_opcode!(opcode);

    let rd = match_register(&tokens[1])?;
    ir |= encode_rd!(rd);

    let csr_index = parse_csr(&tokens[2]);
    ir = encode_csr_index(ir, csr_index)?;

    let uimm = parse_uimm(&tokens[3], labels, pc)?;
    ir = encode_csr_uimm(ir, uimm)?;

    msg += &format!("{:08x}", ir);
    info!("{}", msg);

    Ok(ir)
}
