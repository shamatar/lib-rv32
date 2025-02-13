use crate::{encode_opcode, encode_rd, encode_rs1, error::AssemblerError, parse::*};

use crate::{encode::*, encode_func3};
use lib_rv32_common::constants::*;
use log::info;
use std::collections::HashMap;

pub fn encode_fence(
    _tokens: &[String],
    _labels: &mut HashMap<String, u32>,
    _pc: u32,
    mut msg: String,
) -> Result<u32, AssemblerError> {
    let mut ir = 0u32;

    ir |= encode_opcode!(OPCODE_MISC_MEM);

    ir |= encode_func3!(FUNC3_FENCE);

    msg += &format!("{:08x}", ir);
    info!("{}", msg);

    Ok(ir)
}

pub fn encode_fencei(
    _tokens: &[String],
    _labels: &mut HashMap<String, u32>,
    _pc: u32,
    mut msg: String,
) -> Result<u32, AssemblerError> {
    let mut ir = 0u32;

    ir |= encode_opcode!(OPCODE_MISC_MEM);

    ir |= encode_func3!(FUNC3_FENCEI);

    msg += &format!("{:08x}", ir);
    info!("{}", msg);

    Ok(ir)
}

fn encode_priv(
    opcode: u8,
    _tokens: &[String],
    _labels: &mut HashMap<String, u32>,
    _pc: u32,
    mut msg: String,
    func12: u16,
) -> Result<u32, AssemblerError> {
    let mut ir = 0u32;

    ir |= encode_opcode!(opcode);

    ir |= encode_func3!(FUNC3_PRIV);

    ir = encode_func12(ir, func12)?;

    msg += &format!("{:08x}", ir);
    info!("{}", msg);

    Ok(ir)
}

pub fn create_encode_priv_fn(
    opcode: u8,
    func12: u16,
) -> Box<dyn FnOnce(&[String], &mut HashMap<String, u32>, u32, String) -> Result<u32, AssemblerError>>
{
    Box::new(
        move |tokens: &[String], labels: &mut HashMap<String, u32>, pc: u32, msg: String| {
            encode_priv(opcode, tokens, labels, pc, msg, func12)
        },
    )
}

pub fn parse_csr_with_reg(
    opcode: u8,
    tokens: &[String],
    _labels: &mut HashMap<String, u32>,
    _pc: u32,
    mut msg: String,
    func3: u8,
) -> Result<u32, AssemblerError> {
    let mut ir = 0u32;

    ir |= encode_opcode!(opcode);

    let rd = match_register(&tokens[1])?;
    ir |= encode_rd!(rd);

    ir |= encode_func3!(func3);

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
    func3: u8,
) -> Result<u32, AssemblerError> {
    let mut ir = 0u32;

    ir |= encode_opcode!(opcode);

    let rd = match_register(&tokens[1])?;
    ir |= encode_rd!(rd);

    ir |= encode_func3!(func3);

    let csr_index = parse_csr(&tokens[2]);
    ir = encode_csr_index(ir, csr_index)?;

    let uimm = parse_uimm(&tokens[3], labels, pc)?;
    ir = encode_csr_uimm(ir, uimm)?;

    msg += &format!("{:08x}", ir);
    info!("{}", msg);

    Ok(ir)
}

pub fn create_csr_parse_with_reg(
    opcode: u8,
    func3: u8,
) -> Box<dyn FnOnce(&[String], &mut HashMap<String, u32>, u32, String) -> Result<u32, AssemblerError>>
{
    Box::new(
        move |tokens: &[String], labels: &mut HashMap<String, u32>, pc: u32, msg: String| {
            parse_csr_with_reg(opcode, tokens, labels, pc, msg, func3)
        },
    )
}

pub fn create_csr_parse_with_imm(
    opcode: u8,
    func3: u8,
) -> Box<dyn FnOnce(&[String], &mut HashMap<String, u32>, u32, String) -> Result<u32, AssemblerError>>
{
    Box::new(
        move |tokens: &[String], labels: &mut HashMap<String, u32>, pc: u32, msg: String| {
            parse_csr_with_imm(opcode, tokens, labels, pc, msg, func3)
        },
    )
}
