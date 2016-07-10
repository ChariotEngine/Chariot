// OpenAOE: An open source reimplementation of Age of Empires (1997)
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

use error::*;
use player_data::PlayerData;
use player_resources::PlayerResources;
use player_unit::PlayerUnit;
use map::Map;

use io_tools::*;
use identifier::*;

use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::BTreeMap;

#[derive(Default, Debug)]
pub struct Scenario {
    header: ScenarioHeader,
    pub player_data: PlayerData,
    pub player_resources: BTreeMap<PlayerId, PlayerResources>,
    pub player_units: BTreeMap<PlayerId, Vec<PlayerUnit>>,
    pub map: Map,
}

impl Scenario {
    // TODO: Implement writing

    pub fn read_from_file<P: AsRef<Path>>(file_name: P) -> Result<Scenario> {
        Scenario::read_from_stream(try!(File::open(file_name.as_ref())))
    }

    pub fn read_from_stream<S: Read + Seek>(mut stream: S) -> Result<Scenario> {
        let mut scenario: Scenario = Default::default();
        scenario.header = try!(ScenarioHeader::read_from_stream(&mut stream));

        let mut stream = io::Cursor::new(try!(stream.read_and_decompress()));

        let _next_unit_id = try!(stream.read_u32()); // not sure what this is for yet
        scenario.player_data = try!(PlayerData::read_from_stream(&mut stream));
        scenario.map = try!(Map::read_from_stream(&mut stream));

        let player_unit_group_count = try!(stream.read_u32()) as isize;
        scenario.player_resources = id_map(try!(PlayerResources::read_from_stream(&mut stream)),
                                           &|pr: &PlayerResources| pr.player_id);

        for player_index in 0..player_unit_group_count {
            let player_id = PlayerId(player_index);
            let unit_count = try!(stream.read_u32()) as usize;
            let units = try!(stream.read_array(unit_count, |s| PlayerUnit::read_from_stream(s)));
            scenario.player_units.insert(player_id, units);
        }

        // TODO: Read other player data
        // TODO: Read triggers

        Ok(scenario)
    }
}

const REASONABLE_INSTRUCTION_LIMIT: usize = 512 * 1024; // 0.5 mibibytes

#[derive(Default, Debug)]
struct ScenarioHeader {
    version: String,
    length: u32,
    save_type: i32,
    last_save_time: u32,
    instructions: String,
    victory_type: u32,
    player_count: u32,
}

impl ScenarioHeader {
    // TODO: Implement writing

    fn read_from_stream<S: Read + Seek>(stream: &mut S) -> Result<ScenarioHeader> {
        let mut header: ScenarioHeader = Default::default();
        header.version = try!(stream.read_sized_str(4));
        if header.version != "1.11" {
            return Err(ErrorKind::UnrecognizedScenarioVersion.into());
        }

        header.length = try!(stream.read_u32());
        header.save_type = try!(stream.read_i32());
        header.last_save_time = try!(stream.read_u32());
        header.instructions = {
            let length = try!(stream.read_u32()) as usize;
            if length > REASONABLE_INSTRUCTION_LIMIT {
                // Refuse to load too many instructions
                return Err(ErrorKind::InstructionsTooLarge.into());
            }
            try!(stream.read_sized_str(length))
        };
        header.victory_type = try!(stream.read_u32());
        header.player_count = try!(stream.read_u32());
        Ok(header)
    }
}
