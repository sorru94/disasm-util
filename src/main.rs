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
use std::fs::{write, File};
use std::io::BufReader;
use std::path::Path;
use std::process::exit;

mod disasm;

use disasm::Disasm;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_parser = path_parse)]
    path: String,
}

fn path_parse(path: &str) -> Result<String, String> {
    if Path::new(path).exists() {
        Ok(path.to_string())
    } else {
        Err(format!("The specified input file does not exist!"))
    }
}

fn main() {
    let cli = Cli::parse();

    let fp = File::open(&cli.path).expect("Error opening the file");

    match Disasm::try_from(BufReader::new(fp)) {
        Ok(disasm) => write(format!("{}-parsed", cli.path), disasm.to_string())
            .expect("Error writing the file"),
        Err(msg) => {
            println!("Error: {}", msg);
            exit(1)
        }
    }
}
