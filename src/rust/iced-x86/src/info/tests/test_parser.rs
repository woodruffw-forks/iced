// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

use crate::iced_constants::*;
use crate::info::tests::constants::*;
use crate::info::tests::info_test_case::*;
use crate::test_utils::from_str_conv::*;
use crate::test_utils::get_default_ip;
use crate::*;
use alloc::string::String;
use alloc::vec::Vec;
use core::iter::IntoIterator;
use core::{i16, i32, u16, u32};
use lazy_static::lazy_static;
use static_assertions::const_assert_eq;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines};
use std::path::Path;

pub(super) struct InstrInfoTestParser {
	filename: String,
	lines: Lines<BufReader<File>>,
	bitness: u32,
}

impl InstrInfoTestParser {
	pub(super) fn new(bitness: u32, filename: &Path) -> Self {
		let display_filename = filename.display().to_string();
		let file = File::open(filename).unwrap_or_else(|_| panic!("Couldn't open file {}", display_filename));
		let lines = BufReader::new(file).lines();
		Self { filename: display_filename, lines, bitness }
	}
}

impl IntoIterator for InstrInfoTestParser {
	type Item = InstrInfoTestCase;
	type IntoIter = IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		let mut to_register = clone_register_hashmap();

		// GENERATOR-BEGIN: OpAccessDict
		// ⚠️This was generated by GENERATOR!🦹‍♂️
		let mut to_access: HashMap<&'static str, OpAccess> = HashMap::with_capacity(8);
		let _ = to_access.insert("n", OpAccess::None);
		let _ = to_access.insert("r", OpAccess::Read);
		let _ = to_access.insert("cr", OpAccess::CondRead);
		let _ = to_access.insert("w", OpAccess::Write);
		let _ = to_access.insert("cw", OpAccess::CondWrite);
		let _ = to_access.insert("rw", OpAccess::ReadWrite);
		let _ = to_access.insert("rcw", OpAccess::ReadCondWrite);
		let _ = to_access.insert("nma", OpAccess::NoMemAccess);
		// GENERATOR-END: OpAccessDict

		match self.bitness {
			16 => {
				let _ = to_register.insert(MiscInstrInfoTestConstants::XSP.to_string(), Register::SP);
				let _ = to_register.insert(MiscInstrInfoTestConstants::XBP.to_string(), Register::BP);
			}
			32 => {
				let _ = to_register.insert(MiscInstrInfoTestConstants::XSP.to_string(), Register::ESP);
				let _ = to_register.insert(MiscInstrInfoTestConstants::XBP.to_string(), Register::EBP);
			}
			64 => {
				let _ = to_register.insert(MiscInstrInfoTestConstants::XSP.to_string(), Register::RSP);
				let _ = to_register.insert(MiscInstrInfoTestConstants::XBP.to_string(), Register::RBP);
			}
			_ => unreachable!(),
		}

		for i in 0..IcedConstants::VMM_COUNT {
			let register = IcedConstants::VMM_FIRST + i;
			let _ = to_register.insert(format!("{}{}", MiscInstrInfoTestConstants::VMM_PREFIX, i), register);
		}

		IntoIter { filename: self.filename, lines: self.lines, bitness: self.bitness, line_number: 0, to_register, to_access }
	}
}

pub(super) struct IntoIter {
	filename: String,
	lines: Lines<BufReader<File>>,
	bitness: u32,
	line_number: u32,
	to_register: HashMap<String, Register>,
	to_access: HashMap<&'static str, OpAccess>,
}

