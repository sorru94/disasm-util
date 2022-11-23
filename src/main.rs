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

use std::env;
use std::fs::{write, File};
use std::io::BufReader;
use std::path::Path;
use std::process::exit;

mod disasm;

use disasm::Disasm;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        None => {
            println!("Specify the file to parse using \"cargo run -- <file_name>\"!");
            exit(1);
        }
        Some(input_fp) if Path::new(input_fp).exists() => {
            let fp = File::open(input_fp).expect("Error opening the file");

            let disasm = Disasm::try_from(BufReader::new(fp)).unwrap();

            let output_fp = format!("{input_fp}-parsed");
            write(output_fp, disasm.to_string()).expect("Error writing the file");
        }
        Some(_) => {
            println!("The specified input file does not exist!");
            exit(1);
        }
    }
}
