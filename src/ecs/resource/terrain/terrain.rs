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

use super::border::BorderMatrix;
use super::dir;
use super::elevation::ElevationMatrix;

use scn;
use identifier::{TerrainBorderId, TerrainId};
use dat;

use nalgebra::Vector3;

use std::cmp;

#[derive(Debug)]
pub struct Tile {
    pub terrain_id: u8,
    pub elevation: u8,
}

impl Tile {
    fn from(scn_tile: &scn::MapTile) -> Tile {
        Tile {
            terrain_id: scn_tile.terrain_id,
            elevation: scn_tile.elevation,
        }
    }
}

#[derive(Debug)]
pub struct BlendInfo {
    pub terrain_id: TerrainId,
    pub elevation: u8,
    pub border_id: Option<TerrainBorderId>,
    pub border_style: i16,
    pub border_matrix: BorderMatrix,
    pub elevation_matrix: ElevationMatrix,
}

pub struct Terrain {
    width: i32,
    height: i32,
    tiles: Vec<Tile>,
    empires: dat::EmpiresDbRef,
}

impl Terrain {
    pub fn from(scn_map: &scn::Map, empires: dat::EmpiresDbRef) -> Terrain {
        Terrain {
            width: scn_map.width as i32,
            height: scn_map.height as i32,
            tiles: scn_map.tiles.iter().map(|t| Tile::from(t)).collect(),
            empires: empires,
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn tile_at<'a>(&'a self, world_coord: Vector3<f32>) -> &'a Tile {
        self.tile_at_row_col(world_coord.y.round() as i32, world_coord.x.round() as i32)
    }

    fn tile_at_row_col<'a>(&'a self, row: i32, col: i32) -> &'a Tile {
        // Clamp the row/col
        let row = cmp::max(0, cmp::min(row, self.height - 1));
        let col = cmp::max(0, cmp::min(col, self.width - 1));

        let tile_index = (row * self.width + col) as usize;
        &self.tiles[tile_index]
    }

    fn tile_at_relative<'a>(&'a self, row: i32, col: i32, direction: usize) -> &'a Tile {
        let row = row + (direction as i32 / 3) - 1;
        let col = col + (direction as i32 % 3) - 1;
        self.tile_at_row_col(row, col)
    }

    /// Returns tile blend info at the requested row/col while clamping at the edges of the terrain
    /// so that the request cannot go out of bounds. This means that the tiles at the edges
    /// will repeat indefinitely if you query outside of the terrain.
    pub fn blend_at(&self, row: i32, col: i32) -> BlendInfo {
        let tile = &self.tile_at_row_col(row, col);
        let terrain = &self.terrain_block().terrains[&TerrainId(tile.terrain_id as isize)];

        // Calculate the border matrix and border id
        let mut border_id = None;
        let mut border_style = 0;
        let mut border_matrix = BorderMatrix::new();
        if let Some((border, border_terrain_id)) = self.determine_border(terrain, row, col) {
            border_id = Some(border.id.as_usize() as u8);
            border_style = border.border_style;
            for direction in &dir::ALL {
                let dir_terrain_id = self.tile_at_relative(row, col, *direction)
                    .terrain_id as usize;
                border_matrix.set_at(*direction,
                                     dir_terrain_id == border_terrain_id.as_usize(),
                                     dir_terrain_id == terrain.id.as_usize());
            }
        }

        // Calculate the elevation matrix
        let mut elevation_matrix = ElevationMatrix::new();
        for direction in &dir::ALL {
            let elevation = self.tile_at_relative(row, col, *direction).elevation;

            let mut elev_dir = 1;
            if elevation == tile.elevation {
                elev_dir = 0;
            } else if elevation < tile.elevation {
                elev_dir = -1;
            }
            elevation_matrix.set_elevation_at(*direction, elev_dir);
        }

        BlendInfo {
            terrain_id: TerrainId(tile.terrain_id as isize),
            elevation: tile.elevation,
            border_id: border_id.map(|b| TerrainBorderId(b as isize)),
            border_style: border_style,
            border_matrix: border_matrix,
            elevation_matrix: elevation_matrix,
        }
    }

    fn determine_border(&self,
                        center_terrain: &dat::Terrain,
                        row: i32,
                        col: i32)
                        -> Option<(&dat::TerrainBorder, TerrainId)> {
        for direction in &dir::ALL {
            let border_terrain_id = TerrainId(self.tile_at_relative(row, col, *direction)
                .terrain_id as isize);
            if center_terrain.id != border_terrain_id {
                let border_id = center_terrain.terrain_borders[&border_terrain_id].as_usize();
                let border = &self.terrain_block().terrain_borders[border_id];
                if border.enabled {
                    return Some((border, border_terrain_id));
                }
            }
        }
        None
    }

    #[inline]
    fn terrain_block<'a>(&'a self) -> &'a dat::TerrainBlock {
        &self.empires.terrain_block
    }
}
