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

extern crate open_aoe_drs as drs;
extern crate open_aoe_slp as slp;
extern crate open_aoe_palette as palette;
extern crate open_aoe_dat as dat;
extern crate open_aoe_language as language;
extern crate open_aoe_scn as scn;
extern crate open_aoe_media as media;
extern crate open_aoe_resource as resource;
extern crate open_aoe_types as types;

use types::Point;
use resource::{DrsKey, ShapeKey};
use dat::TerrainId;

use std::collections::HashMap;
use std::process;
use std::cmp;

const DIR_W: usize = 0;
const DIR_NW: usize = 1;
const DIR_N: usize = 2;
const DIR_SW: usize = 3;
const DIR_NE: usize = 5;
const DIR_S: usize = 6;
const DIR_SE: usize = 7;
const DIR_E: usize = 8;

pub struct TerrainTile {
    pub terrain_id: u8,
    pub elevation: u8,
    pub slp_id: u32,
    pub frame_range: Vec<u32>,
}

pub struct TerrainBorderTile {
    pub border_id: u8,
    pub elevation: u8,
    pub slp_id: u32,
    pub frame_range: Vec<u32>,
}

pub struct TerrainCursor<'a> {
    terrain_block: &'a dat::TerrainBlock,
    tiles: &'a [scn::MapTile],
    width: isize,
    height: isize,
    index: isize,
}

