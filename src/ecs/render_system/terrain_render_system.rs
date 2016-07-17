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

use ecs::resource::{Terrain, ViewProjector, Viewport};
use ecs::resource::terrain::{BlendInfo, BorderMatch, ElevationGraphic, ElevationMatch};

use dat;
use media::MediaRef;
use resource::{DrsKey, ShapeKey, ShapeManagerRef};
use identifier::{PlayerColorId, SlpFileId, TerrainBorderId, TerrainId};
use types::Rect;

use nalgebra::{Vector2, Vector3};
use specs;

use std::collections::HashMap;
use std::cmp;

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
    media: MediaRef,
    shape_manager: ShapeManagerRef,
    empires: dat::EmpiresDbRef,
    tiles: HashMap<TileKey<TerrainId>, Tile<TerrainId>>,
    borders: HashMap<TileKey<TerrainBorderId>, Tile<TerrainBorderId>>,
}

impl TerrainRenderSystem {
    pub fn new(media: MediaRef,
               shape_manager: ShapeManagerRef,
               empires: dat::EmpiresDbRef)
               -> TerrainRenderSystem {
        TerrainRenderSystem {
            media: media,
            shape_manager: shape_manager,
            empires: empires,
            tiles: HashMap::new(),
            borders: HashMap::new(),
        }
    }

    pub fn render(&mut self, world: &mut specs::World) {
        let (terrain, projector, viewport) = (world.read_resource::<Terrain>(),
                                              world.read_resource::<ViewProjector>(),
                                              world.read_resource::<Viewport>());

        let area = projector.calculate_visible_world_coords(&viewport);

        let tile_width = (self.terrain_block().tile_half_width * 2) as i32;
        let tile_height = (self.terrain_block().tile_half_height * 2) as i32;

        let mut bounds = Rect::new();
        bounds.x = viewport.top_left().x as i32 - tile_width;
        bounds.y = viewport.top_left().y as i32 - tile_height;
        bounds.w = bounds.x + viewport.size.x as i32 + 2 * tile_width;
        bounds.h = bounds.y + viewport.size.y as i32 + 2 * tile_height;

        for row in area.y..(area.y + area.h) {
            for col in (area.x..(area.x + area.w)).rev() {
                if row >= 0 && row < terrain.width() && col >= 0 && col < terrain.height() {
                    let pos = projector.project(&Vector3::new(col as f32, row as f32, 0f32));
                    if pos.x > bounds.x && pos.y > bounds.y && pos.x < bounds.w &&
                       pos.y < bounds.h {
                        self.blend_and_render_tile(row, col, &terrain);
                    }
                }
            }
        }
    }

    fn blend_and_render_tile(&mut self, row: i32, col: i32, terrain: &Terrain) {
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
            self.render_tile(DrsKey::Terrain, &tile, render_offset_y, row, col);
        }

        if blended_tile.border_id.is_some() {
            if let Some(border_match) = BorderMatch::find_match(blended_tile.border_style,
                                                                blended_tile.border_matrix) {
                self.render_borders(blended_tile.border_id.unwrap(),
                                    &border_match.border_indices,
                                    elevation_graphic.index,
                                    render_offset_y,
                                    row,
                                    col)
            }
        }
    }

    fn render_tile<T>(&self,
                      drs_key: DrsKey,
                      tile: &Tile<T>,
                      render_offset_y: f32,
                      row: i32,
                      col: i32) {
        let (x, y) = self.project_row_col(row, col, render_offset_y);
        let frame_num = ((row + 1) * (col - row)) as usize % tile.frame_range.len();

        let mut media = self.media.borrow_mut();
        let renderer = media.renderer();
        self.shape_manager
            .borrow_mut()
            .get(&ShapeKey::new(drs_key, tile.slp_id, PlayerColorId(0)),
                 renderer)
            .expect("failed to get shape for terrain rendering")
            .render_frame(renderer,
                          tile.frame_range[frame_num] as usize,
                          &Vector2::new(x, y));
    }

    fn render_borders(&mut self,
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
            self.render_tile(DrsKey::Border, border, render_offset_y, row, col);
        }
    }

    fn project_row_col(&self, row: i32, col: i32, render_offset_y: f32) -> (i32, i32) {
        let tile_half_width = self.terrain_block().tile_half_width as i32;
        let tile_half_height = self.terrain_block().tile_half_height as i32;
        let render_offset_y = (render_offset_y * tile_half_height as f32) as i32;
        ((row + col) * tile_half_width,
         (row - col) * tile_half_height - tile_half_height - render_offset_y)
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
        let terrain_def = &self.terrain_block().terrains[&blended_tile.terrain_id];
        let elevation_graphic = &terrain_def.elevation_graphics[elevation_index as usize];
        let start_frame = elevation_graphic.frame_id.as_usize() as u32;
        let end_frame = start_frame + cmp::max(1, elevation_graphic.frame_count) as u32;
        let frames = (start_frame..end_frame).collect();

        let slp_id = if terrain_def.slp_id.as_isize() == -1 {
            let alt_terrain_id = &terrain_def.terrain_to_draw;
            let alt_terrain_def = &self.terrain_block().terrains[alt_terrain_id];
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
        let border = &self.terrain_block().terrain_borders[border_id.as_usize()];
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

    #[inline]
    fn terrain_block<'a>(&'a self) -> &'a dat::TerrainBlock {
        &self.empires.terrain_block
    }
}