// GENERATOR-BEGIN: KeysConstants
// ⚠️This was generated by GENERATOR!🦹‍♂️
lazy_static! {
	pub(super) static ref TO_INSTRUCTION_INFO_KEYS: HashMap<&'static str, u32> = {
		let mut h = HashMap::with_capacity(31);
		let _ = h.insert("priv", InstructionInfoKeys::IS_PRIVILEGED);
		let _ = h.insert("save-restore", InstructionInfoKeys::IS_SAVE_RESTORE_INSTRUCTION);
		let _ = h.insert("stack", InstructionInfoKeys::IS_STACK_INSTRUCTION);
		let _ = h.insert("special", InstructionInfoKeys::IS_SPECIAL);
		let _ = h.insert("fr", InstructionInfoKeys::RFLAGS_READ);
		let _ = h.insert("fu", InstructionInfoKeys::RFLAGS_UNDEFINED);
		let _ = h.insert("fw", InstructionInfoKeys::RFLAGS_WRITTEN);
		let _ = h.insert("fc", InstructionInfoKeys::RFLAGS_CLEARED);
		let _ = h.insert("fs", InstructionInfoKeys::RFLAGS_SET);
		let _ = h.insert("flow", InstructionInfoKeys::FLOW_CONTROL);
		let _ = h.insert("op0", InstructionInfoKeys::OP0_ACCESS);
		let _ = h.insert("op1", InstructionInfoKeys::OP1_ACCESS);
		let _ = h.insert("op2", InstructionInfoKeys::OP2_ACCESS);
		let _ = h.insert("op3", InstructionInfoKeys::OP3_ACCESS);
		let _ = h.insert("op4", InstructionInfoKeys::OP4_ACCESS);
		let _ = h.insert("r", InstructionInfoKeys::READ_REGISTER);
		let _ = h.insert("cr", InstructionInfoKeys::COND_READ_REGISTER);
		let _ = h.insert("w", InstructionInfoKeys::WRITE_REGISTER);
		let _ = h.insert("cw", InstructionInfoKeys::COND_WRITE_REGISTER);
		let _ = h.insert("rw", InstructionInfoKeys::READ_WRITE_REGISTER);
		let _ = h.insert("rcw", InstructionInfoKeys::READ_COND_WRITE_REGISTER);
		let _ = h.insert("rm", InstructionInfoKeys::READ_MEMORY);
		let _ = h.insert("crm", InstructionInfoKeys::COND_READ_MEMORY);
		let _ = h.insert("rwm", InstructionInfoKeys::READ_WRITE_MEMORY);
		let _ = h.insert("rcwm", InstructionInfoKeys::READ_COND_WRITE_MEMORY);
		let _ = h.insert("wm", InstructionInfoKeys::WRITE_MEMORY);
		let _ = h.insert("cwm", InstructionInfoKeys::COND_WRITE_MEMORY);
		let _ = h.insert("decopt", InstructionInfoKeys::DECODER_OPTIONS);
		let _ = h.insert("fpu-inc", InstructionInfoKeys::FPU_TOP_INCREMENT);
		let _ = h.insert("fpu-cond", InstructionInfoKeys::FPU_CONDITIONAL_TOP);
		let _ = h.insert("fpu-writes-top", InstructionInfoKeys::FPU_WRITES_TOP);
		h
	};
}

pub(crate) struct InstructionInfoKeys;
#[allow(dead_code)]
impl InstructionInfoKeys {
	pub(crate) const IS_PRIVILEGED: u32 = 0;
	pub(crate) const IS_SAVE_RESTORE_INSTRUCTION: u32 = 1;
	pub(crate) const IS_STACK_INSTRUCTION: u32 = 2;
	pub(crate) const IS_SPECIAL: u32 = 3;
	pub(crate) const RFLAGS_READ: u32 = 4;
	pub(crate) const RFLAGS_UNDEFINED: u32 = 5;
	pub(crate) const RFLAGS_WRITTEN: u32 = 6;
	pub(crate) const RFLAGS_CLEARED: u32 = 7;
	pub(crate) const RFLAGS_SET: u32 = 8;
	pub(crate) const FLOW_CONTROL: u32 = 9;
	pub(crate) const OP0_ACCESS: u32 = 10;
	pub(crate) const OP1_ACCESS: u32 = 11;
	pub(crate) const OP2_ACCESS: u32 = 12;
	pub(crate) const OP3_ACCESS: u32 = 13;
	pub(crate) const OP4_ACCESS: u32 = 14;
	pub(crate) const READ_REGISTER: u32 = 15;
	pub(crate) const COND_READ_REGISTER: u32 = 16;
	pub(crate) const WRITE_REGISTER: u32 = 17;
	pub(crate) const COND_WRITE_REGISTER: u32 = 18;
	pub(crate) const READ_WRITE_REGISTER: u32 = 19;
	pub(crate) const READ_COND_WRITE_REGISTER: u32 = 20;
	pub(crate) const READ_MEMORY: u32 = 21;
	pub(crate) const COND_READ_MEMORY: u32 = 22;
	pub(crate) const READ_WRITE_MEMORY: u32 = 23;
	pub(crate) const READ_COND_WRITE_MEMORY: u32 = 24;
	pub(crate) const WRITE_MEMORY: u32 = 25;
	pub(crate) const COND_WRITE_MEMORY: u32 = 26;
	pub(crate) const DECODER_OPTIONS: u32 = 27;
	pub(crate) const FPU_TOP_INCREMENT: u32 = 28;
	pub(crate) const FPU_CONDITIONAL_TOP: u32 = 29;
	pub(crate) const FPU_WRITES_TOP: u32 = 30;
}
// GENERATOR-END: KeysConstants

