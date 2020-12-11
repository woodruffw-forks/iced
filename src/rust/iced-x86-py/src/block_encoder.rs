/*
Copyright (C) 2018-2019 de4dot@gmail.com

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use crate::instruction::Instruction;
use crate::utils::to_value_error;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

/// Encodes instructions
///
/// :class:`Encoder` can only encode one instruction at a time. This class can encode any number of
/// instructions and can also fix short branches if the target is too far away.
///
/// It will fail if there's an instruction with a RIP-relative operand (``[rip+123h]``) and the target is too far away.
/// A workaround is to use a new base RIP of the encoded instructions that is close (+/-2GB) to the original location.
///
/// Args:
///     bitness (int): 16, 32 or 64
///     fix_branches (bool): (default = `True`) Fix branches (eg. convert short to near branches if the target is too far away)
///
/// Raises:
///     ValueError: If `bitness` is invalid
///
/// Examples:
///
/// .. testcode::
///
///     from iced_x86 import *
///
///     data = b"\x86\x64\x32\x16\xF0\xF2\x83\x00\x5A\x62\xC1\xFE\xCB\x6F\xD3"
///     decoder = Decoder(64, data)
///     decoder.ip = 0x1234_5678
///
///     instrs = [instr for instr in decoder]
///
///     encoder = BlockEncoder(64)
///     # Add an instruction
///     encoder.add(instrs[0])
///     # Add more instructions
///     encoder.add_many(instrs[1:])
///     try:
///         # Encode all added instructions and get the raw bytes
///         raw_data = encoder.encode(0x3456_789A)
///     except ValueError as ex:
///         print("Could not encode all instructions")
///         raise
///
///     # It has no IP-relative instructions (eg. branches or [rip+xxx] ops)
///     # so the result should be identical to the original code.
///     assert data == raw_data
#[pyclass(module = "_iced_x86_py")]
#[text_signature = "(bitness, fix_branches, /)"]
pub struct BlockEncoder {
	instructions: Vec<iced_x86::Instruction>,
	bitness: u32,
	options: u32,
}

#[pymethods]
impl BlockEncoder {
	#[new]
	#[args(fix_branches = true)]
	fn new(bitness: u32, fix_branches: bool) -> PyResult<BlockEncoder> {
		let options = if fix_branches { iced_x86::BlockEncoderOptions::NONE } else { iced_x86::BlockEncoderOptions::DONT_FIX_BRANCHES };
		match bitness {
			16 | 32 | 64 => Ok(BlockEncoder { instructions: Vec::new(), bitness, options }),
			_ => Err(PyValueError::new_err("bitness must be 16, 32 or 64")),
		}
	}

	/// Adds an instruction that will be encoded when :class:`BlockEncoder.encode` is called.
	///
	/// The input `instruction` can be a decoded instruction or an instruction
	/// created by the user, eg. `Instruction.create*()` methods.
	///
	/// Args:
	///     `instruction` (Instruction): Next instruction to encode
	#[text_signature = "($self, instruction, /)"]
	fn add(&mut self, instruction: &Instruction) {
		self.instructions.push(instruction.instr);
	}

	/// Adds instructions that will be encoded when :class:`BlockEncoder.encode` is called.
	///
	/// Args:
	///     `instructions` (List[Instruction]): Next instructions to encode
	#[text_signature = "($self, instructions, /)"]
	fn add_many(&mut self, instructions: Vec<Instruction>) {
		self.instructions.extend(instructions.iter().map(|i| i.instr));
	}

	/// Encodes all instructions added by :class:`BlockEncoder.add`/:class:`BlockEncoder.add_many` and returns the raw bytes
	///
	/// Args:
	///     `rip` (int): (``u64``) Base IP of all encoded instructions
	///
	/// Returns:
	///     bytes: All encoded instructions
	///
	/// Raises:
	///     ValueError: If one or more instructions couldn't be encoded
	#[text_signature = "($self, rip, /)"]
	fn encode<'py>(&mut self, py: Python<'py>, rip: u64) -> PyResult<&'py PyBytes> {
		let block = iced_x86::InstructionBlock::new(&self.instructions, rip);
		match iced_x86::BlockEncoder::encode(self.bitness, block, self.options) {
			Ok(result) => Ok(PyBytes::new(py, &result.code_buffer)),
			Err(error) => Err(to_value_error(error)),
		}
	}
}
