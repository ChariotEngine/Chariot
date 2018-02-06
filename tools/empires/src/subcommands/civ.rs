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

use std::error::Error;

use clap::{Arg, ArgMatches, App, SubCommand};
use ::EmpiresDb;

use id::{CivilizationId};

pub const COMMAND_NAME: &str = "civ";

const ARG_NAME_CIV_ID: &str = "civ-id";

pub fn get_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(COMMAND_NAME)
        .about("get information about a particular `civilization`")
        .author("Taryn Hill <taryn@phrohdoh.com>")
        .help("example: empires <path/to/empires.db> civ 2")
        .arg(Arg::with_name(ARG_NAME_CIV_ID)
                .index(1)
                .takes_value(true)
                .required(true))
}

pub fn run(db: EmpiresDb, args: &ArgMatches) -> Result<(), String> {
    let civ_id: CivilizationId = (value_t!(args, ARG_NAME_CIV_ID, u8).map_err(|e| e.description().to_string())? as usize).into();
    let civ = db.civilization(civ_id);
    println!("Name: {}", civ.name);

    //let units = civ.units.iter().map(|(unit_id, unit)| format!("id={}, name={}", unit_id.0, unit.name)).collect::<Vec<_>>();
    println!("Units: {:#?}", civ.units);

    Ok(())
}