impl<'a> TerrainCursor<'a> {
    pub fn new(terrain_block: &'a dat::TerrainBlock, tiles: &'a [scn::MapTile],
            width: isize, height: isize) -> TerrainCursor<'a> {
        TerrainCursor {
            terrain_block: terrain_block,
            tiles: tiles,
            width: width,
            height: height,
            index: 0,
        }
    }

    pub fn current(&self) -> &'a scn::MapTile {
        &self.tiles[self.index as usize]
    }

    pub fn index(&self) -> isize {
        self.index
    }

    pub fn row(&self) -> isize {
        self.index / self.width
    }

    pub fn col(&self) -> isize {
        self.index % self.width
    }

    fn at(&self, row: isize, col: isize) -> &'a scn::MapTile {
        // Clamp the row/col
        let row = cmp::max(0, cmp::min(row, self.height - 1));
        let col = cmp::max(0, cmp::min(col, self.width - 1));
        let index = row * self.width + col;
        &self.tiles[index as usize]
    }

    // Returns Option<(border_id, border_index)>
    fn border(&self) -> Option<(u16, u16)> {
        let cur_terrain_id = self.current().terrain_id;
        let cur_terrain = &self.terrain_block.terrains[&TerrainId(cur_terrain_id as isize)];

        let mut matrix = [0xFFu8; 9];
        let (cur_row, cur_col) = (self.row(), self.col());
        for row in 0..3 {
            for col in 0..3 {
                matrix[(3 * row + col) as usize] =
                    self.at(cur_row + row - 1, cur_col + col - 1).terrain_id;
            }
        }

        let mut visited = [0xFFu8; 9];
        let mut bordering_terrain_id = 0;
        let mut border_id = 0;
        let mut border = None;
        for i in 0..9 {
            if cur_terrain_id != matrix[i] && !visited.contains(&matrix[i]) {
                visited[i] = matrix[i];
                bordering_terrain_id = matrix[i];
                border_id = cur_terrain.terrain_borders[&TerrainId(matrix[i] as isize)].as_isize();
                border = Some(&self.terrain_block.terrain_borders[border_id as usize]);
                if border.unwrap().enabled {
                    break;
                }
            }
        }

        if border.is_some() && border.unwrap().enabled {
            for i in 0..9 {
                matrix[i] = if matrix[i] == bordering_terrain_id { 2 }
                    else if matrix[i] == cur_terrain_id { 1 }
                    else { 0 };
            }


            let b_w = matrix[DIR_W] == 2;
            let b_nw = matrix[DIR_NW] == 2;
            let b_n = matrix[DIR_N] == 2;
            let b_ne = matrix[DIR_NE] == 2;
            let b_e = matrix[DIR_E] == 2;
            let b_se = matrix[DIR_SE] == 2;
            let b_s = matrix[DIR_S] == 2;
            let b_sw = matrix[DIR_SW] == 2;

            let c_w = matrix[DIR_W] == 1;
            let c_nw = matrix[DIR_NW] == 1;
            let c_n = matrix[DIR_N] == 1;
            let c_ne = matrix[DIR_NE] == 1;
            let c_e = matrix[DIR_E] == 1;
            let c_se = matrix[DIR_SE] == 1;
            let c_s = matrix[DIR_S] == 1;
            let c_sw = matrix[DIR_SW] == 1;

            let border_style = border.unwrap().border_style;
            let border_index: u16;
            if border_style == 0 {
                border_index =
                         if b_w && b_nw && b_n && b_ne && b_e && c_sw && c_se { 1 }
                    else if b_s && b_sw && b_w && b_nw && b_n && c_se && c_ne { 0 }
                    else if b_s && b_se && b_e && b_ne && b_n && c_sw && c_nw { 3 }
                    else if b_w && b_sw && b_s && b_se && b_e && c_nw && c_ne { 2 }

                    else if b_w && b_nw && b_n && c_sw && c_ne { 8 }
                    else if b_s && b_se && b_e && c_sw && c_ne { 9 }
                    else if b_n && b_ne && b_e && c_nw && c_se { 11 }
                    else if b_w && b_sw && b_s && c_nw && c_se { 10 }

                    else if b_nw && (b_w || b_n) && c_sw && c_ne { 8 }
                    else if b_se && (b_s || b_e) && c_sw && c_ne { 9 }
                    else if b_ne && (b_n || b_e) && c_nw && c_se { 11 }
                    else if b_sw && (b_w || b_s) && c_nw && c_se { 10 }

                    else if b_s && c_sw && c_se { 6 }
                    else if b_e && c_ne && c_se { 7 }
                    else if b_n && c_ne && c_nw { 5 }
                    else if b_w && c_sw && c_nw { 4 }

                    else if b_e && b_ne && b_se && c_s && c_n { 3 }
                    else if b_n && b_nw && b_ne && c_w && c_e { 1 }
                    else if b_w && b_sw && b_nw && c_s && c_n { 0 }
                    else if b_s && b_sw && b_se && c_w && c_e { 2 }

                    else {
                        println!("FAILED: bs: {}, r: {}, c: {}, t: {}:\n  {:?}\n  {:?}\n  {:?}",
                            border_style, self.row(), self.col(), cur_terrain_id,
                            [matrix[DIR_W], matrix[DIR_NW], matrix[DIR_N]],
                            [matrix[DIR_SW], matrix[4], matrix[DIR_NE]],
                            [matrix[DIR_S], matrix[DIR_SE], matrix[DIR_E]]);
                        return None
                    };
            } else if border_style == 1 {
                // TODO: Refactor so that multiple border indices can be
                // returned for border_style 2
                border_index =
                         if b_w && b_nw && b_n && c_sw && c_ne { 0 }
                    else if b_s && b_se && b_e && c_sw && c_ne { 3 }
                    else if b_n && b_ne && b_e && c_nw && c_se { 1 }
                    else if b_w && b_sw && b_s && c_nw && c_se { 2 }

                    else if b_nw && (b_w || b_n) && c_sw && c_ne { 0 }
                    else if b_se && (b_s || b_e) && c_sw && c_ne { 3 }
                    else if b_ne && (b_n || b_e) && c_nw && c_se { 1 }
                    else if b_sw && (b_w || b_s) && c_nw && c_se { 2 }

                    else {
                        println!("FAILED: bs: {}, r: {}, c: {}, t: {}:\n  {:?}\n  {:?}\n  {:?}",
                            border_style, self.row(), self.col(), cur_terrain_id,
                            [matrix[DIR_W], matrix[DIR_NW], matrix[DIR_N]],
                            [matrix[DIR_SW], matrix[4], matrix[DIR_NE]],
                            [matrix[DIR_S], matrix[DIR_SE], matrix[DIR_E]]);
                        return None
                    }
            } else {
                println!("Unrecognized border style: {}", border_style);
                return None
            }

            Some((border_id as u16, border_index))
        } else {
            None
        }
    }

    pub fn has_next(&self) -> bool {
        return self.index < self.width * self.height
    }

    pub fn next(&mut self) {
        self.index += 1;
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct TerrainTileKey {
    pub terrain_id: u8,
    pub elevation: u8,
    pub border_index: i16,
}

impl TerrainTileKey {
    fn new(terrain_id: u8, elevation: u8, border_index: i16) -> TerrainTileKey {
        TerrainTileKey {
            terrain_id: terrain_id,
            elevation: elevation,
            border_index: border_index,
        }
    }
}

// TODO: Move terrain code to a separate module
pub struct Terrain {
    width: isize,
    height: isize,
    map: Vec<u16>,
    map_borders: Vec<u16>,
    tiles: Vec<TerrainTile>,
    borders: Vec<TerrainBorderTile>,
}

impl Terrain {
    pub fn new(map: &scn::Map, terrain_block: &dat::TerrainBlock) -> Terrain {
        let mut terrain = Terrain {
            width: map.width as isize,
            height: map.height as isize,
            map: vec![0u16; (map.width * map.height) as usize],
            map_borders: vec![0u16; (map.width * map.height) as usize],
            tiles: Vec::new(),
            borders: Vec::new(),
        };

        let mut tile_map: HashMap<TerrainTileKey, u16> = HashMap::new();
        let mut border_map: HashMap<TerrainTileKey, u16> = HashMap::new();

        let mut cursor = TerrainCursor::new(terrain_block, &map.tiles, terrain.width, terrain.height);
        while cursor.has_next() {
            let tile = cursor.current();

            let key = TerrainTileKey::new(tile.terrain_id, tile.elevation, -1);
            if !tile_map.get(&key).is_some() {
                let terrain_def = &terrain_block.terrains[&TerrainId(tile.terrain_id as isize)];
                // TODO: account for elevation
                let elevation_graphic = &terrain_def.elevation_graphics[0];
                let start_frame = elevation_graphic.frame_id.as_isize() as u32;
                let end_frame = start_frame + cmp::max(1, elevation_graphic.frame_count) as u32;
                let frames = (start_frame..end_frame).collect();

                let slp_id = if terrain_def.slp_id.as_isize() == -1 {
                    let alt_terrain_id = &terrain_def.terrain_to_draw;
                    let alt_terrain_def = &terrain_block.terrains[alt_terrain_id];
                    alt_terrain_def.slp_id.as_isize() as u32
                } else {
                    terrain_def.slp_id.as_isize() as u32
                };

                let terrain_tile = TerrainTile {
                    terrain_id: tile.terrain_id,
                    elevation: tile.elevation,
                    slp_id: slp_id,
                    frame_range: frames,
                };
                terrain.tiles.push(terrain_tile);
                tile_map.insert(key, (terrain.tiles.len() - 1) as u16);
            }

            if let Some((border_id, border_index)) = cursor.border() {
                let border_key = TerrainTileKey::new(border_id as u8, tile.elevation, border_index as i16);
                if !border_map.get(&border_key).is_some() {
                    let border = &terrain_block.terrain_borders[border_id as usize];
                    // TODO: account for elevation
                    let frame_data = &border.borders[0][border_index as usize];
                    let start_frame = frame_data.frame_id.as_isize() as u32;
                    let end_frame = start_frame + cmp::max(1, frame_data.frame_count) as u32;
                    let frames = (start_frame..end_frame).collect();

                    let border_tile = TerrainBorderTile {
                        border_id: border_id as u8,
                        elevation: tile.elevation,
                        slp_id: border.slp_id.as_isize() as u32,
                        frame_range: frames,
                    };
                    terrain.borders.push(border_tile);
                    border_map.insert(border_key, terrain.borders.len() as u16);
                }
                terrain.map_borders[cursor.index() as usize] = *border_map.get(&border_key).unwrap();
            }

            terrain.map[cursor.index() as usize] = *tile_map.get(&key).unwrap();
            cursor.next();
        }

        terrain
    }

    fn index(&self, row: i32, col: i32) -> usize {
        (row * self.width as i32 + col) as usize
    }

    pub fn tile_at<'a>(&'a self, row: i32, col: i32) -> &'a TerrainTile {
        let tile_index = self.map[self.index(row, col)];
        &self.tiles[tile_index as usize]
    }

    pub fn border_at<'a>(&'a self, row: i32, col: i32) -> Option<&'a TerrainBorderTile> {
        let border_index = self.map_borders[self.index(row, col)];
        if border_index > 0 {
            Some(&self.borders[(border_index - 1) as usize])
        } else {
            None
        }
    }
}

