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

use super::terrain_blender::TerrainBlender;
use super::terrain_blender::BlendInfo;
use super::border::BorderMatch;

use dat;
use media::Renderer;
use resource::{ShapeManager, ShapeKey, DrsKey};
use types::{Point, Rect};
use identifier::{SlpFileId, TerrainId, TerrainBorderId, PlayerColorId};

use std::collections::HashMap;
use std::cmp;

pub struct Tile<T> {
    pub id: T,
    pub slp_id: SlpFileId,
    pub frame_range: Vec<u32>,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct TileKey<T> {
    pub id: T,
    pub index: u16,
    pub elevation_index: u8,
}

impl<T> TileKey<T> {
    fn new(id: T, index: u16, elevation_index: u8) -> TileKey<T> {
        TileKey {
            id: id,
            index: index,
            elevation_index: elevation_index,
        }
    }
}

pub struct TerrainRenderer<'a> {
    terrain_block: &'a dat::TerrainBlock,
    tiles: HashMap<TileKey<TerrainId>, Tile<TerrainId>>,
    borders: HashMap<TileKey<TerrainBorderId>, Tile<TerrainBorderId>>,
}

impl<'a> TerrainRenderer<'a> {
    pub fn new(terrain_block: &'a dat::TerrainBlock) -> TerrainRenderer {
        TerrainRenderer {
            terrain_block: terrain_block,
            tiles: HashMap::new(),
            borders: HashMap::new(),
        }
    }

    pub fn render(&mut self,
                  renderer: &mut Renderer,
                  shape_manager: &mut ShapeManager,
                  terrain_blender: &TerrainBlender,
                  area: Rect) {
        for row in area.y..(area.y + area.h) {
            for col in (area.x..(area.x + area.w)).rev() {
                let blended_tile = terrain_blender.blend_at(row as isize, col as isize);
                let elevation_index = self.resolve_elevation_index(&blended_tile);
                let elevation = blended_tile.elevation;

                let tile_key = TileKey::new(blended_tile.terrain_id, 0, elevation_index);
                {
                    if !self.tiles.get(&tile_key).is_some() {
                        let tile = self.resolve_tile(&blended_tile, elevation_index);
                        self.tiles.insert(tile_key, tile);
                    }

                    let tile = self.tiles.get(&tile_key).unwrap();
                    self.render_tile(renderer,
                                     shape_manager,
                                     DrsKey::Terrain,
                                     &tile,
                                     elevation,
                                     row,
                                     col);
                }

                if blended_tile.border_id.is_some() {
                    match BorderMatch::find_match(blended_tile.border_style,
                                                  blended_tile.border_matrix) {
                        Some(border_match) => {
                            self.render_borders(renderer,
                                                shape_manager,
                                                blended_tile.border_id.unwrap(),
                                                &border_match.border_indices,
                                                elevation_index,
                                                elevation,
                                                row,
                                                col)
                        }
                        None => {
                            println!("Terrain border failed: bs: {}, r: {}, c: {}\n{:?}\n",
                                     blended_tile.border_style,
                                     row,
                                     col,
                                     blended_tile.border_matrix);
                        }
                    }
                }
            }
        }
    }

    fn render_tile<T>(&self,
                      renderer: &mut Renderer,
                      shape_manager: &mut ShapeManager,
                      drs_key: DrsKey,
                      tile: &Tile<T>,
                      elevation: u8,
                      row: i32,
                      col: i32) {
        let (x, y) = self.project_row_col(row, col, elevation);
        let frame_num = ((row + 1) * (col - row)) as usize % tile.frame_range.len();

        shape_manager.get(&ShapeKey::new(drs_key, tile.slp_id, PlayerColorId(0)),
                 renderer)
            .expect("failed to get shape for terrain rendering")
            .render_frame(renderer,
                          tile.frame_range[frame_num] as usize,
                          &Point::new(x, y));
    }

    fn render_borders(&mut self,
                      renderer: &mut Renderer,
                      shape_manager: &mut ShapeManager,
                      border_id: TerrainBorderId,
                      border_indices: &'static [u16],
                      elevation_index: u8,
                      elevation: u8,
                      row: i32,
                      col: i32) {
        for border_index in border_indices {
            let border_key = TileKey::new(border_id, *border_index, elevation_index);
            if !self.borders.get(&border_key).is_some() {
                let border = self.resolve_border(border_id, *border_index, elevation_index);
                self.borders.insert(border_key, border);
            }
            let border = self.borders.get(&border_key).unwrap();
            self.render_tile(renderer,
                             shape_manager,
                             DrsKey::Border,
                             border,
                             elevation,
                             row,
                             col);
        }
    }

    fn project_row_col(&self, row: i32, col: i32, elevation: u8) -> (i32, i32) {
        let tile_half_width = self.terrain_block.tile_half_width as i32;
        let tile_half_height = self.terrain_block.tile_half_height as i32;
        ((row + col) * tile_half_width,
         (row - col) * tile_half_height - tile_half_height - elevation as i32 * tile_half_height)
    }

    fn resolve_elevation_index(&self, _blended_tile: &BlendInfo) -> u8 {
        0 // TODO: elevation
    }

    fn resolve_tile(&self, blended_tile: &BlendInfo, elevation_index: u8) -> Tile<TerrainId> {
        let terrain_def = &self.terrain_block.terrains[&blended_tile.terrain_id];
        let elevation_graphic = &terrain_def.elevation_graphics[elevation_index as usize];
        let start_frame = elevation_graphic.frame_id.as_usize() as u32;
        let end_frame = start_frame + cmp::max(1, elevation_graphic.frame_count) as u32;
        let frames = (start_frame..end_frame).collect();

        let slp_id = if terrain_def.slp_id.as_isize() == -1 {
            let alt_terrain_id = &terrain_def.terrain_to_draw;
            let alt_terrain_def = &self.terrain_block.terrains[alt_terrain_id];
            alt_terrain_def.slp_id
        } else {
            terrain_def.slp_id
        };

        Tile {
            id: blended_tile.terrain_id,
            slp_id: slp_id,
            frame_range: frames,
        }
    }

    fn resolve_border(&self,
                      border_id: TerrainBorderId,
                      border_index: u16,
                      elevation_index: u8)
                      -> Tile<TerrainBorderId> {
        let border = &self.terrain_block.terrain_borders[border_id.as_usize()];
        let frame_data = &border.borders[elevation_index as usize][border_index as usize];
        let start_frame = frame_data.frame_id.as_usize() as u32;
        let end_frame = start_frame + cmp::max(1, frame_data.frame_count) as u32;
        let frames = (start_frame..end_frame).collect();

        Tile {
            id: border_id,
            slp_id: border.slp_id,
            frame_range: frames,
        }
    }
}
