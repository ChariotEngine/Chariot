// Chariot: An open source reimplementation of Age of Empires (1997)
// Copyright (c) 2016 Kevin Fuller
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

extern crate chariot_drs as drs;

use std::path::Path;
use std::path::PathBuf;
use std::io::prelude::*;
use std::env;
use std::fs;

fn extract<P: AsRef<Path>>(file_name: P, drs_file: drs::DrsFile) {
    let drs_name = file_name.as_ref().file_stem().unwrap().clone();
    println!("Successfully loaded {:?}...", drs_name);

    for table in &drs_file.tables {
        println!("Extracting table \"{}\"...", table.header.file_extension());

        let mut root_path = PathBuf::new();
        root_path.push(drs_name);
        root_path.push(table.header.file_extension());
        fs::create_dir_all(&root_path).expect("Failed to create directory");

        for i in 0..table.entries.len() {
            let mut file_name = root_path.clone();
            file_name.push(format!("{}.{}", table.entries[i].file_id, table.header.file_extension()));
            println!("  Extracting {:?}...", file_name);

            let mut file = fs::File::create(&file_name).expect("Failed to open file");
            file.write_all(&table.contents[i][..]).expect("Failed to write file");
        }
    }
}

fn main() {
    let file_name = env::args().skip(1).next().expect("usage: extract-drs file-name");
    match drs::DrsFile::read_from_file(&file_name) {
        Ok(drs_file) => {
            extract(&file_name, drs_file);
        },
        Err(err) => {
            println!("Failed to read the DRS file: {}", err);
        }
    }
}
