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

//! Access to the Section struct.
//!
//! This module contains the Section struct which is a named collection of symbols.
use std::fmt;

use super::Instruction;
use super::Symbol;

#[derive(Debug, PartialEq, Eq)]
pub struct Section {
    name: String,
    symbols: Vec<Symbol>,
}

impl Section {
    pub fn new(name: &str) -> Self {
        Section {
            name: name.to_string(),
            symbols: Vec::new(),
        }
    }

    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.symbols.push(symbol);
    }

    pub fn add_instruction(&mut self, instruction: Instruction) -> Result<(), String> {
        self.symbols
            .last_mut()
            .ok_or("Attempted to add an instruction without first defining a symbol")?
            .add_instruction(instruction);
        Ok(())
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn sort_symbols(&mut self) {
        self.symbols.sort_by(|a, b| a.get_name().cmp(b.get_name()));
    }
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Stringify all symbols and concatenate them
        let symbols_str = self
            .symbols
            .iter()
            .fold("".to_string(), |acc, x| acc + &x.to_string());
        // Add fours spaces before each line
        let symbols_str = symbols_str.split('\n').fold("".to_string(), |acc, x| {
            acc + if !x.is_empty() { "    " } else { "" }
                + x
                + if !x.is_empty() { "\n" } else { "" }
        });
        write!(f, "{}:\n{}", self.name, symbols_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn new_section_with_no_name_ok() {
        let section = Section::new("");
        assert_eq!(
            section,
            Section {
                name: "".to_string(),
                symbols: Vec::new(),
            }
        )
    }

    #[test]
    fn new_section_with_name_ok() {
        let section = Section::new("Section name");
        assert_eq!(
            section,
            Section {
                name: "Section name".to_string(),
                symbols: Vec::new(),
            }
        )
    }

    #[test]
    fn add_symbol_ok() {
        let mut section = Section::new("sec");
        section.add_symbol(Symbol::new("sym1"));
        section.add_symbol(Symbol::new("sym2"));
        assert_eq!(
            section,
            Section {
                name: "sec".to_string(),
                symbols: Vec::from([Symbol::new("sym1"), Symbol::new("sym2")]),
            }
        )
    }

    #[test]
    fn add_instruction_single_symbol_single_instruction_fails() {
        let mut section = Section::new("sec");
        let add_instr_res = section.add_instruction(Instruction::new("", "", ""));
        assert_eq!(
            add_instr_res,
            Err("Attempted to add an instruction without first defining a symbol".to_string())
        );
    }

    #[test]
    fn add_instruction_single_symbol_multiple_instructions_ok() {
        let mut section = Section::new("sec");
        section.add_symbol(Symbol::new("sym1"));
        let _ = section.add_instruction(Instruction::new("nop", "", ""));
        let _ = section.add_instruction(Instruction::new("mov", "-0x1198(%rbp),%rax", ""));

        let mut comparison_sym = Symbol::new("sym1");
        comparison_sym.add_instruction(Instruction::new("nop", "", ""));
        comparison_sym.add_instruction(Instruction::new("mov", "-0x1198(%rbp),%rax", ""));
        assert_eq!(
            section,
            Section {
                name: "sec".to_string(),
                symbols: Vec::from([comparison_sym]),
            }
        );
    }

    #[test]
    fn add_instruction_multiple_symbols_multiple_instructions_ok() {
        let mut section = Section::new("sec");
        section.add_symbol(Symbol::new("sym1"));
        let _ = section.add_instruction(Instruction::new("nop", "", ""));
        let _ = section.add_instruction(Instruction::new("mov", "-0x1198(%rbp),%rax", ""));
        section.add_symbol(Symbol::new("sym2"));
        let _ = section.add_instruction(Instruction::new("lea", "0x357d6(%rip),%rcx", ""));

        let mut comparison_sym1 = Symbol::new("sym1");
        comparison_sym1.add_instruction(Instruction::new("nop", "", ""));
        comparison_sym1.add_instruction(Instruction::new("mov", "-0x1198(%rbp),%rax", ""));
        let mut comparison_sym2 = Symbol::new("sym2");
        comparison_sym2.add_instruction(Instruction::new("lea", "0x357d6(%rip),%rcx", ""));
        assert_eq!(
            section,
            Section {
                name: "sec".to_string(),
                symbols: Vec::from([comparison_sym1, comparison_sym2]),
            }
        );
    }

    #[test]
    fn get_name_with_no_name_ok() {
        let section = Section::new("");
        assert_eq!(section.get_name(), "")
    }

    #[test]
    fn get_name_with_name_ok() {
        let section = Section::new("symbol name");
        assert_eq!(section.get_name(), "symbol name")
    }

    #[test]
    fn sort_symbols_ok() {
        let mut section = Section::new("sec");
        section.add_symbol(Symbol::new("sym2"));
        section.add_symbol(Symbol::new("sym2"));
        section.add_symbol(Symbol::new("sym1"));
        section.add_symbol(Symbol::new("zsym"));
        section.add_symbol(Symbol::new("asym"));
        section.sort_symbols();
        assert_eq!(
            section,
            Section {
                name: "sec".to_string(),
                symbols: Vec::from([
                    Symbol::new("asym"),
                    Symbol::new("sym1"),
                    Symbol::new("sym2"),
                    Symbol::new("sym2"),
                    Symbol::new("zsym")
                ]),
            }
        );
    }

    #[test]
    fn to_string_unnamed_empty_section_ok() {
        let section = Section::new("");
        assert_eq!(section.to_string(), ":\n".to_string())
    }

    #[test]
    fn to_string_named_empty_section_ok() {
        let section = Section::new("sec");
        assert_eq!(section.to_string(), "sec:\n".to_string())
    }

    #[test]
    fn to_string_named_and_non_empty_section_ok() {
        let mut section = Section::new("sec");
        section.add_symbol(Symbol::new("sym1"));
        assert_eq!(
            section.add_instruction(Instruction::new("nop", "", "")),
            Ok(())
        );
        assert_eq!(
            section.add_instruction(Instruction::new("mov", "-0x1198(%rbp),%rax", "")),
            Ok(())
        );
        section.add_symbol(Symbol::new("sym2"));
        assert_eq!(
            section.add_instruction(Instruction::new("lea", "0x357d6(%rip),%rcx", "")),
            Ok(())
        );

        assert_eq!(
            section.to_string(),
            indoc! {"
                sec:
                    sym1:
                        nop
                        mov
                    sym2:
                        lea
            "}
            .to_string()
        )
    }
}
