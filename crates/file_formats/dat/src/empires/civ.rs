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
//

use empires::resource::ResourceType;
use empires::unit::{Unit, read_unit};
use error::*;

use identifier::*;
use chariot_io_tools::*;
use std::collections::{BTreeMap, HashMap};

use std::io::prelude::*;

#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[derive(Default, Debug)]
pub struct CivilizationStartingValues {
    /// Starting resource values (for random map)
    resources: BTreeMap<ResourceType, f32>,

    /// Multiplier for trade
    trade_productivity: f32,

    /// Amount of food a farm provides; can increase with research
    farm_food_capacity: f32,

    /// Amount of a tribute that is removed as a penalty; can decrease with research
    tribute_penalty: f32,

    /// Multiplier for gold mined; increases with research
    gold_mine_productivity: f32,

    /// Used to initialize unit attributes for the civ
    age_id: Option<AgeId>,

    /// If not starting in the default age, grant the given tech based on what the starting age is
    tool_age_research_id: ResearchId,
    bronze_age_research_id: ResearchId,
    iron_age_research_id: ResearchId,

    attack_warning_sound_id: SoundGroupId,
}

#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[derive(Default, Debug)]
pub struct Civilization {
    id: CivilizationId,
    enabled: bool,
    pub name: String,
    starting_values: CivilizationStartingValues,

    /// Determines which user interface theme to use
    /// 0 => Egyption interface, 1 => Greek, 2 => Babylonian, 3 => Asiatic, 4 => Roman
    icon_set: i8,

    pub units: HashMap<UnitId, Unit>,
}

impl Civilization {
    /// Retrieve unit data by ID
    #[inline]
    pub fn unit<'a>(&'a self, unit_id: UnitId) -> &'a Unit {
        return &self.units[&unit_id];
    }
}

pub fn read_civs<R: Read + Seek>(stream: &mut R) -> Result<Vec<Civilization>> {
    let civ_count = try!(stream.read_u16()) as usize;
    let mut result = try!(stream.read_array(civ_count, |c| read_civ(c)));
    for (index, civ) in result.iter_mut().enumerate() {
        // The rest of the data files refer to civs with a 1-based index
        civ.id = (index + 1).into();
    }
    Ok(result)
}

fn read_civ<R: Read + Seek>(stream: &mut R) -> Result<Civilization> {
    let mut civ: Civilization = Default::default();
    civ.enabled = try!(stream.read_u8()) != 0;
    civ.name = try!(stream.read_sized_str(20));

    let starting_value_count = try!(stream.read_u16()) as usize;
    civ.starting_values.age_id = optional_id!(try!(stream.read_i16()));

    // As far as I can tell, AOE holds a massive blob of floating point data for each civ,
    // and it initializes that blob with whatever is in the file here. Several of the values
    // only make sense in the context of the game having been played for a while
    // (i.e., kill count). Others are useful for the start of the game, however.
    // Only save the values that make sense in the context of starting the game.
    let starting_values = try!(stream.read_array(starting_value_count, |c| c.read_f32()));
    civ.starting_values.resources.insert(ResourceType::Food, starting_values[0]);
    civ.starting_values.resources.insert(ResourceType::Wood, starting_values[1]);
    civ.starting_values.resources.insert(ResourceType::Stone, starting_values[2]);
    civ.starting_values.resources.insert(ResourceType::Gold, starting_values[3]);
    civ.starting_values.trade_productivity = starting_values[10];
    civ.starting_values.farm_food_capacity = starting_values[36];
    civ.starting_values.tribute_penalty = starting_values[46];
    civ.starting_values.gold_mine_productivity = starting_values[47];
    civ.starting_values.tool_age_research_id = required_id!(starting_values[25] as i32);
    civ.starting_values.bronze_age_research_id = required_id!(starting_values[23] as i32);
    civ.starting_values.iron_age_research_id = required_id!(starting_values[24] as i32);
    civ.starting_values.attack_warning_sound_id = required_id!(starting_values[26] as i32);

    civ.icon_set = try!(stream.read_i8());

    let unit_count = try!(stream.read_u16()) as usize;
    let unit_pointers = try!(stream.read_array(unit_count, |c| c.read_i32()));
    for i in 0..unit_count {
        // Similarly with graphics, units have an array of pointers that are meaningless
        // except that if one of them is zero, that unit has to be skipped
        if unit_pointers[i] != 0 {
            let unit = try!(read_unit(stream));
            civ.units.insert(unit.id, unit);
        }
    }
    Ok(civ)
}
