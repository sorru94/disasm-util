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

//! Access to the Disasm struct.
//!
//! This module contains the Disasm struct which can be used to parse the output file of a objdump command.
//! This file operates over files generated with the following combination of flags:
//! objdump -d --no-addresses --no-show-raw-insn
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod instruction;
mod section;
mod symbol;

use instruction::Instruction;
use section::Section;
use symbol::Symbol;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub struct Disasm {
    file_name: String,
    file_format: String,
    sections: Vec<Section>,
}

impl Disasm {
    fn from_lines(lines: Vec<String>) -> Result<Self, String> {
        let mut disasm = Disasm {
            file_name: String::from(""),
            file_format: String::from(""),
            sections: Vec::new(),
        };
        // Filter out empty lines
        let mut lines_iter = lines.into_iter().filter(|line| !line.trim().is_empty());
        // Process the first line
        let line = lines_iter
            .next()
            .ok_or("Error, the file does not contain any text".to_string())?;
        disasm.process_first_line(line)?;
        // Process all other lines
        for line in lines_iter {
            disasm.process_other_line(line)?;
        }
        // Sort the stored data
        disasm.sort_sections();
        Ok(disasm)
    }

    fn process_first_line(&mut self, line: String) -> Result<(), String> {
        let err_msg = "Incorrect format for the first line";

        let (file_name, leftover_line) = line.split_once(':').ok_or(err_msg.to_string())?;
        self.file_name = file_name.to_string();
        self.file_format = leftover_line
            .trim()
            .strip_prefix("file format ")
            .ok_or(err_msg.to_string())?
            .to_string();
        Ok(())
    }

    fn process_other_line(&mut self, line: String) -> Result<(), String> {
        lazy_static! {
            static ref RE_SECTION: Regex =
                Regex::new(r"^Disassembly of section (?P<sec_name>.[[:alnum:].]+):$").unwrap();
            static ref RE_SYMBOL: Regex = Regex::new(r"^(?P<sym_name><.+>):$").unwrap();
            static ref RE_INSTRUCTION: Regex = Regex::new(
                r"(?x)^
                    [[:space:]]
                    (?P<opcode>  [[[:lower:]][[:digit:]][[:space:]]]*)
                    (?P<operands>[[:space:]]+[^[[:space:]]]+)??
                    ([[:space:]]+\#(?P<comment>.*))??
                    [[:space:]]*
                    $"
            )
            .unwrap();
        }

        if let Some(sec_name) = RE_SECTION
            .captures(&line)
            .and_then(|cap| cap.name("sec_name").map(|sec| sec.as_str()))
        {
            self.add_section(Section::new(sec_name));
            Ok(())
        } else if let Some(sym_name) = RE_SYMBOL
            .captures(&line)
            .and_then(|cap| cap.name("sym_name").map(|sym| sym.as_str()))
        {
            self.add_symbol(Symbol::new(sym_name))
        } else if let Some(ins_cap) = RE_INSTRUCTION.captures(&line) {
            let opcode = ins_cap.name("opcode").map_or("", |m| m.as_str()).trim();
            let operands = ins_cap.name("operands").map_or("", |m| m.as_str()).trim();
            let comment = ins_cap.name("comment").map_or("", |m| m.as_str()).trim();
            self.add_instruction(Instruction::new(opcode, operands, comment))
        } else {
            Err(format!(
                "Unrecognized format for the following line: '{line}'"
            ))
        }
    }

    fn add_section(&mut self, section: Section) {
        self.sections.push(section);
    }

    fn add_symbol(&mut self, symbol: Symbol) -> Result<(), String> {
        self.sections
            .last_mut()
            .ok_or("Attempted to add a symbol without first defining a section")?
            .add_symbol(symbol);
        Ok(())
    }

    fn add_instruction(&mut self, instruction: Instruction) -> Result<(), String> {
        self.sections
            .last_mut()
            .ok_or("Attempted to add an instruction without first defining a section")?
            .add_instruction(instruction)
    }

    fn sort_sections(&mut self) {
        for section in &mut self.sections {
            section.sort_symbols();
        }
        self.sections.sort_by(|a, b| a.get_name().cmp(b.get_name()));
    }
}

impl fmt::Display for Disasm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let joined = self
            .sections
            .iter()
            .map(|sec| sec.to_string())
            .collect::<Vec<_>>()
            .join("");
        write!(f, "{}", joined)
    }
}

impl TryFrom<BufReader<File>> for Disasm {
    type Error = String;

