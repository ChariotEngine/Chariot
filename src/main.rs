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

#[macro_use]
extern crate lazy_static;

mod terrain;

use terrain::TerrainCursor;

use types::Point;
use resource::{DrsKey, ShapeKey};
use dat::TerrainId;

use std::collections::HashMap;
use std::process;
use std::cmp;

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
    map_borders: Vec<[u16; 4]>,
    tiles: Vec<TerrainTile>,
    borders: Vec<TerrainBorderTile>,
}

impl Terrain {
    pub fn new(map: &scn::Map, terrain_block: &dat::TerrainBlock) -> Terrain {
        let mut terrain = Terrain {
            width: map.width as isize,
            height: map.height as isize,
            map: vec![0u16; (map.width * map.height) as usize],
            map_borders: vec![[0; 4]; (map.width * map.height) as usize],
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

            if let Some((border_id, border_indices)) = cursor.border() {
                let mut indices: Vec<u16> = Vec::new();
                for border_index in border_indices {
                    let border_key = TerrainTileKey::new(border_id as u8, tile.elevation, *border_index as i16);
                    if !border_map.get(&border_key).is_some() {
                        let border = &terrain_block.terrain_borders[border_id as usize];
                        // TODO: account for elevation
                        let frame_data = &border.borders[0][*border_index as usize];
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
                    indices.push(*border_map.get(&border_key).unwrap());
                }
                indices.resize(4, 0);
                terrain.map_borders[cursor.index() as usize] = [indices[0], indices[1], indices[2], indices[3]];
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

    pub fn borders_at<'a>(&'a self, row: i32, col: i32) -> [Option<&'a TerrainBorderTile>; 4] {
        let border_indices = self.map_borders[self.index(row, col)];
        let map = &|index: usize| {
            if border_indices[index] > 0 {
                Some(&self.borders[(border_indices[index] - 1) as usize])
            } else {
                None
            }
        };
        [map(0), map(1), map(2), map(3)]
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

    // Temporary hardcoded camera offset
    // let camera_pos = Point::new(0, -300);
    let camera_pos = Point::new(126 * tile_half_width, -145 * tile_half_height);
    // let camera_pos = Point::new(256 * tile_half_width, -300);
    // let camera_pos = Point::new(126 * tile_half_width, 125 * tile_half_height);

    while media.is_open() {
        media.update();

        media.renderer().present();

        let map = &test_scn.map;
        for row in 0..(map.height as i32) {
            for col in 0..(map.width as i32) {
                let tile = terrain.tile_at(row, col);
                let borders = terrain.borders_at(row, col);
                let (x, y) = project_row_col(row, col, tile_half_width, tile_half_height);


                media.renderer().set_camera_position(&camera_pos);

                let frame_num = ((row + 1) * (col - row)) as usize % tile.frame_range.len();

                shape_manager.borrow_mut()
                    .get(&ShapeKey::new(DrsKey::Terrain, tile.slp_id, 0), media.renderer()).unwrap()
                    .render_frame(media.renderer(), tile.frame_range[frame_num] as usize, &Point::new(x, y));

                for border in &borders {
                    if let Some(border_tile) = *border {
                        let frame_num = ((row + 1) * (col - row)) as usize % border_tile.frame_range.len();
                        shape_manager.borrow_mut()
                            .get(&ShapeKey::new(DrsKey::Border, border_tile.slp_id, 0), media.renderer()).unwrap()
                            .render_frame(media.renderer(), border_tile.frame_range[frame_num] as usize, &Point::new(x, y));
                    }
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