// GENERATOR-BEGIN: RflagsBitsConstants
// ⚠️This was generated by GENERATOR!🦹‍♂️
pub(crate) struct RflagsBitsConstants;
#[allow(dead_code)]
impl RflagsBitsConstants {
	pub(crate) const AF: char = 'a';
	pub(crate) const CF: char = 'c';
	pub(crate) const OF: char = 'o';
	pub(crate) const PF: char = 'p';
	pub(crate) const SF: char = 's';
	pub(crate) const ZF: char = 'z';
	pub(crate) const IF: char = 'i';
	pub(crate) const DF: char = 'd';
	pub(crate) const AC: char = 'A';
	pub(crate) const C0: char = '0';
	pub(crate) const C1: char = '1';
	pub(crate) const C2: char = '2';
	pub(crate) const C3: char = '3';
	pub(crate) const UIF: char = 'u';
}
// GENERATOR-END: RflagsBitsConstants

impl Iterator for IntoIter {
	type Item = InstrInfoTestCase;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			let result = match self.lines.next()? {
				Ok(line) => {
					self.line_number += 1;
					if line.is_empty() || line.starts_with('#') {
						continue;
					}
					self.read_next_test_case(line, self.line_number)
				}
				Err(err) => Err(err.to_string()),
			};
			match result {
				Ok(tc) => {
					if let Some(tc) = tc {
						return Some(tc);
					} else {
						continue;
					}
				}
				Err(err) => panic!("Error parsing instruction info test case file '{}', line {}: {}", self.filename, self.line_number, err),
			}
		}
	}
}

