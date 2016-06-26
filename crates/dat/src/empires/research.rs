//
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
//

use error::*;

use io_tools::*;

use std::io::prelude::*;

const MAX_REQUIRED_TECHS: usize = 4;
const RESOURCE_COST_COUNT: usize = 3;

#[derive(Default, Debug)]
pub struct ResearchCost {
    type_id: i16,
    amount: i16,
    enabled: bool,
}

#[derive(Default, Debug)]
pub struct Research {
    id: u16,
    required_techs: Vec<i16>,
    resource_costs: Vec<ResearchCost>,
    location: i16,
    language_dll_name: u16,
    language_dll_description: u16,
    time: i16,
    age_id: i16,
    type_id: i16,
    icon_id: i16,
    button_id: i8,
    language_dll_help: i32,
    language_dll_tech_tree: i32,
    name: String,
}

pub fn read_research<R: Read + Seek>(stream: &mut R) -> EmpiresDbResult<Vec<Research>> {
    let research_count = try!(stream.read_u16()) as usize;
    let mut research = try!(stream.read_array(research_count, |c| read_single_research(c)));
    for i in 0..research.len() {
        research[i].id = i as u16;
    }
    Ok(research)
}

fn read_single_research<R: Read + Seek>(stream: &mut R) -> EmpiresDbResult<Research> {
    let mut research: Research = Default::default();
    research.required_techs = try!(stream.read_array(MAX_REQUIRED_TECHS, |c| c.read_i16()));
    research.resource_costs = try!(stream.read_array(RESOURCE_COST_COUNT, |c| read_research_cost(c)));

    let actual_required_techs = try!(stream.read_u16()) as usize;
    if actual_required_techs > MAX_REQUIRED_TECHS {
        return Err(EmpiresDbError::BadFile("more required techs than possible"))
    } else {
        research.required_techs.resize(actual_required_techs, Default::default());
    }

    research.location = try!(stream.read_i16());
    research.language_dll_name = try!(stream.read_u16());
    research.language_dll_description = try!(stream.read_u16());
    research.time = try!(stream.read_i16());
    research.age_id = try!(stream.read_i16());
    research.type_id = try!(stream.read_i16());
    research.icon_id = try!(stream.read_i16());
    research.button_id = try!(stream.read_i8());
    research.language_dll_help = try!(stream.read_i32());
    research.language_dll_tech_tree = try!(stream.read_i32());
    try!(stream.read_i32()); // Unknown

    let name_length = try!(stream.read_u16()) as usize;
    if name_length > 0 {
        research.name = try!(stream.read_sized_str(name_length));
    }
    Ok(research)
}

fn read_research_cost<R: Read>(stream: &mut R) -> EmpiresDbResult<ResearchCost> {
    let mut cost: ResearchCost = Default::default();
    cost.type_id = try!(stream.read_i16());
    cost.amount = try!(stream.read_i16());
    cost.enabled = try!(stream.read_u8()) != 0;
    Ok(cost)
}
