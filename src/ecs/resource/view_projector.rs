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

use ecs::resource::Viewport;
use ecs::resource::Terrain;

use types::Rect;

use nalgebra::{Cast, Vector2, Vector3};

/// Resource for converting world coordinates to/from screen coordinates
pub struct ViewProjector {
    tile_half_width: f32,
    tile_half_height: f32,
}

impl ViewProjector {
    pub fn new(tile_half_width: i32, tile_half_height: i32) -> ViewProjector {
        ViewProjector {
            tile_half_width: tile_half_width as f32,
            tile_half_height: tile_half_height as f32,
        }
    }

    /// Projects world coordinates into screen coordinates
    pub fn project(&self, world_coord: &Vector3<f32>) -> Vector2<i32> {
        Vector2::new(((world_coord.y + world_coord.x) * self.tile_half_width) as i32,
                     ((world_coord.y - world_coord.x - world_coord.z) * self.tile_half_height) as i32)
    }

    /// Unprojects screen coordinates back into world coordinates
    pub fn unproject(&self, screen_coord: &Vector2<i32>, terrain: &Terrain) -> Vector3<f32> {
        let (min_elevation, max_elevation) = terrain.elevation_range();
        let (map_width, map_height) = (terrain.width() as f32, terrain.height() as f32);

        // We can't calculate the elevation from the screen coordinates since that information
        // is lost when the world coordinates are projected. However, we can iterate over every
        // possible terrain elevation at the given screen coordinate and determine the correct
        // tile by cross-comparing with the actual terrain.
        let mut selected: Option<Vector3<f32>> = None;
        for elevation in min_elevation..(max_elevation + 1) {
            let world_coord = self.unproject_at_elevation(screen_coord, elevation as f32);
            if selected.is_none() ||
               (world_coord.x >= 0.0 && world_coord.y >= 0.0 && world_coord.x < map_width &&
                world_coord.y < map_height && world_coord.z >= selected.unwrap().z) {
                if selected.is_none() || terrain.tile_at(world_coord).elevation as i32 == elevation {
                    selected = Some(world_coord)
                }
            }
        }
        selected.unwrap()
    }

    pub fn unproject_at_elevation(&self, screen_coord: &Vector2<i32>, elevation: f32) -> Vector3<f32> {
        let (tile_width, tile_height) = (self.tile_half_width as f32 * 2.0,
                                         self.tile_half_height as f32 * 2.0);
        let world_x = (screen_coord.x as f32 / tile_width) - (screen_coord.y as f32 / tile_height) -
                      (elevation / 2.0);
        let world_y = (screen_coord.x as f32 / tile_width) + (screen_coord.y as f32 / tile_height) +
                      (elevation / 2.0);
        Vector3::new(world_x, world_y, elevation)
    }

    /// Returns an approximate rectangle of visible world coords
    pub fn calculate_visible_world_coords(&self, viewport: &Viewport, terrain: &Terrain) -> Rect {
        use std::cmp::{max, min};

        let round =
            |v: Vector3<f32>| Vector3::new(v.x.round() as i32, v.y.round() as i32, v.z.round() as i32);

        let vtl: Vector2<i32> = Cast::from(*viewport.top_left());
        let vsize: Vector2<i32> = Cast::from(viewport.size);

        // top left, top right, bottom left, and bottom right; excuse the short names
        let tl = round(self.unproject(&vtl, terrain));
        let tr = round(self.unproject(&(vtl + Vector2::new(vsize.x, 0)), terrain));
        let bl = round(self.unproject(&(vtl + Vector2::new(0, vsize.y)), terrain));
        let br = round(self.unproject(&(vtl + vsize), terrain));

        let mut area = Rect::new();
        area.x = min(tl.x, min(tr.x, min(bl.x, br.x)));
        area.y = min(tl.y, min(tr.y, min(bl.y, br.y)));
        area.w = max(tl.x, max(tr.x, max(bl.x, br.x))) - area.x;
        area.h = max(tl.y, max(tr.y, max(bl.y, br.y))) - area.y;
        area
    }
}

#[cfg(test)]
mod tests {
    use super::ViewProjector;
    use ecs::resource::{Terrain, Tile};
    use dat::{EmpiresDb, EmpiresDbRef};
    use nalgebra::{Vector2, Vector3};

    #[test]
    fn test_project_z() {
        let projector = ViewProjector::new(32, 16);
        let world_coord = Vector3::new(10f32, 15f32, 3f32);

        let screen_coord = projector.project(&world_coord);
        assert_eq!(Vector2::new(800i32, 32i32), screen_coord);
    }

    fn round_trip_coord(projector: &ViewProjector, terrain: &Terrain, coord: Vector2<f32>) -> Vector3<i32> {
        let tile = terrain.tile_at(Vector3::new(coord.x, coord.y, 0.0));
        let world_coord = Vector3::new(coord.x, coord.y, tile.elevation as f32);
        let projected = projector.project(&world_coord);
        let unprojected = projector.unproject(&projected, &terrain);
        Vector3::new(unprojected.x as i32,
                     unprojected.y as i32,
                     unprojected.z as i32)
    }

    fn test_terrain() -> Terrain {
        let (width, height) = (3, 3);
        let tiles = vec![
            Tile::new(0.into(), 0),
            Tile::new(0.into(), 0),
            Tile::new(0.into(), 0),

            Tile::new(0.into(), 0),
            Tile::new(0.into(), 1),
            Tile::new(0.into(), 0),

            Tile::new(0.into(), 1),
            Tile::new(0.into(), 2),
            Tile::new(0.into(), 1),
        ];
        let empires = EmpiresDbRef::new(EmpiresDb::new());
        Terrain::new(width, height, tiles, empires)
    }

    #[test]
    fn test_project_unproject() {
        let terrain = test_terrain();
        let projector = ViewProjector::new(32, 16);
        let world_coord = Vector3::new(10f32, 15f32, 0f32);

        assert_eq!(Vector3::new(0, 0, 0),
                   round_trip_coord(&projector, &terrain, Vector2::new(0.5, 0.5)));
        // The elevation of the middle tile causes that tile to be selected instead of (1, 0)
        assert_eq!(Vector3::new(1, 1, 1),
                   round_trip_coord(&projector, &terrain, Vector2::new(1.5, 0.5)));
        assert_eq!(Vector3::new(2, 0, 0),
                   round_trip_coord(&projector, &terrain, Vector2::new(2.5, 0.5)));

        assert_eq!(Vector3::new(0, 2, 1),
                   round_trip_coord(&projector, &terrain, Vector2::new(0.5, 1.5)));
        assert_eq!(Vector3::new(1, 2, 2),
                   round_trip_coord(&projector, &terrain, Vector2::new(1.5, 1.5)));
        assert_eq!(Vector3::new(1, 2, 2),
                   round_trip_coord(&projector, &terrain, Vector2::new(2.5, 1.5)));

        assert_eq!(Vector3::new(0, 2, 1),
                   round_trip_coord(&projector, &terrain, Vector2::new(0.5, 2.5)));
        assert_eq!(Vector3::new(1, 2, 2),
                   round_trip_coord(&projector, &terrain, Vector2::new(1.5, 2.5)));
        assert_eq!(Vector3::new(2, 2, 1),
                   round_trip_coord(&projector, &terrain, Vector2::new(2.5, 2.5)));
    }
}
