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

use empires::id::*;
use error::*;

use io_tools::*;

use std::io::prelude::*;
use std::io::SeekFrom;
use std::collections::BTreeMap;

const TILE_TYPE_COUNT: usize = 19;
const MAX_TERRAIN_UNITS: usize = 30;

#[derive(Default, Debug)]
pub struct TerrainFrameData {
    frame_count: i16,
    angle_count: i16,
    shape_id: i16, // TODO: Figure out what this ID references
}

#[derive(Default, Debug)]
pub struct TerrainBorder {
    enabled: bool,
    name: String,
    short_name: String,
    slp_id: SlpFileId,
    sound_group_id: SoundGroupId,
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

    /// Which terrain is drawn on the bottom
    underlay_terrain_id: TerrainId,

    border_style: i16,
    borders: Vec<Vec<TerrainFrameData>>,
}

#[derive(Default, Debug)]
pub struct TerrainUnit {
    unit_id: UnitId,
    density: i16,
    priority: i8,
}

#[derive(Default, Debug)]
pub struct Terrain {
    id: TerrainId,
    enabled: bool,
    name: String,
    short_name: String,
    slp_id: SlpFileId,
    sound_group_id: SoundGroupId,
    colors: [u8; 3],
    cliff_colors: [u8; 2],

    /// ID of the equivalent terrain (same everything) that is passable
    pass_terrain_id: TerrainId,

    /// ID of the equivalent terrain (same everything) that is not passable
    impass_terrain_id: TerrainId,

    animated: bool,
    animation_frames: i16,
    pause_frames: i16,
    frame_interval: f32,
    pause_between_loops: f32,
    frame: i16,
    draw_frame: i16,
    animate_last: f32,
    frame_changed: i8,
    elevation_graphics: Vec<TerrainFrameData>,

    /// If this is set, use the graphics for this other terrain instead of the ones for this one
    terrain_to_draw: TerrainId,

    /// Speculating: this seems to be a minimum brush size for random map generation
    terrain_width: i16,
    terrain_height: i16,

    /// Number of tiles wide of a border to make around the given terrain ID during random map generation
    terrain_borders: BTreeMap<TerrainId, i16>,

    /// Units that speckle this terrain (randomly)
    terrain_units: Vec<TerrainUnit>,
}

#[derive(Default, Debug)]
pub struct TileSize {
    width: i16,
    height: i16,
    delta_y: i16,
}

