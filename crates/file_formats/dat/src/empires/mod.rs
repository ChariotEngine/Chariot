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

#[macro_use]
mod resource;

mod age;
mod civ;
mod graphic;
mod player_color;
mod random_map;
mod research;
mod sound;
mod terrain_block;
mod terrain_restrictions;
mod unit;


use empires::age::{ResearchEffectGroup, read_ages};
pub use empires::civ::Civilization;
use empires::civ::read_civs;
pub use empires::graphic::Graphic;
use empires::graphic::read_graphics;
use empires::player_color::{PlayerColor, read_player_colors};
use empires::random_map::{RandomMap, read_random_maps};
use empires::research::{Research, read_research};
use empires::sound::{SoundEffectGroup, read_sound_effect_groups};
pub use empires::terrain_block::Terrain;

pub use empires::terrain_block::TerrainBlock;
pub use empires::terrain_block::TerrainBorder;
use empires::terrain_block::read_terrain_block;
use empires::terrain_restrictions::{TerrainRestriction, read_terrain_restrictions};
pub use empires::unit::{InteractionMode, Unit};
use error::{Result, ErrorKind};

use identifier::{UnitTerrainRestrictionId, TerrainId, CivilizationId, ResearchId, TerrainBorderId, PlayerColorId, UnitId, GraphicId, SoundGroupId, AgeId};
use chariot_io_tools::ReadExt;
use std::fs::File;
use std::io;
use std::io::prelude::{Read, Seek};
use std::path::Path;

use std::sync::Arc;

const EXPECTED_FILE_VERSION: &'static str = "VER 3.7\0";

/// Struct containing all of the game's information about terrain, civilizations,
/// players, units, sounds, tech, and random map generation.
#[derive(Default, Debug)]
pub struct EmpiresDb {
    terrain_restrictions: Vec<TerrainRestriction>,
    player_colors: Vec<PlayerColor>,
    sound_effect_groups: Vec<SoundEffectGroup>,
    graphics: Vec<Graphic>,
    terrain_block: TerrainBlock,
    random_maps: Vec<RandomMap>,
    ages: Vec<ResearchEffectGroup>,
    civilizations: Vec<Civilization>,
    research: Vec<Research>,
}

pub type EmpiresDbRef = Arc<EmpiresDb>;

impl EmpiresDb {
    pub fn new() -> EmpiresDb {
        Default::default()
    }

    /// Retrieve an age by ID
    #[inline]
    pub fn age<'a>(&'a self, age_id: AgeId) -> &'a ResearchEffectGroup {
        &self.ages[*age_id as usize]
    }

    /// Retrieve a player color by ID
    #[inline]
    pub fn player_color<'a>(&'a self, player_color_id: PlayerColorId) -> &'a PlayerColor {
        &self.player_colors[*player_color_id as usize]
    }

    /// Retrieve a civilization by ID
    #[inline]
    pub fn civilization<'a>(&'a self, civilization_id: CivilizationId) -> &'a Civilization {
        &self.civilizations[(*civilization_id - 1) as usize]
    }

    /// Retrieve a graphic by ID
    #[inline]
    pub fn graphic<'a>(&'a self, graphic_id: GraphicId) -> &'a Graphic {
        &self.graphics[*graphic_id as usize]
    }

    /// Convenience to quickly get unit information
    #[inline]
    pub fn unit<'a>(&'a self, civilization_id: CivilizationId, unit_id: UnitId) -> &'a Unit {
        self.civilization(civilization_id).unit(unit_id)
    }

    /// Retrieve the terrain information
    #[inline]
    pub fn terrain_block<'a>(&'a self) -> &'a TerrainBlock {
        &self.terrain_block
    }

    /// Convenience that returns terrain by ID
    #[inline]
    pub fn terrain<'a>(&'a self, terrain_id: TerrainId) -> &'a Terrain {
        self.terrain_block().terrain(terrain_id)
    }

    /// Convenience that returns a terrain restriction by ID
    #[inline]
    pub fn terrain_restrictions<'a>(&'a self,
                                    unit_terrain_restriction_id: UnitTerrainRestrictionId)
                                    -> &'a TerrainRestriction {
        &self.terrain_restrictions[unit_terrain_restriction_id.as_index()]
    }

    /// Convenience that returns terrain border by ID
    #[inline]
    pub fn terrain_border<'a>(&'a self, terrain_border_id: TerrainBorderId) -> &'a TerrainBorder {
        self.terrain_block().terrain_border(terrain_border_id)
    }

    /// Convenience to get the tile half sizes from the terrain block
    #[inline]
    pub fn tile_half_sizes(&self) -> (i32, i32) {
        self.terrain_block().tile_half_sizes()
    }

    /// Retrieve research information by ID
    #[inline]
    pub fn research<'a>(&'a self, research_id: ResearchId) -> &'a Research {
        &self.research[*research_id as usize]
    }

    /// Retrieve a sound effect group by ID
    #[inline]
    pub fn sound_effect_group<'a>(&'a self, sound_group_id: SoundGroupId) -> &'a SoundEffectGroup {
        &self.sound_effect_groups[*sound_group_id as usize]
    }

    /// Read all of the game data from the empires.dat file specified
    pub fn read_from_file<P: AsRef<Path>>(file_name: P) -> Result<EmpiresDb> {
        let file = try!(File::open(file_name.as_ref()));
        let mut stream = io::Cursor::new(try!(file.read_and_decompress()));

        try!(read_header(&mut stream));
        let terrain_restriction_count = try!(stream.read_u16()) as usize;
        let terrain_count = try!(stream.read_u16()) as usize;

        let mut db = EmpiresDb::new();

        db.terrain_restrictions =
            try!(read_terrain_restrictions(&mut stream, terrain_restriction_count, terrain_count));
        db.player_colors = try!(read_player_colors(&mut stream));
        db.sound_effect_groups = try!(read_sound_effect_groups(&mut stream));
        db.graphics = try!(read_graphics(&mut stream));
        db.terrain_block = try!(read_terrain_block(&mut stream));
        db.random_maps = try!(read_random_maps(&mut stream));
        db.ages = try!(read_ages(&mut stream));
        db.civilizations = try!(read_civs(&mut stream));
        db.research = try!(read_research(&mut stream));

        Ok(db)
    }
}

fn read_header<R: Read + Seek>(stream: &mut R) -> Result<()> {
    let mut version = [0u8; 8];
    try!(stream.read_exact(&mut version));
    if version != EXPECTED_FILE_VERSION.as_bytes() {
        return Err(ErrorKind::BadFile("unexpected file version").into());
    }

    Ok(())
}
