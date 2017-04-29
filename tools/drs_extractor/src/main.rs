// Chariot: An open source reimplementation of Age of Empires (1997)
// Copyright (c) 2017 Taryn Hill
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
extern crate chariot_slp as slp;
extern crate chariot_palette as palette;

use std::io::Write;

extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("drs_extractor")
        .version("0.1.0")
        .author("Taryn Hill <taryn@phrohdoh.com>")
        .about("Extract files from a DRS archive")
        .arg(Arg::with_name("drs")
            .long("drs-path")
            .value_name("drs")
            .help("The path to the DRS you will extract from")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("file_id")
            .long("file-id")
            .help("ID of the file to extract")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("file_type")
            .long("file-type")
            .help("Type of the file to extract. One of: 'slp', 'shp', 'bin', or 'wav'")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("output_path")
            .short("o")
            .long("output-path")
            .help("Output filepath")
            .required(true)
            .takes_value(true))
        .get_matches();

    let drs_name = matches.value_of("drs").unwrap();
    let file_id = matches.value_of("file_id").unwrap().parse::<u32>().expect("Could not parse file_id into a u32");
    let file_type = match &*matches.value_of("file_type").unwrap()[..].to_lowercase() {
        "slp" => drs::DrsFileType::Slp,
        "shp" => drs::DrsFileType::Shp,
        "bin" => drs::DrsFileType::Binary,
        "wav" => drs::DrsFileType::Wav,
        _ => panic!("Invalid file_type! Expected one of: 'slp', 'shp', 'bin', or 'wav'"),
    };

    let archive = drs::DrsFile::read_from_file(&drs_name)
        .expect(&format!("Failed to load {}", &drs_name));

    let table = archive.find_table(file_type)
        .expect(&format!("Failed to find {:?} table in {}", &file_type, &drs_name));

    let contents = table.find_file_contents(file_id)
        .expect(&format!("Failed to find {:?} with id {} in {}", &file_type, &file_id, &drs_name));

    let output_path = matches.value_of("output_path").unwrap();
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(output_path)
        .expect(&format!("Failed to setup file {}", &output_path));

    f.write(&contents).expect(&format!("Failed to write to {}", &output_path));
    println!("Wrote {} bytes to {}", contents.len(), output_path);
}