impl IntoIter {
	fn read_next_test_case(&self, line: String, line_number: u32) -> Result<Option<InstrInfoTestCase>, String> {
		const_assert_eq!(MiscInstrInfoTestConstants::INSTR_INFO_ELEMS_PER_LINE, 5);
		let elems: Vec<_> = line.splitn(MiscInstrInfoTestConstants::INSTR_INFO_ELEMS_PER_LINE, ',').collect();
		if elems.len() != MiscInstrInfoTestConstants::INSTR_INFO_ELEMS_PER_LINE {
			return Err(format!("Invalid number of commas: {}", elems.len() - 1));
		}

		let mut tc = InstrInfoTestCase::default();
		tc.line_number = line_number;
		tc.bitness = self.bitness;

		tc.hex_bytes = elems[0].trim().to_string();
		tc.ip = get_default_ip(tc.bitness);
		let _ = to_vec_u8(&tc.hex_bytes)?;
		if is_ignored_code(elems[1]) {
			return Ok(None);
		}
		tc.code = to_code(elems[1])?;
		tc.encoding = to_encoding_kind(elems[2])?;
		let cpuid_feature_coll: Vec<_> = elems[3].trim().split(';').collect();

		tc.cpuid_features = Vec::with_capacity(cpuid_feature_coll.len());
		for s in cpuid_feature_coll {
			tc.cpuid_features.push(to_cpuid_features(s)?);
		}

		for kv in elems[4].split_whitespace() {
			if kv.trim().is_empty() {
				continue;
			}
			let key;
			let value;
			if let Some(index) = kv.find('=') {
				let s = kv.split_at(index);
				key = s.0;
				value = &s.1[1..];
			} else {
				key = kv;
				value = "";
			}

			match *(*TO_INSTRUCTION_INFO_KEYS).get(key).unwrap_or(&u32::MAX) {
				InstructionInfoKeys::IS_PRIVILEGED => {
					if !value.is_empty() {
						return Err(format!("Invalid key-value value, '{}'", kv));
					}
					tc.is_privileged = true;
				}
				InstructionInfoKeys::IS_SAVE_RESTORE_INSTRUCTION => {
					if !value.is_empty() {
						return Err(format!("Invalid key-value value, '{}'", kv));
					}
					tc.is_save_restore_instruction = true;
				}
				InstructionInfoKeys::IS_STACK_INSTRUCTION => {
					tc.is_stack_instruction = true;
					tc.stack_pointer_increment = to_i32(value)?;
				}
				InstructionInfoKeys::IS_SPECIAL => {
					if !value.is_empty() {
						return Err(format!("Invalid key-value value, '{}'", kv));
					}
					tc.is_special = true;
				}
				InstructionInfoKeys::RFLAGS_READ => tc.rflags_read |= IntoIter::parse_rflags(value)?,
				InstructionInfoKeys::RFLAGS_UNDEFINED => tc.rflags_undefined |= IntoIter::parse_rflags(value)?,
				InstructionInfoKeys::RFLAGS_WRITTEN => tc.rflags_written |= IntoIter::parse_rflags(value)?,
				InstructionInfoKeys::RFLAGS_CLEARED => tc.rflags_cleared |= IntoIter::parse_rflags(value)?,
				InstructionInfoKeys::RFLAGS_SET => tc.rflags_set |= IntoIter::parse_rflags(value)?,
				InstructionInfoKeys::FLOW_CONTROL => tc.flow_control = to_flow_control(value)?,
				InstructionInfoKeys::OP0_ACCESS => tc.op0_access = self.to_op_access(value)?,
				InstructionInfoKeys::OP1_ACCESS => tc.op1_access = self.to_op_access(value)?,
				InstructionInfoKeys::OP2_ACCESS => tc.op2_access = self.to_op_access(value)?,
				InstructionInfoKeys::OP3_ACCESS => tc.op3_access = self.to_op_access(value)?,
				InstructionInfoKeys::OP4_ACCESS => tc.op4_access = self.to_op_access(value)?,
				InstructionInfoKeys::READ_REGISTER => self.add_registers(value, OpAccess::Read, &mut tc)?,
				InstructionInfoKeys::COND_READ_REGISTER => self.add_registers(value, OpAccess::CondRead, &mut tc)?,
				InstructionInfoKeys::WRITE_REGISTER => self.add_registers(value, OpAccess::Write, &mut tc)?,
				InstructionInfoKeys::COND_WRITE_REGISTER => self.add_registers(value, OpAccess::CondWrite, &mut tc)?,
				InstructionInfoKeys::READ_WRITE_REGISTER => self.add_registers(value, OpAccess::ReadWrite, &mut tc)?,
				InstructionInfoKeys::READ_COND_WRITE_REGISTER => self.add_registers(value, OpAccess::ReadCondWrite, &mut tc)?,
				InstructionInfoKeys::READ_MEMORY => self.add_memory(value, OpAccess::Read, &mut tc)?,
				InstructionInfoKeys::COND_READ_MEMORY => self.add_memory(value, OpAccess::CondRead, &mut tc)?,
				InstructionInfoKeys::READ_WRITE_MEMORY => self.add_memory(value, OpAccess::ReadWrite, &mut tc)?,
				InstructionInfoKeys::READ_COND_WRITE_MEMORY => self.add_memory(value, OpAccess::ReadCondWrite, &mut tc)?,
				InstructionInfoKeys::WRITE_MEMORY => self.add_memory(value, OpAccess::Write, &mut tc)?,
				InstructionInfoKeys::COND_WRITE_MEMORY => self.add_memory(value, OpAccess::CondWrite, &mut tc)?,
				InstructionInfoKeys::DECODER_OPTIONS => tc.decoder_options |= IntoIter::parse_decoder_options(value)?,
				InstructionInfoKeys::FPU_TOP_INCREMENT => {
					tc.fpu_top_increment = to_i32(value)?;
					tc.fpu_writes_top = true;
				}
				InstructionInfoKeys::FPU_CONDITIONAL_TOP => tc.fpu_conditional_top = true,
				InstructionInfoKeys::FPU_WRITES_TOP => tc.fpu_writes_top = true,
				_ => return Err(format!("Invalid key {}", key)),
			}
		}

		Ok(Some(tc))
	}

