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

use empires::resource::*;

use error::*;

use identifier::*;
use chariot_io_tools::*;

use std::io::prelude::*;

const MAX_REQUIRED_TECHS: usize = 4;
const RESOURCE_COST_COUNT: usize = 3;

pub type ResearchCost = ResourceCost<i16, u8>;

#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[derive(Default, Debug)]
pub struct Research {
    pub id: ResearchId,
    pub required_techs: Vec<i16>,
    pub resource_costs: Vec<ResearchCost>,

    /// Unit id of the location this research can be performed
    pub location: Option<UnitId>,

    pub name_id: LocalizationId,
    pub description_id: LocalizationId,

    /// How much time the research takes to complete
    pub time_seconds: i16,

    pub age_id: Option<AgeId>,
    pub type_id: i16,

    /// Frame number in 50729.slp in interfac.drs to use for the button graphic
    pub icon_id: i16,

    /// Button slot position
    pub button_id: i8,
    pub help_id: Option<LocalizationId>,
    pub tech_tree_id: Option<LocalizationId>,
    pub name: String,
}

pub fn read_research<R: Read + Seek>(stream: &mut R) -> Result<Vec<Research>> {
    let research_count = try!(stream.read_u16()) as usize;
    let mut research = try!(stream.read_array(research_count, |c| read_single_research(c)));
    for i in 0..research.len() {
        research[i].id = i.into();
    }
    Ok(research)
}

fn read_single_research<R: Read + Seek>(stream: &mut R) -> Result<Research> {
    let mut research: Research = Default::default();
    research.required_techs = try!(stream.read_array(MAX_REQUIRED_TECHS, |c| c.read_i16()));
    research.resource_costs = read_resource_costs!(i16, u8, stream, RESOURCE_COST_COUNT);

    let actual_required_techs = try!(stream.read_u16()) as usize;
    if actual_required_techs > MAX_REQUIRED_TECHS {
        return Err(ErrorKind::BadFile("more required techs than possible").into());
    } else {
        research.required_techs.resize(actual_required_techs, Default::default());
    }

    research.location = optional_id!(try!(stream.read_i16()));
    research.name_id = required_id!(try!(stream.read_i16()));
    research.description_id = required_id!(try!(stream.read_i16()));
    research.time_seconds = try!(stream.read_i16());
    research.age_id = optional_id!(try!(stream.read_i16()));
    research.type_id = try!(stream.read_i16());
    research.icon_id = try!(stream.read_i16());
    research.button_id = try!(stream.read_i8());
    research.help_id = optional_id!(try!(stream.read_i32()));
    research.tech_tree_id = optional_id!(try!(stream.read_i32()));
    try!(stream.read_i32()); // Unknown

    let name_length = try!(stream.read_u16()) as usize;
    if name_length > 0 {
        research.name = try!(stream.read_sized_str(name_length));
    }
    Ok(research)
}