#[derive(Default, Debug)]
pub struct TerrainBlock {
    map_width: i32,
    map_height: i32,
    world_width: i32,
    world_height: i32,
    tile_sizes: Vec<TileSize>,
    terrains: BTreeMap<TerrainId, Terrain>,
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

pub fn read_terrain_block<R: Read + Seek>(stream: &mut R) -> EmpiresDbResult<TerrainBlock> {
    let mut terrain_block: TerrainBlock = Default::default();

    try!(stream.read_i32()); // Map pointer; not needed
    try!(stream.read_i32()); // Unknown
    terrain_block.map_width = try!(stream.read_i32());
    terrain_block.map_height = try!(stream.read_i32());
    terrain_block.world_width = try!(stream.read_i32());
    terrain_block.world_height = try!(stream.read_i32());

    try!(read_tile_sizes(&mut terrain_block, stream));
    try!(stream.read_u16()); // Unknown

    terrain_block.terrains = id_map(
        try!(read_terrains(stream)),
        &|t: &Terrain| t.id);

    try!(read_terrain_borders(&mut terrain_block, stream));

    try!(stream.read_i32()); // Unknown pointer
    terrain_block.terrains_used = try!(stream.read_u16());
    terrain_block.borders_used = try!(stream.read_u16());
    terrain_block.max_terrain = try!(stream.read_i16());
    terrain_block.tile_width = try!(stream.read_i16());
    terrain_block.tile_height = try!(stream.read_i16());
    terrain_block.tile_half_height = try!(stream.read_i16());
    terrain_block.tile_half_width = try!(stream.read_i16());
    terrain_block.elevation_height = try!(stream.read_i16());
    terrain_block.current_row = try!(stream.read_i16());
    terrain_block.current_col = try!(stream.read_i16());
    terrain_block.block_begin_row = try!(stream.read_i16());
    terrain_block.block_end_row = try!(stream.read_i16());
    terrain_block.block_begin_col = try!(stream.read_i16());
    terrain_block.block_end_col = try!(stream.read_i16());

    try!(stream.read_u32()); // Unknown pointer
    try!(stream.read_u32()); // Unknown pointer
    terrain_block.any_frame_change = try!(stream.read_i8());
    terrain_block.map_visible = try!(stream.read_u8()) != 0;
    terrain_block.fog = try!(stream.read_u8()) != 0;

    try!(stream.seek(SeekFrom::Current(25))); // Skip 25 unknown bytes
    Ok(terrain_block)
}

fn read_tile_sizes<R: Read + Seek>(terrain_block: &mut TerrainBlock, stream: &mut R)
        -> EmpiresDbResult<()> {
    for _ in 0..TILE_TYPE_COUNT {
        let mut tile_size: TileSize = Default::default();
        tile_size.width = try!(stream.read_i16());
        tile_size.height = try!(stream.read_i16());
        tile_size.delta_y = try!(stream.read_i16());
        terrain_block.tile_sizes.push(tile_size);
    }
    Ok(())
}

fn read_terrains<R: Read + Seek>(stream: &mut R) -> EmpiresDbResult<Vec<Terrain>> {
    let mut terrains = Vec::new();

    let terrain_count = 32;
    for i in 0..terrain_count {
        let mut terrain: Terrain = Default::default();

        terrain.id = TerrainId(i as isize);
        terrain.enabled = try!(stream.read_u8()) != 0;
        try!(stream.read_i8()); // Unused (always zero)
        terrain.name = try!(stream.read_sized_str(13));
        terrain.short_name = try!(stream.read_sized_str(13));
        terrain.slp_id = SlpFileId(try!(stream.read_i32()) as isize);
        try!(stream.read_u32()); // Unknown
        terrain.sound_group_id = SoundGroupId(try!(stream.read_i32()) as isize);

        for i in 0..3 {
            terrain.colors[i] = try!(stream.read_u8());
        }
        for i in 0..2 {
            terrain.cliff_colors[i] = try!(stream.read_u8());
        }
        terrain.pass_terrain_id = TerrainId(try!(stream.read_i8()) as isize);
        terrain.impass_terrain_id = TerrainId(try!(stream.read_i8()) as isize);

        terrain.animated = try!(stream.read_u8()) != 0;
        terrain.animation_frames = try!(stream.read_i16());
        terrain.pause_frames = try!(stream.read_i16());
        terrain.frame_interval = try!(stream.read_f32());
        terrain.pause_between_loops = try!(stream.read_f32());
        terrain.frame = try!(stream.read_i16());
        terrain.draw_frame = try!(stream.read_i16());
        terrain.animate_last = try!(stream.read_f32());
        terrain.frame_changed = try!(stream.read_i8());
        try!(stream.read_i8()); // Unused; always zero

        terrain.elevation_graphics =
            try!(stream.read_array(TILE_TYPE_COUNT, |c| read_frame_data(c)));

        terrain.terrain_to_draw = TerrainId(try!(stream.read_i16()) as isize);
        terrain.terrain_width = try!(stream.read_i16());
        terrain.terrain_height = try!(stream.read_i16());

        let terrain_borders = try!(stream.read_array(terrain_count, |c| c.read_i16()));
        for (index, terrain_border) in terrain_borders.iter().enumerate() {
            terrain.terrain_borders.insert(TerrainId(index as isize), *terrain_border);
        }

        try!(read_terrain_units(&mut terrain.terrain_units, stream));
        try!(stream.read_u16()); // Unknown

        terrains.push(terrain);
    }
    Ok(terrains)
}

fn read_terrain_units<R: Read>(terrain_units: &mut Vec<TerrainUnit>, stream: &mut R)
        -> EmpiresDbResult<()> {
    let (ids, densities, priorities) = (
        try!(stream.read_array(MAX_TERRAIN_UNITS, |c| c.read_i16())),
        try!(stream.read_array(MAX_TERRAIN_UNITS, |c| c.read_i16())),
        try!(stream.read_array(MAX_TERRAIN_UNITS, |c| c.read_i8()))
    );

    let terrain_units_used = try!(stream.read_i16()) as usize;
    if terrain_units_used > MAX_TERRAIN_UNITS {
        return Err(EmpiresDbError::BadFile("invalid number of terrain units used"))
    }

    for i in 0..terrain_units_used {
        let mut unit: TerrainUnit = Default::default();
        unit.unit_id = UnitId(ids[i] as isize);
        unit.density = densities[i];
        unit.priority = priorities[i];
        terrain_units.push(unit);
    }
    Ok(())
}

fn read_frame_data<R: Read>(stream: &mut R) -> EmpiresDbResult<TerrainFrameData> {
    let mut frame_data: TerrainFrameData = Default::default();
    frame_data.frame_count = try!(stream.read_i16());
    frame_data.angle_count = try!(stream.read_i16());
    frame_data.shape_id = try!(stream.read_i16());
    Ok(frame_data)
}

fn read_terrain_borders<R: Read + Seek>(terrain_block: &mut TerrainBlock, stream: &mut R)
        -> EmpiresDbResult<()> {
    let terrain_border_count = 16;
    for _ in 0..terrain_border_count {
        let mut border: TerrainBorder = Default::default();

        border.enabled = try!(stream.read_u8()) != 0;
        try!(stream.read_i8()); // Unused (always zero)
        border.name = try!(stream.read_sized_str(13));
        border.short_name = try!(stream.read_sized_str(13));
        border.slp_id = SlpFileId(try!(stream.read_i32()) as isize);
        try!(stream.read_u32()); // Unknown
        border.sound_group_id = SoundGroupId(try!(stream.read_i32()) as isize);

        for i in 0..3 {
            border.colors[i] = try!(stream.read_u8());
        }

        border.animated = try!(stream.read_u8()) != 0;
        border.animation_frames = try!(stream.read_i16());
        border.pause_frames = try!(stream.read_i16());
        border.frame_interval = try!(stream.read_f32());
        border.pause_between_loops = try!(stream.read_f32());
        border.frame = try!(stream.read_i16());
        border.draw_frame = try!(stream.read_i16());
        border.animate_last = try!(stream.read_f32());
        border.frame_changed = try!(stream.read_i8());
        try!(stream.read_i8()); // Unused; always zero

        border.borders = try!(stream.read_array(TILE_TYPE_COUNT, |outer_stream| {
            outer_stream.read_array(12, |inner_stream| {
                read_frame_data(inner_stream)
            })
        }));
        if !border.enabled {
            // Don't bother saving all of this data if we're not enabled
            border.borders.clear();
        }

        try!(stream.read_i16()); // Unused; always zero
        border.underlay_terrain_id = TerrainId(try!(stream.read_i16()) as isize);
        border.border_style = try!(stream.read_i16());

        terrain_block.terrain_borders.push(border);
    }
    Ok(())
}
