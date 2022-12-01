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

use clap::Parser;
use std::fs::write;
use std::io::{self, ErrorKind, Write};
use std::path::Path;
use std::process::Command;
use std::str;

mod disasm;

use disasm::Disasm;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(
        value_parser = path_parse,
        value_name = "OBJ-FILE",
        help="Disassemble <OBJ-FILE>"
    )]
    path_obj_file: String,
    #[arg(
        short='e',
        long = "executable",
        value_name = "FILE",
        value_parser = path_parse,
        help="Use the objdump executable <FILE>"
    )]
    path_objdump: Option<String>,
    #[arg(
        short = 'o',
        long = "out",
        value_name = "FILE",
        help = "Place the output into <FILE>"
    )]
    path_out_file: Option<String>,
}

fn path_parse(path: &str) -> Result<String, String> {
    if Path::new(path).is_file() {
        Ok(path.to_string())
    } else {
        Err(format!("File does not exist!"))
    }
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let objdump = cli.path_objdump.unwrap_or("objdump".to_string());

    let objdump_res = Command::new(&objdump)
        .args([
            "-d",
            "--no-addresses",
            "--no-show-raw-insn",
            &cli.path_obj_file,
        ])
        .output()
        .map_err(|e| match e.kind() {
            ErrorKind::NotFound => {
                "'objdump' was not found! Check your PATH or explicitly provide an executable"
                    .to_string()
            }
            _ => e.to_string(),
        })?;

    let stderr = str::from_utf8(&objdump_res.stderr).map_err(|msg| msg.to_string())?;
    if !stderr.is_empty() {
        return Err(stderr.to_string());
    }

    let stdout = str::from_utf8(&objdump_res.stdout)
        .map_err(|msg| msg.to_string())?
        .to_string();

    let disasm = Disasm::try_from(stdout)?.to_string();

    match cli.path_out_file {
        Some(file) => write(file, disasm).map_err(|msg| msg.to_string()),
        None => io::stdout()
            .write_all(disasm.as_bytes())
            .map_err(|msg| msg.to_string()),
    }
}
