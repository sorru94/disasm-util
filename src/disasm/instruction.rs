/*
 * This file is part of Disasm-Util.
 *
 * Copyright 2022 SECO Mind Srl
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * SPDX-License-Identifier: Apache-2.0
 */

//! Access to the Instruction struct.
//!
//! This module contains the Instruction struct which can parse a string containing an instruction and store in
//! its components.
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    opcode: String,
    operands: String,
    comment: String,
}

impl Instruction {
    pub fn new(opcode: &str, operands: &str, comment: &str) -> Self {
        Instruction {
            opcode: opcode.to_string(),
            operands: operands.to_string(),
            comment: comment.to_string(),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n", self.opcode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty_instruction_ok() {
        let instruction = Instruction::new("", "", "");
        assert_eq!(
            instruction,
            Instruction {
                opcode: "".to_string(),
                operands: "".to_string(),
                comment: "".to_string()
            }
        )
    }

    #[test]
    fn new_full_instruction_ok() {
        let instruction =
            Instruction::new("my opcode", "operand1, operand 2", "some kind of comment");
        assert_eq!(
            instruction,
            Instruction {
                opcode: "my opcode".to_string(),
                operands: "operand1, operand 2".to_string(),
                comment: "some kind of comment".to_string()
            }
        )
    }

    #[test]
    fn to_string_only_opcode_ok() {
        let instruction = Instruction::new("my opcode", "", "");
        assert_eq!(instruction.to_string(), "my opcode\n".to_string())
    }

    #[test]
    fn to_string_complete_ok() {
        let instruction = Instruction::new("opcode", "operands", "comment");
        assert_eq!(instruction.to_string(), "opcode\n".to_string())
    }
}
