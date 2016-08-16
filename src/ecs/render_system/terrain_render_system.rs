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


use dat;
use ecs::resource::{RenderCommands, Terrain, ViewProjector, Viewport};
use ecs::resource::terrain::{BlendInfo, BorderMatch, ElevationGraphic, ElevationMatch};
use identifier::{SlpFileId, TerrainBorderId, TerrainId};

use nalgebra::Vector2;
use resource::{DrsKey, RenderCommand, ShapeKey};
use specs;
use std::cmp;

use std::collections::HashMap;
use super::RenderSystem;
use types::{Fixed, Rect, Vector3};

const TERRAIN_LAYER: u16 = 0;

lazy_static! {
    static ref DEFAULT_ELEVATION: ElevationMatch =
        ElevationMatch::new(0, ElevationGraphic::new(0, 0.));
}

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

pub struct TerrainRenderSystem {
    empires: dat::EmpiresDbRef,
    tiles: HashMap<TileKey<TerrainId>, Tile<TerrainId>>,
    borders: HashMap<TileKey<TerrainBorderId>, Tile<TerrainBorderId>>,
}

impl RenderSystem for TerrainRenderSystem {
    fn render(&mut self, arg: specs::RunArg, _lerp: Fixed) {
        fetch_components!(arg, entities, [
            resource(projector: ViewProjector),
            resource(viewport: Viewport),
            mut resource(terrain: Terrain),
            mut resource(render_commands: RenderCommands),
        ]);

        let area = projector.calculate_visible_world_coords(&viewport, &*terrain);

        let (tile_half_width, tile_half_height) = self.empires.tile_half_sizes();
        let (tile_width, tile_height) = (tile_half_width * 2, tile_half_height * 2);

        let mut bounds = Rect::new();
        bounds.x = viewport.top_left_i32().x - tile_width;
        bounds.y = viewport.top_left_i32().y - tile_height;
        bounds.w = bounds.x + viewport.size.x + 2 * tile_width;
        bounds.h = bounds.y + viewport.size.y + 2 * tile_height;

        for row in area.y..(area.y + area.h) {
            for col in (area.x..(area.x + area.w)).rev() {
                if row >= 0 && row < terrain.width() && col >= 0 && col < terrain.height() {
                    let pos = projector.project(&Vector3::new(col.into(), row.into(), 0.into()));
                    if pos.x > bounds.x && pos.y > bounds.y && pos.x < bounds.w && pos.y < bounds.h {
                        self.blend_and_render_tile(&mut *render_commands, row, col, &mut terrain);
                    }
                }
            }
        }
    }
}

impl TerrainRenderSystem {
    pub fn new(empires: dat::EmpiresDbRef) -> TerrainRenderSystem {
        TerrainRenderSystem {
            empires: empires,
            tiles: HashMap::new(),
            borders: HashMap::new(),
        }
    }

    fn blend_and_render_tile(&mut self,
                             render_commands: &mut RenderCommands,
                             row: i32,
                             col: i32,
                             terrain: &mut Terrain) {
        let blended_tile = terrain.blend_at(row, col);
        let elevation_match = self.resolve_elevation(&blended_tile);

        let elevation_graphic = &elevation_match.elevation_graphic;
        let render_offset_y = elevation_graphic.render_offset_y + blended_tile.elevation as f32;

        let tile_key = TileKey::new(blended_tile.terrain_id, 0, elevation_graphic.index);
        {
            if !self.tiles.get(&tile_key).is_some() {
                let tile = self.resolve_tile(&blended_tile, elevation_graphic.index);
                self.tiles.insert(tile_key, tile);
            }

            let tile = self.tiles.get(&tile_key).unwrap();
            self.render_tile(render_commands,
                             DrsKey::Terrain,
                             &tile,
                             render_offset_y,
                             row,
                             col);
        }

        if blended_tile.border_id.is_some() {
            if let Some(border_match) = BorderMatch::find_match(blended_tile.border_style,
                                                                blended_tile.border_matrix) {
                self.render_borders(render_commands,
                                    blended_tile.border_id.unwrap(),
                                    &border_match.border_indices,
                                    elevation_graphic.index,
                                    render_offset_y,
                                    row,
                                    col)
            }
        }
    }

    fn render_tile<T>(&self,
                      render_commands: &mut RenderCommands,
                      drs_key: DrsKey,
                      tile: &Tile<T>,
                      render_offset_y: f32,
                      row: i32,
                      col: i32) {
        let (x, y) = self.project_row_col(row, col, render_offset_y);
        let frame_num = ((row + 1) * (col - row)) as usize % tile.frame_range.len();

        render_commands.push(RenderCommand::new_shape(TERRAIN_LAYER,
                                                      y,
                                                      ShapeKey::new(drs_key, tile.slp_id, 0.into()),
                                                      tile.frame_range[frame_num] as u16,
                                                      Vector2::new(x, y),
                                                      false,
                                                      false));
    }

    fn render_borders(&mut self,
                      render_commands: &mut RenderCommands,
                      border_id: TerrainBorderId,
                      border_indices: &'static [u16],
                      elevation_index: u8,
                      render_offset_y: f32,
                      row: i32,
                      col: i32) {
        for border_index in border_indices {
            let border_key = TileKey::new(border_id, *border_index, elevation_index);
            if !self.borders.get(&border_key).is_some() {
                let border = self.resolve_border(border_id, *border_index, elevation_index);
                self.borders.insert(border_key, border);
            }
            let border = self.borders.get(&border_key).unwrap();
            self.render_tile(render_commands,
                             DrsKey::Border,
                             border,
                             render_offset_y,
                             row,
                             col);
        }
    }

    fn project_row_col(&self, row: i32, col: i32, render_offset_y: f32) -> (i32, i32) {
        let (tile_half_width, tile_half_height) = self.empires.tile_half_sizes();
        let render_offset_y = (render_offset_y * tile_half_height as f32) as i32;
        ((row + col) * tile_half_width, (row - col) * tile_half_height - tile_half_height - render_offset_y)
    }

    fn resolve_elevation(&self, blended_tile: &BlendInfo) -> &'static ElevationMatch {
        match ElevationMatch::find_match(blended_tile.elevation_matrix) {
            Some(elevation_match) => elevation_match,
            None => {
                println!("Elevation match failed:\n{:?}",
                         blended_tile.elevation_matrix);
                &DEFAULT_ELEVATION
            }
        }
    }

    fn resolve_tile(&self, blended_tile: &BlendInfo, elevation_index: u8) -> Tile<TerrainId> {
        let terrain_def = self.empires.terrain(blended_tile.terrain_id);
        let elevation_graphic = &terrain_def.elevation_graphics[elevation_index as usize];
        let start_frame = *elevation_graphic.frame_id;
        let end_frame = start_frame + cmp::max(1, elevation_graphic.frame_count) as u32;
        let frames = (start_frame..end_frame).collect();

        let slp_id = if terrain_def.slp_id.is_none() {
            let alt_terrain_id = terrain_def.terrain_to_draw.unwrap();
            self.empires.terrain(alt_terrain_id).slp_id.unwrap()
        } else {
            terrain_def.slp_id.unwrap()
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
        let border = self.empires.terrain_border(border_id);
        let frame_data = &border.borders[elevation_index as usize][border_index as usize];
        let start_frame = *frame_data.frame_id;
        let end_frame = start_frame + cmp::max(1, frame_data.frame_count) as u32;
        let frames = (start_frame..end_frame).collect();

        Tile {
            id: border_id,
            slp_id: border.slp_id,
            frame_range: frames,
        }
    }
}
