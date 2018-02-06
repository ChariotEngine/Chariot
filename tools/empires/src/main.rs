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

extern crate chariot_dat as dat;
use dat::EmpiresDb;

extern crate chariot_identifier as id;

mod subcommands;
use subcommands as cmd;

#[macro_use(value_t)]
extern crate clap;

use clap::{App, Arg, AppSettings};

fn main() {
    let matches = App::new("empires")
        .version("0.0.1")
        .author("Taryn Hill <taryn@phrohdoh.com>")
        .about("Allows you to query the `empires.dat` file shipped with Age of Empires (1997)")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(Arg::with_name("DAT_LOCATION")
            .index(1)
            .value_name("DAT_LOCATION")
            .help("Sets the path to the `empires.dat` file")
            .required(true)
            .takes_value(true))
        .subcommand(cmd::unit::get_command())
        .subcommand(cmd::civ::get_command())
        .get_matches();

    let dat_location = matches.value_of("DAT_LOCATION").unwrap();
    let db = EmpiresDb::read_from_file(dat_location).unwrap();

    let run_result = match matches.subcommand() {
        (cmd::unit::COMMAND_NAME, Some(args)) => cmd::unit::run(db, args),
        (cmd::civ::COMMAND_NAME, Some(args)) => cmd::civ::run(db, args),
        _ => Ok(()),
    };

    if let Err(e) = run_result {
        eprintln!("ERROR: {}", e);
        std::process::exit(1);
    }
}
