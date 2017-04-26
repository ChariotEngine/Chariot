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

use std::fmt;
use std::ops::Deref;

macro_rules! create_id_type {
    ($name:ident, $underlying_type:ty) => {
        #[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $name($underlying_type);

        // Implement Debug instead of deriving it so that we can keep it all
        // on one line when formatted with {:#?}
        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, concat!(stringify!($name), "({})"), self.0)
            }
        }

        impl Deref for $name {
            type Target = $underlying_type;
            fn deref(&self) -> &$underlying_type {
                &self.0
            }
        }

        impl Into<$name> for usize {
            fn into(self) -> $name {
                $name(self as $underlying_type)
            }
        }
    }
}

/// ID for players
create_id_type!(PlayerId, u8);

impl Into<PlayerColorId> for PlayerId {
    fn into(self) -> PlayerColorId {
        PlayerColorId(self.0)
    }
}

/// ID for player colors in the empires.dat file
create_id_type!(PlayerColorId, u8);

/// ID for an SLP file in a DRS package
create_id_type!(SlpFileId, u32);

/// ID for a frame inside of an SLP file
create_id_type!(SlpFrameId, u32);

/// ID for a WAV file in a DRS package
create_id_type!(WavFileId, u32);

/// ID for a graphic in the empires.dat file
create_id_type!(GraphicId, u32);

/// ID for a sound group in the empires.dat file
create_id_type!(SoundGroupId, u32);

/// ID for an age (that defines effects for research) in the empires.dat file
create_id_type!(AgeId, u32);

/// ID for research in the empires.dat file
create_id_type!(ResearchId, u32);

/// ID for civilizations in the empires.dat file
create_id_type!(CivilizationId, u8);

/// ID for units in the empires.dat file
create_id_type!(UnitId, u32);

/// ID for unit classes in the empires.dat file
create_id_type!(UnitClassId, u32);

/// ID for unit commands in the empires.dat file
create_id_type!(UnitCommandId, u32);

/// Spawn ID for units on a map (basically a unique identifier for that unit instance on the map)
create_id_type!(SpawnId, u32);

/// ID for finding a localized string in language.dll
create_id_type!(LocalizationId, u32);

/// ID for random map script references in the empires.dat file
create_id_type!(RandomMapScriptId, u32);

/// ID for terrains defined in empires.dat
create_id_type!(TerrainId, u8);

/// ID for terrain borders defined in empires.dat
create_id_type!(TerrainBorderId, u8);

/// Different classes of terrain restriction for a unit
#[derive(Copy, Clone, Debug)]
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

    pub fn as_index(&self) -> usize {
        use self::UnitTerrainRestrictionId::*;
        match *self {
            Flying => 0,
            GroundWildlife => 1,
            Beach => 2,
            WaterBorne => 3,
            GroundBuilding => 4,
            CompletelyRestricted => 5,
            Dock => 6,
            GroundUnit => 7,
            ResourceSite => 8,
            Farm => 9,
            Wall => 10,
            Unknown(index) => index,
        }
    }
}

impl Default for UnitTerrainRestrictionId {
    fn default() -> UnitTerrainRestrictionId {
        UnitTerrainRestrictionId::Flying
    }
}

impl Into<UnitTerrainRestrictionId> for usize {
    fn into(self) -> UnitTerrainRestrictionId {
        UnitTerrainRestrictionId::from_index(self)
    }
}

/// Checks that the given integral type is not -1 (or max int if unsigned)
/// and converts the value to a usize.
#[macro_export]
macro_rules! required_id {
    ($id:expr) => {
        {
            let id = $id;
            if id == -1 {
                panic!("Required ID is -1");
            }
            (id as usize).into()
        }
    }
}

/// If the given value is -1, returns None. Otherwise, returns an option of
/// the integral type converted to usize.
#[macro_export]
macro_rules! optional_id {
    ($id:expr) => {
        {
            let id = $id;
            if id == -1 {
                None
            } else {
                Some((id as usize).into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deref() {
        assert_eq!(5u8, *TerrainId(5));
    }

    #[test]
    #[should_panic]
    fn test_required_id_failure() {
        let _id: TerrainId = required_id!(-1i16);
    }

    #[test]
    fn test_required_id() {
        let id: TerrainId = required_id!(5i32);
        assert_eq!(TerrainId(5), id);

        let id: TerrainId = required_id!(5isize);
        assert_eq!(TerrainId(5), id);
    }

    #[test]
    fn test_optional_id() {
        let val: Option<TerrainId> = optional_id!(-1i32);
        assert_eq!(None, val);

        let val: Option<TerrainId> = optional_id!(5i32);
        assert_eq!(Some(TerrainId(5)), val);
    }
}