fn main() {
    let drs_manager = resource::DrsManager::new();
    if let Err(err) = drs_manager.borrow_mut().preload() {
        println!("Failed to preload DRS archives: {}", err);
        process::exit(1);
    }

    let shape_manager = match resource::ShapeManager::new(drs_manager.clone()) {
        Ok(shape_manager) => shape_manager,
        Err(err) => {
            println!("Failed to initialize the shape manager: {}", err);
            process::exit(1);
        }
    };

    let empires = dat::EmpiresDb::read_from_file("data/empires.dat").expect("empires.dat");
    println!("Loaded empires.dat");

    let test_scn = scn::Scenario::read_from_file("data/test2.scn").expect("test2.scn");
    println!("Loaded test2.scn");

    let mut media = match media::create_media(1024, 768, "OpenAOE") {
        Ok(media) => media,
        Err(err) => {
            println!("Failed to create media window: {}", err);
            process::exit(1);
        }
    };

    let tile_half_width = empires.terrain_block.tile_half_width as i32;
    let tile_half_height = empires.terrain_block.tile_half_height as i32;

    let terrain = Terrain::new(&test_scn.map, &empires.terrain_block);

    while media.is_open() {
        media.update();

        media.renderer().present();

        let map = &test_scn.map;
        for row in 0..(map.height as i32) {
            for col in 0..(map.width as i32) {
                let tile = terrain.tile_at(row, col);
                let border = terrain.border_at(row, col);
                let (x, y) = project_row_col(row, col, tile_half_width, tile_half_height);

                // Temporary hardcoded camera offset
                let (x, y) = (x - 126 * tile_half_width, y + 145 * tile_half_height);

                let frame_num = (row * col) as usize % tile.frame_range.len();

                shape_manager.borrow_mut()
                    .get(&ShapeKey::new(DrsKey::Terrain, tile.slp_id, 0), media.renderer()).unwrap()
                    .render_frame(media.renderer(), tile.frame_range[frame_num] as usize, &Point::new(x, y));

                if let Some(border_tile) = border {
                    shape_manager.borrow_mut()
                        .get(&ShapeKey::new(DrsKey::Border, border_tile.slp_id, 0), media.renderer()).unwrap()
                        .render_frame(media.renderer(), border_tile.frame_range[0] as usize, &Point::new(x, y));
                }
            }
        }
    }
}

/// Projects the row/col pair into isometric (x, y) coordinates
fn project_row_col(row: i32, col: i32, tile_half_width: i32, tile_half_height: i32) -> (i32, i32) {
    (
        (row + col) * tile_half_width,
        (row - col) * tile_half_height - tile_half_height,
    )
}
