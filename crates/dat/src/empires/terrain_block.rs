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

use empires::EmpiresDb;
use error::*;

use io_tools::*;

use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;

const TILE_TYPE_COUNT: usize = 19;
const MAX_TERRAIN_UNITS: usize = 30;

#[derive(Default, Debug)]
pub struct TerrainFrameData {
    frame_count: i16,
    angle_count: i16,
    shape_id: i16,
}

impl TerrainFrameData {
    pub fn new() -> TerrainFrameData {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct TerrainBorder {
    enabled: bool,
    random: i8,
    name: String,
    short_name: String,
    slp_resource_id: i32,
    sound_id: i32,
    colors: [u8; 3],
    animated: bool,
    animation_frames: i16,
    pause_frames: i16,
    frame_interval: f32,
    pause_between_loops: f32,
    frame: i16,
    draw_frame: i16,
    animate_last: f32,
    frame_changed: i8,
    drawn: i8,
    borders: Vec<TerrainFrameData>,
    draw_tile: i16,
    underlay_terrain: i16,
    border_style: i16,
}

impl TerrainBorder {
    pub fn new() -> TerrainBorder {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct TerrainUnit {
    id: i16,
    density: i16,
    priority: i8,
}

impl TerrainUnit {
    pub fn new() -> TerrainUnit {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct Terrain {
    enabled: bool,
    random: i8,
    name: String,
    short_name: String,
    slp_resource_id: i32,
    sound_id: i32,
    colors: [u8; 3],
    cliff_colors: [u8; 2],
    passable_terrain: i8,
    impassable_terrain: i8,
    animated: bool,
    animation_frames: i16,
    pause_frames: i16,
    frame_interval: f32,
    pause_between_loops: f32,
    frame: i16,
    draw_frame: i16,
    animate_last: f32,
    frame_changed: i8,
    drawn: i8,
    elevation_graphics: Vec<TerrainFrameData>,
    terrain_to_draw: i16,
    terrain_width: i16,
    terrain_height: i16,
    terrain_borders: Vec<i16>,
    terrain_units: Vec<TerrainUnit>,
}

impl Terrain {
    pub fn new() -> Terrain {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct TileSize {
    width: i16,
    height: i16,
    delta_y: i16,
}

impl TileSize {
    pub fn new() -> TileSize {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct TerrainBlock {
    map_pointer: i32,
    map_width: i32,
    map_height: i32,
    world_width: i32,
    world_height: i32,
    tile_sizes: Vec<TileSize>,
    terrains: Vec<Terrain>,
    terrain_borders: Vec<TerrainBorder>,
    terrains_used: u16,
    borders_used: u16,
    max_terrain: i16,
    tile_width: i16,
    tile_height: i16,
    tile_half_height: i16,
    tile_half_width: i16,
    elevation_height: i16,
    current_row: i16,
    current_col: i16,
    block_begin_row: i16,
    block_end_row: i16,
    block_begin_col: i16,
    block_end_col: i16,
    any_frame_change: i8,
    map_visible: bool,
    fog: bool,
}

impl TerrainBlock {
    pub fn new() -> TerrainBlock {
        Default::default()
    }
}

impl EmpiresDb {
    pub fn read_terrain_block<R: Read + Seek>(&mut self, cursor: &mut R) -> EmpiresDbResult<()> {
        self.terrain_block.map_pointer = try!(cursor.read_i32());
        try!(cursor.read_i32()); // Unknown
        self.terrain_block.map_width = try!(cursor.read_i32());
        self.terrain_block.map_height = try!(cursor.read_i32());
        self.terrain_block.world_width = try!(cursor.read_i32());
        self.terrain_block.world_height = try!(cursor.read_i32());

        try!(EmpiresDb::read_tile_sizes(&mut self.terrain_block, cursor));
        try!(cursor.read_u16()); // Unknown
        try!(EmpiresDb::read_terrains(&mut self.terrain_block, cursor));
        try!(EmpiresDb::read_terrain_borders(&mut self.terrain_block, cursor));

        try!(cursor.read_i32()); // Unknown pointer
        self.terrain_block.terrains_used = try!(cursor.read_u16());
        self.terrain_block.borders_used = try!(cursor.read_u16());
        self.terrain_block.max_terrain = try!(cursor.read_i16());
        self.terrain_block.tile_width = try!(cursor.read_i16());
        self.terrain_block.tile_height = try!(cursor.read_i16());
        self.terrain_block.tile_half_height = try!(cursor.read_i16());
        self.terrain_block.tile_half_width = try!(cursor.read_i16());
        self.terrain_block.elevation_height = try!(cursor.read_i16());
        self.terrain_block.current_row = try!(cursor.read_i16());
        self.terrain_block.current_col = try!(cursor.read_i16());
        self.terrain_block.block_begin_row = try!(cursor.read_i16());
        self.terrain_block.block_end_row = try!(cursor.read_i16());
        self.terrain_block.block_begin_col = try!(cursor.read_i16());
        self.terrain_block.block_end_col = try!(cursor.read_i16());

        try!(cursor.read_u32()); // Unknown pointer
        try!(cursor.read_u32()); // Unknown pointer
        self.terrain_block.any_frame_change = try!(cursor.read_byte()) as i8;
        self.terrain_block.map_visible = try!(cursor.read_byte()) != 0;
        self.terrain_block.fog = try!(cursor.read_byte()) != 0;

        try!(cursor.seek(SeekFrom::Current(25))); // Skip 25 unknown bytes
        Ok(())
    }

    fn read_tile_sizes<R: Read + Seek>(terrain_block: &mut TerrainBlock, cursor: &mut R)
            -> EmpiresDbResult<()> {
        for _ in 0..TILE_TYPE_COUNT {
            let mut tile_size = TileSize::new();
            tile_size.width = try!(cursor.read_i16());
            tile_size.height = try!(cursor.read_i16());
            tile_size.delta_y = try!(cursor.read_i16());
            terrain_block.tile_sizes.push(tile_size);
        }
        Ok(())
    }

    fn read_terrains<R: Read + Seek>(terrain_block: &mut TerrainBlock, cursor: &mut R)
            -> EmpiresDbResult<()> {
        let terrain_count = 32;
        for _ in 0..terrain_count {
            let mut terrain = Terrain::new();

            terrain.enabled = try!(cursor.read_byte()) != 0;
            terrain.random = try!(cursor.read_byte()) as i8;
            terrain.name = try!(cursor.read_sized_str(13));
            terrain.short_name = try!(cursor.read_sized_str(13));
            terrain.slp_resource_id = try!(cursor.read_i32());
            try!(cursor.read_u32()); // Unknown
            terrain.sound_id = try!(cursor.read_i32());

            for i in 0..3 {
                terrain.colors[i] = try!(cursor.read_byte());
            }
            for i in 0..2 {
                terrain.cliff_colors[i] = try!(cursor.read_byte());
            }
            terrain.passable_terrain = try!(cursor.read_byte()) as i8;
            terrain.impassable_terrain = try!(cursor.read_byte()) as i8;

            terrain.animated = try!(cursor.read_byte()) != 0;
            terrain.animation_frames = try!(cursor.read_i16());
            terrain.pause_frames = try!(cursor.read_i16());
            terrain.frame_interval = try!(cursor.read_f32());
            terrain.pause_between_loops = try!(cursor.read_f32());
            terrain.frame = try!(cursor.read_i16());
            terrain.draw_frame = try!(cursor.read_i16());
            terrain.animate_last = try!(cursor.read_f32());
            terrain.frame_changed = try!(cursor.read_byte()) as i8;
            terrain.drawn = try!(cursor.read_byte()) as i8;

            try!(read_into_vec(&mut terrain.elevation_graphics, TILE_TYPE_COUNT,
                &mut || EmpiresDb::read_frame_data(cursor)));

            terrain.terrain_to_draw = try!(cursor.read_i16());
            terrain.terrain_width = try!(cursor.read_i16());
            terrain.terrain_height = try!(cursor.read_i16());

            try!(read_into_vec(&mut terrain.terrain_borders, terrain_count, &mut || cursor.read_i16()));
            try!(EmpiresDb::read_terrain_units(&mut terrain.terrain_units, cursor));
            try!(cursor.read_u16()); // Unknown

            terrain_block.terrains.push(terrain);
        }
        Ok(())
    }

    fn read_terrain_units<R: Read>(terrain_units: &mut Vec<TerrainUnit>, cursor: &mut R)
            -> EmpiresDbResult<()> {
        let (mut ids, mut densities, mut priorities) = (Vec::new(), Vec::new(), Vec::new());
        try!(read_into_vec(&mut ids, MAX_TERRAIN_UNITS, &mut || cursor.read_i16()));
        try!(read_into_vec(&mut densities, MAX_TERRAIN_UNITS, &mut || cursor.read_i16()));
        try!(read_into_vec(&mut priorities, MAX_TERRAIN_UNITS,
            &mut || { cursor.read_byte().map(|v| v as i8) }));

        let terrain_units_used = try!(cursor.read_i16()) as usize;
        if terrain_units_used > MAX_TERRAIN_UNITS {
            return Err(EmpiresDbError::BadFile("invalid number of terrain units used"))
        }

        for i in 0..terrain_units_used {
            let mut unit = TerrainUnit::new();
            unit.id = ids[i];
            unit.density = densities[i];
            unit.priority = priorities[i];
            terrain_units.push(unit);
        }
        Ok(())
    }

    fn read_frame_data<R: Read>(cursor: &mut R) -> io::Result<TerrainFrameData> {
        let mut frame_data = TerrainFrameData::new();
        frame_data.frame_count = try!(cursor.read_i16());
        frame_data.angle_count = try!(cursor.read_i16());
        frame_data.shape_id = try!(cursor.read_i16());
        Ok(frame_data)
    }

    fn read_terrain_borders<R: Read + Seek>(terrain_block: &mut TerrainBlock, cursor: &mut R)
            -> EmpiresDbResult<()> {
        let terrain_border_count = 16;
        for _ in 0..terrain_border_count {
            let mut border = TerrainBorder::new();

            border.enabled = try!(cursor.read_byte()) != 0;
            border.random = try!(cursor.read_byte()) as i8;
            border.name = try!(cursor.read_sized_str(13));
            println!("ENABLED {} NAME {} OFFSET: {:X}", border.enabled, border.name, cursor.seek(SeekFrom::Current(0)).unwrap());
            border.short_name = try!(cursor.read_sized_str(13));
            border.slp_resource_id = try!(cursor.read_i32());
            try!(cursor.read_u32()); // Unknown
            border.sound_id = try!(cursor.read_i32());

            for i in 0..3 {
                border.colors[i] = try!(cursor.read_byte());
            }

            border.animated = try!(cursor.read_byte()) != 0;
            border.animation_frames = try!(cursor.read_i16());
            border.pause_frames = try!(cursor.read_i16());
            border.frame_interval = try!(cursor.read_f32());
            border.pause_between_loops = try!(cursor.read_f32());
            border.frame = try!(cursor.read_i16());
            border.draw_frame = try!(cursor.read_i16());
            border.animate_last = try!(cursor.read_f32());
            border.frame_changed = try!(cursor.read_byte()) as i8;
            border.drawn = try!(cursor.read_byte()) as i8;

            try!(read_into_vec(&mut border.borders, 12, &mut || EmpiresDb::read_frame_data(cursor)));

            border.draw_tile = try!(cursor.read_i16());
            border.underlay_terrain = try!(cursor.read_i16());
            border.border_style = try!(cursor.read_i16());

            terrain_block.terrain_borders.push(border);
        }
        Ok(())
    }
}

fn read_into_vec<T, F>(into: &mut Vec<T>, count: usize, read_method: &mut F)
        -> EmpiresDbResult<()>
        where F: FnMut() -> io::Result<T> {
    for _ in 0..count {
        into.push(try!(read_method()));
    }
    Ok(())
}
