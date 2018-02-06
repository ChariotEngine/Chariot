// Chariot: An open source reimplementation of Age of Empires (1997)
// Copyright (c) 2018 Taryn Hill
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

use clap::{ArgMatches, App, SubCommand};
use ::{EmpiresDb, serde_json};

pub const COMMAND_NAME: &str = "empires2json";

pub fn get_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(COMMAND_NAME)
        .about("convert the `empires.dat` file to JSON which gets printed to STDOUT")
        .author("Taryn Hill <taryn@phrohdoh.com>")
        .help("example: empires <path/to/empires.db> empires2json")
}

pub fn run(db: EmpiresDb, _args: &ArgMatches) -> Result<(), String> {
    let json = serde_json::to_string(&db)
        .map_err(|e| format!("Failed to serialize the database to json: {}", e))?;

    println!("{}", json);
    Ok(())
}