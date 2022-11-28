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

//! Access to the Symbol struct.
//!
//! This module contains the Symbol struct which is a named collection of instructions.
use std::fmt;

use super::Instruction;

#[derive(Debug, PartialEq, Eq)]
pub struct Symbol {
    _addr: String,
    name: String,
    instructions: Vec<Instruction>,
}

impl Symbol {
    pub fn new(addr: &str, name: &str) -> Self {
        Symbol {
            _addr: addr.to_string(),
            name: name.to_string(),
            instructions: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let joined = self
            .instructions
            .iter()
            .map(|sec| format!("    {}", sec))
            .collect::<Vec<_>>()
            .join("");
        write!(f, "{}:\n{}", self.name, joined)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_symbol_with_no_name() {
        let symbol = Symbol::new("", "");
        assert_eq!(
            symbol,
            Symbol {
                _addr: "".to_string(),
                name: "".to_string(),
                instructions: Vec::new(),
            }
        )
    }

    #[test]
    fn new_symbol_with_name() {
        let symbol = Symbol::new("addr", "symbol name");
        assert_eq!(
            symbol,
            Symbol {
                _addr: "addr".to_string(),
                name: "symbol name".to_string(),
                instructions: Vec::new(),
            }
        )
    }

    #[test]
    fn add_instruction() {
        let mut symbol = Symbol::new("addr", "sym");
        symbol.add_instruction(Instruction::new("addr", "nop", "", ""));
        symbol.add_instruction(Instruction::new("addr", "bnd jmp", "<_init+0x20>", ""));
        assert_eq!(
            symbol,
            Symbol {
                _addr: "addr".to_string(),
                name: "sym".to_string(),
                instructions: Vec::from([
                    Instruction::new("addr", "nop", "", ""),
                    Instruction::new("addr", "bnd jmp", "<_init+0x20>", "")
                ]),
            }
        )
    }

    #[test]
    fn get_name_with_no_name() {
        let symbol = Symbol::new("", "");
        assert_eq!(symbol.get_name(), "")
    }

    #[test]
    fn get_name_with_name() {
        let symbol = Symbol::new("addr", "symbol name ");
        assert_eq!(symbol.get_name(), "symbol name ")
    }

    #[test]
    fn to_string_unnamed_empty_symbol() {
        let symbol = Symbol::new("addr", "");
        assert_eq!(symbol.to_string(), ":\n".to_string())
    }

    #[test]
    fn to_string_named_empty_symbol() {
        let symbol = Symbol::new("addr", "sym");
        assert_eq!(symbol.to_string(), "sym:\n".to_string())
    }

    #[test]
    fn to_string_named_and_non_empty_symbol() {
        let mut symbol = Symbol::new("addr", "sym");
        symbol.add_instruction(Instruction::new("1", "nop", "", ""));
        symbol.add_instruction(Instruction::new("1", "bnd jmp", "<_init+0x20>", ""));
        assert_eq!(
            symbol.to_string(),
            "\
sym:
    nop
    bnd jmp
"
            .to_string()
        )
    }
}
