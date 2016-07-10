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

use std::fmt;
use std::cmp;
use std::collections::BTreeMap;
use std::hash;

/// ID for players
#[derive(Default, Clone, Copy)]
pub struct PlayerId(pub isize);

/// ID for player colors in the empires.dat file
#[derive(Default, Clone, Copy)]
pub struct PlayerColorId(pub isize);

/// ID for an SLP file in a DRS package
#[derive(Default, Clone, Copy)]
pub struct SlpFileId(pub isize);

/// ID for a frame inside of an SLP file
#[derive(Default, Clone, Copy)]
pub struct SlpFrameId(pub isize);

/// ID for a WAV file in a DRS package
#[derive(Default, Clone, Copy)]
pub struct WavFileId(pub isize);

/// ID for a graphic in the empires.dat file
#[derive(Default, Clone, Copy)]
pub struct GraphicId(pub isize);

/// ID for a sound group in the empires.dat file
#[derive(Default, Clone, Copy)]
pub struct SoundGroupId(pub isize);

/// ID for an age (that defines effects for research) in the empires.dat file
#[derive(Default, Clone, Copy)]
pub struct AgeId(pub isize);

/// ID for research in the empires.dat file
#[derive(Default, Clone, Copy)]
pub struct ResearchId(pub isize);

/// ID for units in the empires.dat file
#[derive(Default, Clone, Copy)]
pub struct UnitId(pub isize);

/// ID for unit classes in the empires.dat file
#[derive(Default, Clone, Copy)]
pub struct UnitClassId(pub isize);

/// ID for unit commands in the empires.dat file
#[derive(Default, Clone, Copy)]
pub struct UnitCommandId(pub isize);

/// Spawn ID for units on a map (basically a unique identifier for that unit instance on the map)
#[derive(Default, Clone, Copy)]
pub struct SpawnId(pub isize);

/// ID for finding a localized string in language.dll
#[derive(Default, Clone, Copy)]
pub struct LocalizationId(pub isize);

/// ID for random map script references in the empires.dat file
#[derive(Default, Clone, Copy)]
pub struct RandomMapScriptId(pub isize);

/// ID for terrains defined in empires.dat
#[derive(Default, Clone, Copy)]
pub struct TerrainId(pub isize);

/// ID for terrain borders defined in empires.dat
#[derive(Default, Clone, Copy)]
pub struct TerrainBorderId(pub isize);

pub fn id_map<I: Ord, T>(items: Vec<T>, id_getter: &Fn(&T) -> I) -> BTreeMap<I, T> {
    let mut map: BTreeMap<I, T> = BTreeMap::new();
    for item in items {
        let id = id_getter(&item);
        map.insert(id, item);
    }
    map
}

macro_rules! impl_id {
    ($id_type:ty) => {
        // Implement Debug instead of deriving it so that we can keep it all
        // on one line when formatted with {:#?}
        impl fmt::Debug for $id_type {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, concat!(stringify!($id_type), "({})"), self.0)
            }
        }

        impl cmp::PartialEq for $id_type {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl cmp::Eq for $id_type { }

        impl cmp::PartialOrd for $id_type {
            fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        impl cmp::Ord for $id_type {
            fn cmp(&self, other: &Self) -> cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }

        impl hash::Hash for $id_type {
            fn hash<H>(&self, state: &mut H) where H: hash::Hasher {
                self.0.hash(state)
            }
        }

        impl $id_type {
            pub fn as_isize(&self) -> isize {
                self.0
            }

            pub fn as_usize(&self) -> usize {
                if self.0 < 0 {
                    println!("WARN: Negative ID taken as unsigned");
                }
                self.0 as usize
            }
        }
    }
}

impl_id!(PlayerId);
impl_id!(PlayerColorId);
impl_id!(SlpFileId);
impl_id!(SlpFrameId);
impl_id!(WavFileId);
impl_id!(GraphicId);
impl_id!(SoundGroupId);
impl_id!(AgeId);
impl_id!(ResearchId);
impl_id!(UnitId);
impl_id!(UnitClassId);
impl_id!(UnitCommandId);
impl_id!(SpawnId);
impl_id!(LocalizationId);
impl_id!(RandomMapScriptId);
impl_id!(TerrainId);
impl_id!(TerrainBorderId);

/// Different classes of terrain restriction for a unit
#[derive(Debug)]
pub enum UnitTerrainRestrictionId {
    /// Units that fly or are in the air (dying units and missiles)
    Flying,

    GroundWildlife,

    /// Identifies terrain as "beach"
    Beach,

    /// Typically boats
    WaterBorne,

    GroundBuilding,

    /// All terrains are restricted; nothing seems to use this
    CompletelyRestricted,

    /// The dock is a special unit that has to be placed on land, beach, shallows, and ocean
    Dock,

    GroundUnit,

    /// Not sure what this is for yet... terrains that gold/stone/berries can be placed on?
    ResourceSite,

    Farm,
    Wall,
    Unknown(usize),
}

impl UnitTerrainRestrictionId {
    pub fn from_index(index: usize) -> UnitTerrainRestrictionId {
        use self::UnitTerrainRestrictionId::*;
        match index {
            0 => Flying,
            1 => GroundWildlife,
            2 => Beach,
            3 => WaterBorne,
            4 => GroundBuilding,
            5 => CompletelyRestricted,
            6 => Dock,
            7 => GroundUnit,
            8 => ResourceSite,
            9 => Farm,
            10 => Wall,
            _ => Unknown(index),
        }
    }
}

impl Default for UnitTerrainRestrictionId {
    fn default() -> UnitTerrainRestrictionId {
        UnitTerrainRestrictionId::Flying
    }
}
