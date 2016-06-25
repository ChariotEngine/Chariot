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

#[derive(Default, Debug)]
pub struct RandomMapHeader {
    script_id: i32,
    border_sw: i32,
    border_nw: i32,
    border_ne: i32,
    border_se: i32,
    border_usage: i32,
    water_shape: i32,
    non_base_terrain: i32,
    base_zone_coverage: i32,
    base_zone_count: u32,
    terrain_count: u32,
    unit_count: u32,
}

impl RandomMapHeader {
    pub fn new() -> RandomMapHeader {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct BaseZone {
    base_terrain: i32,
    space_between_players: i32,
    start_area_radius: i32,
}

impl BaseZone {
    pub fn new() -> BaseZone {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct MapTerrain {
    proportion: i32,
    terrain: i32,
    clump_count: i32,
    spacing_to_other_terrains: i32,
    placement_zone: i32,
}

impl MapTerrain {
    pub fn new() -> MapTerrain {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct MapUnit {
    unit: i32,
    host_terrain: i32,
    objects_per_group: i32,
    fluctuation: i32,
    groups_per_player: i32,
    group_radius: i32,
    own_at_start: i32,
    set_place_for_all_players: i32,
    min_distance_to_players: i32,
    max_distance_to_players: i32,
}

impl MapUnit {
    pub fn new() -> MapUnit {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct RandomMap {
    border_sw: i32,
    border_nw: i32,
    border_ne: i32,
    border_se: i32,
    border_usage: i32,
    water_shape: i32,
    non_base_terrain: i32,
    base_zone_coverage: i32,
    base_zones: Vec<BaseZone>,
    terrains: Vec<MapTerrain>,
    units: Vec<MapUnit>,
}

impl RandomMap {
    pub fn new() -> RandomMap {
        Default::default()
    }
}

impl EmpiresDb {
    pub fn read_random_maps<R: Read + Seek>(&mut self, cursor: &mut R) -> EmpiresDbResult<()> {
        let random_map_count = try!(cursor.read_u32()) as usize;
        try!(cursor.read_u32()); // Unused: random map pointer

        for i in 0..random_map_count {
            let header = try!(EmpiresDb::read_random_map_header(cursor));
            // TODO: do something with header? Not sure how it's useful yet
        }
        for _ in 0..random_map_count {
            self.random_maps.push(try!(EmpiresDb::read_random_map(cursor)));
        }
        Ok(())
    }

    fn read_map_unit<R: Read>(cursor: &mut R) -> io::Result<MapUnit> {
        let mut unit = MapUnit::new();
        unit.unit = try!(cursor.read_i32());
        unit.host_terrain = try!(cursor.read_i32());
        try!(cursor.read_i32()); // Unknown
        unit.objects_per_group = try!(cursor.read_i32());
        unit.fluctuation = try!(cursor.read_i32());
        unit.groups_per_player = try!(cursor.read_i32());
        unit.group_radius = try!(cursor.read_i32());
        unit.own_at_start = try!(cursor.read_i32());
        unit.set_place_for_all_players = try!(cursor.read_i32());
        unit.min_distance_to_players = try!(cursor.read_i32());
        unit.max_distance_to_players = try!(cursor.read_i32());
        Ok(unit)
    }

    fn read_map_terrain<R: Read>(cursor: &mut R) -> io::Result<MapTerrain> {
        let mut terrain = MapTerrain::new();
        terrain.proportion = try!(cursor.read_i32());
        terrain.terrain = try!(cursor.read_i32());
        terrain.clump_count = try!(cursor.read_i32());
        terrain.spacing_to_other_terrains = try!(cursor.read_i32());
        terrain.placement_zone = try!(cursor.read_i32());
        try!(cursor.read_i32()); // Unknown
        Ok(terrain)
    }

    fn read_base_zone<R: Read + Seek>(cursor: &mut R) -> io::Result<BaseZone> {
        let mut zone = BaseZone::new();
        try!(cursor.read_u32()); // Unknown
        zone.base_terrain = try!(cursor.read_i32());
        zone.space_between_players = try!(cursor.read_i32());
        try!(cursor.seek(SeekFrom::Current(20))); // 20 unknown bytes
        zone.start_area_radius = try!(cursor.read_i32());
        try!(cursor.seek(SeekFrom::Current(8))); // 8 unknown bytes
        Ok(zone)
    }

    fn read_random_map<R: Read + Seek>(cursor: &mut R) -> EmpiresDbResult<RandomMap> {
        let mut map = RandomMap::new();
        map.border_sw = try!(cursor.read_i32());
        map.border_nw = try!(cursor.read_i32());
        map.border_ne = try!(cursor.read_i32());
        map.border_se = try!(cursor.read_i32());
        map.border_usage = try!(cursor.read_i32());
        map.water_shape = try!(cursor.read_i32());
        map.non_base_terrain = try!(cursor.read_i32());
        map.base_zone_coverage = try!(cursor.read_i32());
        try!(cursor.read_i32()); // Unknown

        let base_zone_count = try!(cursor.read_u32()) as usize;
        try!(cursor.read_u32()); // Unused: Base zone pointer
        map.base_zones = try!(cursor.read_array(base_zone_count, |c| EmpiresDb::read_base_zone(c)));

        let terrain_count = try!(cursor.read_u32()) as usize;
        try!(cursor.read_u32()); // Unused: Terrain pointer
        map.terrains = try!(cursor.read_array(terrain_count, |c| EmpiresDb::read_map_terrain(c)));

        let unit_count = try!(cursor.read_u32()) as usize;
        try!(cursor.read_u32()); // Unused: Unit pointer
        map.units = try!(cursor.read_array(unit_count, |c| EmpiresDb::read_map_unit(c)));

        let unknown_count = try!(cursor.read_u32()) as i64;
        try!(cursor.read_u32()); // Unused: Unknown pointer
        try!(cursor.seek(SeekFrom::Current(24 * unknown_count))); // Skip unknown data

        Ok(map)
    }

    fn read_random_map_header<R: Read + Seek>(cursor: &mut R) -> EmpiresDbResult<RandomMapHeader> {
        let mut header = RandomMapHeader::new();
        header.script_id = try!(cursor.read_i32());
        header.border_sw = try!(cursor.read_i32());
        header.border_nw = try!(cursor.read_i32());
        header.border_ne = try!(cursor.read_i32());
        header.border_se = try!(cursor.read_i32());
        header.border_usage = try!(cursor.read_i32());
        header.water_shape = try!(cursor.read_i32());
        header.non_base_terrain = try!(cursor.read_i32());
        header.base_zone_coverage = try!(cursor.read_i32());
        try!(cursor.read_i32()); // Unknown

        header.base_zone_count = try!(cursor.read_u32());
        try!(cursor.read_i32()); // Unused: Base zone pointer

        header.terrain_count = try!(cursor.read_u32());
        try!(cursor.read_i32()); // Unused: Terrain pointer

        header.unit_count = try!(cursor.read_u32());
        try!(cursor.read_i32()); // Unused: Unit pointer

        try!(cursor.read_i32()); // Unknown count
        try!(cursor.read_i32()); // Unused: unknown pointer
        Ok(header)
    }
}