    fn try_from(buffer: BufReader<File>) -> Result<Self, Self::Error> {
        let lines = buffer
            .lines()
            .collect::<Result<Vec<_>, _>>()
            .map_err(|msg| format!("Error reading a line of the disassembly file :{msg}"))?;
        Disasm::from_lines(lines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn from_lines_empty_vector_fails() {
        let result = Disasm::from_lines(Vec::new());
        assert_eq!(
            result,
            Err("Error, the file does not contain any text".to_string())
        );
    }

    #[test]
    fn from_lines_incorrectly_formatted_first_line_fails() {
        let lines = Vec::from(["New line with incorrect formatting".to_string()]);
        let result = Disasm::from_lines(lines);
        assert_eq!(
            result,
            Err("Incorrect format for the first line".to_string())
        );
    }

    #[test]
    fn from_lines_only_first_line_ok() {
        let lines = Vec::from([r"folder\file:     file format some_format   ".to_string()]);
        let result = Disasm::from_lines(lines);
        assert_eq!(
            result,
            Ok(Disasm {
                file_name: r"folder\file".to_string(),
                file_format: "some_format".to_string(),
                sections: Vec::new(),
            })
        );
    }

    #[test]
    fn from_lines_simple_corect_file_ok() {
        let lines: Vec<String> = Vec::from([
            "  ",
            " ",
            "",
            "folder\\file:     file format some_format   ",
            "",
            " ",
            "Disassembly of section sec1:",
            "",
            "",
            "",
            "<sym1>:",
            "	opc1 ",
            "	opc2    %opr1,%opr2",
            "	opc3    %opr3                   # comment1",
            "",
            "<sym2>:",
            "    opc4   %opr4  # comment2 ",
            " ",
            "Disassembly of section sec2:",
            "",
            "<sym3>:",
            "",
        ])
        .iter()
        .map(|l| l.to_string())
        .collect();

        let result = Disasm::from_lines(lines);

        let mut sec1 = Section::new("sec1");
        sec1.add_symbol(Symbol::new("<sym1>"));
        let _ = sec1.add_instruction(Instruction::new("opc1", "", ""));
        let _ = sec1.add_instruction(Instruction::new("opc2", "%opr1,%opr2", ""));
        let _ = sec1.add_instruction(Instruction::new("opc3", "%opr3", "comment1"));
        sec1.add_symbol(Symbol::new("<sym2>"));
        let _ = sec1.add_instruction(Instruction::new("opc4", "%opr4", "comment2"));
        let mut sec2 = Section::new("sec2");
        sec2.add_symbol(Symbol::new("<sym3>"));

        assert_eq!(
            result,
            Ok(Disasm {
                file_name: "folder\\file".to_string(),
                file_format: "some_format".to_string(),
                sections: Vec::from([sec1, sec2]),
            })
        );
    }

    #[test]
    fn from_lines_instruction_before_section_fails() {
        let lines: Vec<String> =
            Vec::from(["folder\\file:     file format some_format   ", "	opc1 "])
                .iter()
                .map(|l| l.to_string())
                .collect();

        let result = Disasm::from_lines(lines);

        assert_eq!(
            result,
            Err("Attempted to add an instruction without first defining a section".to_string())
        );
    }

    #[test]
    fn from_lines_symbol_before_section_fails() {
        let lines: Vec<String> =
            Vec::from(["folder\\file:     file format some_format   ", "<sym1>:"])
                .iter()
                .map(|l| l.to_string())
                .collect();

        let result = Disasm::from_lines(lines);

        assert_eq!(
            result,
            Err("Attempted to add a symbol without first defining a section".to_string())
        );
    }

    #[test]
    fn from_lines_incorrectly_formatted_section_name_start_fails() {
        let lines: Vec<String> = Vec::from([
            "folder\\file:     file format some_format   ",
            "gibberish of section sec1:",
        ])
        .iter()
        .map(|l| l.to_string())
        .collect();

        let result = Disasm::from_lines(lines);

        assert_eq!(
            result,
            Err(
                "Unrecognized format for the following line: 'gibberish of section sec1:'"
                    .to_string()
            )
        );
    }

    #[test]
    fn from_lines_incorrectly_formatted_section_name_end_fails() {
        let lines: Vec<String> = Vec::from([
            "folder\\file:     file format some_format   ",
            "Disassembly of section sec1:gibberish",
        ])
        .iter()
        .map(|l| l.to_string())
        .collect();

        let result = Disasm::from_lines(lines);

        assert_eq!(
            result,
            Err("Unrecognized format for the following line: 'Disassembly of section sec1:gibberish'".to_string())
        );
    }

    #[test]
    fn from_lines_incorrectly_formatted_symbol_name_start_fails() {
        let lines: Vec<String> = Vec::from([
            "folder\\file:     file format some_format   ",
            "Disassembly of section sec1:",
            "gibberish<sym1>:",
        ])
        .iter()
        .map(|l| l.to_string())
        .collect();

        let result = Disasm::from_lines(lines);

        assert_eq!(
            result,
            Err("Unrecognized format for the following line: 'gibberish<sym1>:'".to_string())
        );
    }

    #[test]
    fn from_lines_incorrectly_formatted_symbol_name_end_fails() {
        let lines: Vec<String> = Vec::from([
            "folder\\file:     file format some_format   ",
            "Disassembly of section sec1:",
            "<sym1>:gibberish",
        ])
        .iter()
        .map(|l| l.to_string())
        .collect();

        let result = Disasm::from_lines(lines);

        assert_eq!(
            result,
            Err("Unrecognized format for the following line: '<sym1>:gibberish'".to_string())
        );
    }

    #[test]
    fn from_lines_incorrectly_formatted_instruction_missing_leading_space_fails() {
        let lines: Vec<String> = Vec::from([
            "folder\\file:     file format some_format   ",
            "Disassembly of section sec1:",
            "<sym1>:",
            "nop",
        ])
        .iter()
        .map(|l| l.to_string())
        .collect();

        let result = Disasm::from_lines(lines);

        assert_eq!(
            result,
            Err("Unrecognized format for the following line: 'nop'".to_string())
        );
    }

    #[test]
    fn from_lines_incorrectly_formatted_instruction_bad_opcode_fails() {
        let lines: Vec<String> = Vec::from([
            "folder\\file:     file format some_format   ",
            "Disassembly of section sec1:",
            "<sym1>:",
            " n%op",
        ])
        .iter()
        .map(|l| l.to_string())
        .collect();

        let result = Disasm::from_lines(lines);

        assert_eq!(
            result,
            Err("Unrecognized format for the following line: ' n%op'".to_string())
        );
    }

    #[test]
    fn from_lines_unsorted_sections_ok() {
        let lines: Vec<String> = Vec::from([
            "folder\\file:     file format some_format   ",
            "Disassembly of section abb:",
            "<zsym1>:",
            "<asym2>:",
            "<bsym4>:",
            "<bsym3>:",
            "Disassembly of section aaa:",
            "<sym3>:",
            "<sym1>:",
            "Disassembly of section adc:",
            "Disassembly of section abc:",
            "Disassembly of section acc:",
        ])
        .iter()
        .map(|l| l.to_string())
        .collect();

        let result = Disasm::from_lines(lines);

        let mut sec1 = Section::new("aaa");
        sec1.add_symbol(Symbol::new("<sym1>"));
        sec1.add_symbol(Symbol::new("<sym3>"));
        let mut sec2 = Section::new("abb");
        sec2.add_symbol(Symbol::new("<asym2>"));
        sec2.add_symbol(Symbol::new("<bsym3>"));
        sec2.add_symbol(Symbol::new("<bsym4>"));
        sec2.add_symbol(Symbol::new("<zsym1>"));
        let sec3 = Section::new("abc");
        let sec4 = Section::new("acc");
        let sec5 = Section::new("adc");

        assert_eq!(
            result,
            Ok(Disasm {
                file_name: "folder\\file".to_string(),
                file_format: "some_format".to_string(),
                sections: Vec::from([sec1, sec2, sec3, sec4, sec5]),
            })
        )
    }

    #[test]
    fn to_string_ok() {
        let mut sec1 = Section::new("aaa");
        sec1.add_symbol(Symbol::new("<sym1>"));
        sec1.add_symbol(Symbol::new("<sym3>"));
        let _ = sec1.add_instruction(Instruction::new("opc5", "", ""));
        let _ = sec1.add_instruction(Instruction::new("opc3", "", ""));
        let sec2 = Section::new("abc");
        let mut sec3 = Section::new("abb");
        sec3.add_symbol(Symbol::new("<zsym2>"));
        let _ = sec3.add_instruction(Instruction::new("opc1", "", ""));
        let _ = sec3.add_instruction(Instruction::new("opc2", "opr1,opr2", ""));
        let _ = sec3.add_instruction(Instruction::new("opc4", "opr3", "comment1"));
        sec3.add_symbol(Symbol::new("<asym1>"));
        let disasm = Disasm {
            file_name: "folder\\file".to_string(),
            file_format: "some_format".to_string(),
            sections: Vec::from([sec1, sec2, sec3]),
        };

        assert_eq!(
            disasm.to_string(),
            indoc! {"
                aaa:
                    <sym1>:
                    <sym3>:
                        opc5
                        opc3
                abc:
                abb:
                    <zsym2>:
                        opc1
                        opc2
                        opc4
                    <asym1>:
            "}
            .to_string()
        )
    }
}
