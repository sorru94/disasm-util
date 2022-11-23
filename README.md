<!---
  Copyright 2022 SECO Mind Srl
  SPDX-License-Identifier: Apache-2.0
-->

# disasm-util

`disasm-util` is a simple command line utility to parse the output of the `objdump` tool, part of the GNU Binary Utilities.

## Supported inputs

Support is only provided for a specific configuration of `objdump`, the tool should be run as for the following:
```
objdump -d --no-addresses --no-show-raw-insn
```
The required flags are:
- `-d`, `-disassemble`: print assembler mnemonics for the machine instructions.
- `--no-addresses`: when disassembling do not print addresses.
- `--no-show-raw-insn`: when disassembling do not print instruction's bytes.

See the [GNU Binutils](https://sourceware.org/binutils/docs-2.39/binutils/index.html) documentation for more
information.

## Usage

The utility is provided as a rust binary crate.
Building and executing the tool can be achieved by running the following from terminal:
```
cargo run -- <objdump_out_file>
```
This command requires `rust` to be installed on your system. See the [rust documentation](https://doc.rust-lang.org/book/) for more information.

## Parsed output

The input file is parsed and saved to a new file with the same name as the imput file plus the `-parsed` keyword
appended at the end.
The parsed output is in the following format:
```
section 1 name:
    symbol 1 name:
        opcode instruction 1
    symbol 2 name:
        opcode instruction 2
        opcode instruction 3
        opcode instruction 4
section 2 name:
    symbol 3 name:
        opcode instruction 5
        opcode instruction 6
```
Sections and symbols are alphabetically sorted.
