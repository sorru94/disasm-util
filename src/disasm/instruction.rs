//! Access to the Instruction struct.
//!
//! This module contains the Instruction struct which can parse a string containing an instruction and store in
//! its components.
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    opcode: String,
    _operands: String,
    _comment: String,
}

impl From<&str> for Instruction {
    fn from(string: &str) -> Self {
        //Note that the string should follow the specific encoding:
        // opcode(s) [operand(s)] [# comment]
        // Operands and comments are optional.
        // Multiple operands should not be separated by spaces.
        let (leftover_string, _comment) = string.split_once('#').unwrap_or((string, ""));

        let (opcode, _operands) = leftover_string
            .trim()
            .rsplit_once(' ')
            .unwrap_or((leftover_string, ""));

        Instruction {
            opcode: opcode.trim().to_string(),
            _operands: _operands.to_string(),
            _comment: _comment.trim().to_string(),
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
    fn from_str_empty_string() {
        let instruction = Instruction::from("");
        assert_eq!(
            instruction,
            Instruction {
                opcode: "".to_string(),
                _operands: "".to_string(),
                _comment: "".to_string()
            }
        )
    }
    #[test]
    fn from_str_instruction_with_no_operands_no_comment() {
        let instruction = Instruction::from("	endbr64 ");
        assert_eq!(
            instruction,
            Instruction {
                opcode: "endbr64".to_string(),
                _operands: "".to_string(),
                _comment: "".to_string()
            }
        )
    }
    #[test]
    fn from_str_instruction_with_operands_no_comment() {
        let instruction = Instruction::from("	mov    -0x1198(%rbp),%rax    ");
        assert_eq!(
            instruction,
            Instruction {
                opcode: "mov".to_string(),
                _operands: "-0x1198(%rbp),%rax".to_string(),
                _comment: "".to_string()
            }
        )
    }
    #[test]
    fn from_str_instruction_with_operands_and_comment() {
        let instruction =
            Instruction::from("	lea    0x357d6(%rip),%rcx        # <_IO_stdin_used+0x24f8>  ");
        assert_eq!(
            instruction,
            Instruction {
                opcode: "lea".to_string(),
                _operands: "0x357d6(%rip),%rcx".to_string(),
                _comment: "<_IO_stdin_used+0x24f8>".to_string()
            }
        )
    }
    #[test]
    fn from_str_double_instruction_with_operands_and_no_comment() {
        let instruction = Instruction::from("	bnd jmp <_init+0x20>");
        assert_eq!(
            instruction,
            Instruction {
                opcode: "bnd jmp".to_string(),
                _operands: "<_init+0x20>".to_string(),
                _comment: "".to_string()
            }
        )
    }
    #[test]
    fn to_string_complete_instruction() {
        let instruction =
            Instruction::from("	lea    0x357d6(%rip),%rcx        # <_IO_stdin_used+0x24f8>  ");
        assert_eq!(instruction.to_string(), "lea\n".to_string())
    }
    #[test]
    fn to_string_double_instruction() {
        let instruction = Instruction::from("	bnd jmp <_init+0x20>");
        assert_eq!(instruction.to_string(), "bnd jmp\n".to_string())
    }
}