	fn parse_rflags(value: &str) -> Result<u32, String> {
		let mut rflags = 0;
		for c in value.chars() {
			match c {
				RflagsBitsConstants::AF => rflags |= RflagsBits::AF,
				RflagsBitsConstants::CF => rflags |= RflagsBits::CF,
				RflagsBitsConstants::OF => rflags |= RflagsBits::OF,
				RflagsBitsConstants::PF => rflags |= RflagsBits::PF,
				RflagsBitsConstants::SF => rflags |= RflagsBits::SF,
				RflagsBitsConstants::ZF => rflags |= RflagsBits::ZF,
				RflagsBitsConstants::IF => rflags |= RflagsBits::IF,
				RflagsBitsConstants::DF => rflags |= RflagsBits::DF,
				RflagsBitsConstants::AC => rflags |= RflagsBits::AC,
				RflagsBitsConstants::UIF => rflags |= RflagsBits::UIF,
				RflagsBitsConstants::C0 => rflags |= RflagsBits::C0,
				RflagsBitsConstants::C1 => rflags |= RflagsBits::C1,
				RflagsBitsConstants::C2 => rflags |= RflagsBits::C2,
				RflagsBitsConstants::C3 => rflags |= RflagsBits::C3,
				_ => return Err(format!("Invalid flags string: {}, char: {}", value, c)),
			}
		}
		Ok(rflags)
	}

	fn to_op_access(&self, value: &str) -> Result<OpAccess, String> {
		match self.to_access.get(value) {
			Some(access) => Ok(*access),
			None => Err(format!("Invalid op access: {}", value)),
		}
	}

	fn get_register(&self, reg_str: &str, encoding: EncodingKind, access: OpAccess) -> Result<Register, String> {
		let reg = match self.to_register.get(reg_str) {
			Some(reg) => *reg,
			None => return Err(format!("Invalid register: {}", reg_str)),
		};
		if encoding != EncodingKind::Legacy && encoding != EncodingKind::D3NOW {
			match access {
				OpAccess::None | OpAccess::Read | OpAccess::NoMemAccess | OpAccess::CondRead => {}

				OpAccess::Write | OpAccess::CondWrite | OpAccess::ReadWrite | OpAccess::ReadCondWrite => {
					if Register::XMM0 <= reg && reg <= IcedConstants::VMM_LAST && !reg_str.starts_with(MiscInstrInfoTestConstants::VMM_PREFIX) {
						return Err(format!(
							"Register {} is written ({:?}) but {} pseudo register should be used instead",
							reg_str,
							access,
							MiscInstrInfoTestConstants::VMM_PREFIX
						));
					}
				}
			}
		}

		Ok(reg)
	}

	fn add_registers(&self, value: &str, access: OpAccess, tc: &mut InstrInfoTestCase) -> Result<(), String> {
		for reg_str in value.split(';') {
			let reg_str = reg_str.trim();
			if reg_str.contains('-') {
				let reg_parts: Vec<_> = reg_str.splitn(2, '-').collect();
				let first_reg = self.get_register(reg_parts[0].trim(), tc.encoding, access)?;
				let last_reg = self.get_register(reg_parts[1].trim(), tc.encoding, access)?;
				if last_reg < first_reg {
					return Err(format!("Invalid register range: {}", reg_str));
				}
				for index in 0..(last_reg as u32) - (first_reg as u32) + 1 {
					let reg = first_reg + index;
					tc.used_registers.push(UsedRegister::new(reg, access));
				}
			} else {
				let reg = self.get_register(reg_str, tc.encoding, access)?;
				tc.used_registers.push(UsedRegister::new(reg, access));
			}
		}
		Ok(())
	}

	fn add_memory(&self, value: &str, access: OpAccess, tc: &mut InstrInfoTestCase) -> Result<(), String> {
		if !self.add_memory_core(value, access, tc)? {
			Err(format!("Invalid memory value: '{}'", value))
		} else {
			Ok(())
		}
	}

