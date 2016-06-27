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

use empires::resource::*;
use error::*;

use io_tools::*;

use std::io::prelude::*;

#[derive(Debug)]
pub enum UnitAttributeId {
    HitPoints,
    LineOfSight,
    SizeRadius1,
    SizeRadius2,
    Speed,
    ArmorStrength,
    AttackStrength,
    ReloadTime,
    AttackAccuracy,
    AttackRange,
    WorkRate,
    ResourceCarryCapacity,
    MissileUnitId,
    BuildingUpgradeLevel,
    MissileAccuracyMode,
    ResourceCost,
    Unknown(i16),
}

impl UnitAttributeId {
    pub fn from_i16(val: i16) -> UnitAttributeId {
        use self::UnitAttributeId::*;
        match val {
            0 => HitPoints,
            1 => LineOfSight,
            3 => SizeRadius1,
            4 => SizeRadius2,
            5 => Speed,
            8 => ArmorStrength,
            9 => AttackStrength,
            10 => ReloadTime,
            11 => AttackAccuracy,
            12 => AttackRange,
            13 => WorkRate,
            14 => ResourceCarryCapacity,
            16 => MissileUnitId,
            17 => BuildingUpgradeLevel,
            19 => MissileAccuracyMode,
            100 => ResourceCost,
            _ => Unknown(val),
        }
    }
}

#[derive(Debug)]
pub enum AgeEffectValue {
    SetTo(f32),
    Add(f32),
    MultiplyBy(f32),
}

#[derive(Debug)]
pub enum AgeEffect {
    UnitAttribute {
        target_unit_id: i16,
        target_unit_class_id: i16,
        attribute_id: UnitAttributeId,
        effect: AgeEffectValue,
    },

    CivHeader {
        target_civ_header_id: i16,
        effect: AgeEffectValue,
    },

    SetUnitEnabled {
        target_unit_id: i16,
        enabled: bool,
    },

    UpgradeUnit {
        source_unit_id: i16,
        target_unit_id: i16,
    },

    ResearchCost {
        research_id: i16,
        resource_type: ResourceType,
        effect: AgeEffectValue,
    },

    DisableResearch {
        research_id: i16,
    },

    GainResearch {
        research_id: i16,
    },

    Unknown {
        type_id: i8,
        param_a: i16,
        param_b: i16,
        param_c: i16,
        param_d: f32,
    },
}

impl Default for AgeEffect {
    fn default() -> AgeEffect {
        AgeEffect::Unknown {
            type_id: -1,
            param_a: -1,
            param_b: -1,
            param_c: -1,
            param_d: -1f32,
        }
    }
}


#[derive(Default, Debug)]
pub struct Age {
    id: usize,
    name: String,
    effects: Vec<AgeEffect>,
}

pub fn read_ages<R: Read + Seek>(stream: &mut R) -> EmpiresDbResult<Vec<Age>> {
    let age_count = try!(stream.read_u32()) as usize;
    let mut ages = try!(stream.read_array(age_count, |c| read_age(c)));
    for (index, age) in ages.iter_mut().enumerate() {
        age.id = index;
    }
    Ok(ages)
}

pub fn read_age<R: Read + Seek>(stream: &mut R) -> EmpiresDbResult<Age> {
    let mut age: Age = Default::default();
    age.name = try!(stream.read_sized_str(31));

    let effect_count = try!(stream.read_u16()) as usize;
    age.effects = try!(stream.read_array(effect_count, |c| read_age_effect(c)));
    Ok(age)
}

fn read_age_effect<R: Read + Seek>(stream: &mut R) -> EmpiresDbResult<AgeEffect> {
    let type_id = try!(stream.read_i8());
    let param_a = try!(stream.read_i16());
    let param_b = try!(stream.read_i16());
    let param_c = try!(stream.read_i16());
    let param_d = try!(stream.read_f32());

    use self::AgeEffect::*;
    use self::AgeEffectValue::*;
    let result = match type_id {
        0 | 4 | 5 => UnitAttribute {
            target_unit_id: param_a,
            target_unit_class_id: param_b,
            attribute_id: UnitAttributeId::from_i16(param_c),
            effect: match type_id {
                0 => SetTo(param_d),
                4 => Add(param_d),
                _ => MultiplyBy(param_d),
            },
        },

        1 if param_b == 0 || param_b == 1 => CivHeader {
            target_civ_header_id: param_a,
            effect: match param_b {
                0 => SetTo(param_d),
                _ => Add(param_d),
            }
        },

        6 => CivHeader {
            target_civ_header_id: param_a,
            effect: MultiplyBy(param_d),
        },

        2 => SetUnitEnabled {
            target_unit_id: param_a,
            enabled: param_b == 1,
        },

        3 => UpgradeUnit {
            source_unit_id: param_a,
            target_unit_id: param_b,
        },

        101 if param_c == 0 || param_c == 1 => ResearchCost {
            research_id: param_a,
            resource_type: ResourceType::from_i16(param_b),
            effect: match param_c {
                0 => SetTo(param_d),
                _ => Add(param_d),
            }
        },

        102 => DisableResearch {
            research_id: param_d as i16,
        },

        103 => GainResearch {
            research_id: param_a,
        },

        _ => Unknown {
            type_id: type_id,
            param_a: param_a,
            param_b: param_b,
            param_c: param_c,
            param_d: param_d,
        }
    };

    Ok(result)
}