	fn add_memory_core(&self, value: &str, access: OpAccess, tc: &mut InstrInfoTestCase) -> Result<bool, String> {
		let elems: Vec<_> = value.split(';').collect();
		if elems.len() != 2 {
			return Ok(false);
		}
		let expr = elems[0];
		let memory_size = to_memory_size(elems[1])?;
		match self.try_parse_mem_expr(expr) {
			None => Ok(false),
			Some((segment, base, index, scale, mut displ, address_size, vsib_size)) => {
				match address_size {
					CodeSize::Code16 => {
						if !(i16::MIN as i64 <= displ as i64 && displ as i64 <= i16::MAX as i64) && displ > u16::MAX as u64 {
							return Ok(false);
						}
						displ = displ as u16 as u64;
					}
					CodeSize::Code32 => {
						if !(i32::MIN as i64 <= displ as i64 && displ as i64 <= i32::MAX as i64) && displ > u32::MAX as u64 {
							return Ok(false);
						}
						displ = displ as u32 as u64;
					}
					CodeSize::Code64 => {}
					_ => unreachable!(),
				}
				if access != OpAccess::NoMemAccess {
					tc.used_memory.push(UsedMemory::new2(segment, base, index, scale, displ, memory_size, access, address_size, vsib_size));
				}
				Ok(true)
			}
		}
	}

	fn try_parse_mem_expr(&self, expr: &str) -> Option<(Register, Register, Register, u32, u64, CodeSize, u32)> {
		let mut segment = Register::None;
		let mut base = Register::None;
		let mut index = Register::None;
		let mut scale = 1;
		let mut displ = 0;
		let mut address_size = CodeSize::Unknown;
		let mut vsib_size = 0;

		let mem_args: Vec<_> = expr.split('|').collect();
		for &option in mem_args.iter().skip(1) {
			match option {
				MiscInstrInfoTestConstants::MEM_SIZE_OPTION_ADDR16 => address_size = CodeSize::Code16,
				MiscInstrInfoTestConstants::MEM_SIZE_OPTION_ADDR32 => address_size = CodeSize::Code32,
				MiscInstrInfoTestConstants::MEM_SIZE_OPTION_ADDR64 => address_size = CodeSize::Code64,
				MiscInstrInfoTestConstants::MEM_SIZE_OPTION_VSIB32 => vsib_size = 4,
				MiscInstrInfoTestConstants::MEM_SIZE_OPTION_VSIB64 => vsib_size = 8,
				_ => return None,
			}
		}

		let mut has_base = false;
		for s in mem_args[0].split('+') {
			let mut s = s;
			let mut is_index = has_base;

			if let Some(index) = s.find(':') {
				let info = s.split_at(index);
				s = &info.1[1..];
				segment = *self.to_register.get(info.0)?;
				if !(Register::ES <= segment && segment <= Register::GS) {
					return None;
				}
			}
			if s.find('*').is_some() {
				scale = if s.ends_with("*1") {
					1
				} else if s.ends_with("*2") {
					2
				} else if s.ends_with("*4") {
					4
				} else if s.ends_with("*8") {
					8
				} else {
					return None;
				};
				s = s.split_at(s.len() - 2).0;
				is_index = true;
			}
			if let Some(reg) = self.to_register.get(s) {
				if is_index {
					index = *reg;
				} else {
					base = *reg;
					has_base = true;
				}
			} else {
				if let Ok(v) = to_u64(s) {
					displ = v;
				} else {
					return None;
				}
			}
		}

		if address_size == CodeSize::Unknown {
			let reg = if base != Register::None { base } else { index };
			if reg.is_gpr16() {
				address_size = CodeSize::Code16
			} else if reg.is_gpr32() {
				address_size = CodeSize::Code32
			} else if reg.is_gpr64() {
				address_size = CodeSize::Code64
			}
		}
		if address_size == CodeSize::Unknown {
			match self.bitness {
				16 => address_size = CodeSize::Code16,
				32 => address_size = CodeSize::Code32,
				64 => address_size = CodeSize::Code64,
				_ => unreachable!(),
			}
		}
		if vsib_size == 0 && base.is_vector_register() {
			return None;
		}
		if vsib_size != 0 && !index.is_vector_register() {
			return None;
		}

		if segment == Register::None {
			None
		} else {
			Some((segment, base, index, scale, displ, address_size, vsib_size))
		}
	}

	fn parse_decoder_options(value: &str) -> Result<u32, String> {
		let mut decoder_options = 0;
		for option_str in value.split(';') {
			match to_decoder_options(option_str) {
				Ok(value) => decoder_options |= value,
				Err(_) => return Err(format!("Invalid decoder options: {}", option_str)),
			}
		}
		Ok(decoder_options)
	}
}
