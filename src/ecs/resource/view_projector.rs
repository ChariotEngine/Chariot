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

use ecs::resource::Terrain;
use ecs::resource::Viewport;
use nalgebra::Vector2;
use types::{Fixed, Rect, Vector3};

/// Resource for converting world coordinates to/from screen coordinates
pub struct ViewProjector {
    tile_half_width: Fixed,
    tile_half_height: Fixed,
}

impl ViewProjector {
    pub fn new(tile_half_width: i32, tile_half_height: i32) -> ViewProjector {
        ViewProjector {
            tile_half_width: tile_half_width.into(),
            tile_half_height: tile_half_height.into(),
        }
    }

    /// Projects world coordinates into screen coordinates
    pub fn project(&self, world_coord: &Vector3) -> Vector2<i32> {
        Vector2::new(((world_coord.y + world_coord.x) * self.tile_half_width).into(),
                     ((world_coord.y - world_coord.x - world_coord.z) * self.tile_half_height).into())
    }

    /// Unprojects screen coordinates back into world coordinates
    pub fn unproject(&self, screen_coord: &Vector2<i32>, terrain: &Terrain) -> Vector3 {
        let (min_elevation, max_elevation) = terrain.elevation_range();
        let (map_width, map_height) = (Fixed::from(terrain.width()), Fixed::from(terrain.height()));

        // We can't calculate the elevation from the screen coordinates since that information
        // is lost when the world coordinates are projected. However, we can iterate over every
        // possible terrain elevation at the given screen coordinate and determine the correct
        // tile by cross-comparing with the actual terrain.
        let mut selected: Option<Vector3> = None;
        for elevation in min_elevation..(max_elevation + 1) {
            let world_coord = self.unproject_at_elevation(screen_coord, elevation.into());
            if selected.is_none() ||
               (world_coord.x >= 0.into() && world_coord.y >= 0.into() && world_coord.x < map_width &&
                world_coord.y < map_height && world_coord.z >= selected.unwrap().z) {
                if selected.is_none() || terrain.tile_at(world_coord).elevation as i32 == elevation {
                    selected = Some(world_coord)
                }
            }
        }
        selected.unwrap()
    }

    pub fn unproject_at_elevation(&self, screen_coord: &Vector2<i32>, elevation: Fixed) -> Vector3 {
        let (tile_width, tile_height) = (self.tile_half_width * 2.into(), self.tile_half_height * 2.into());
        let (sx, sy) = (Fixed::from(screen_coord.x), Fixed::from(screen_coord.y));
        let half_elevation = elevation / 2.into();
        let world_x = (sx / tile_width) - (sy / tile_height) - half_elevation;
        let world_y = (sx / tile_width) + (sy / tile_height) + half_elevation;
        Vector3::new(world_x, world_y, elevation)
    }

    /// Returns an approximate rectangle of visible world coords
    pub fn calculate_visible_world_coords(&self, viewport: &Viewport, terrain: &Terrain) -> Rect {
        use std::cmp::{max, min};

        let round = |v: Vector3| Vector3::new(v.x.round(), v.y.round(), v.z.round());

        let vtl = viewport.top_left_i32();
        let vsize = viewport.size;

        // top left, top right, bottom left, and bottom right; excuse the short names
        let tl = round(self.unproject(&vtl, terrain));
        let tr = round(self.unproject(&(vtl + Vector2::new(vsize.x, 0)), terrain));
        let bl = round(self.unproject(&(vtl + Vector2::new(0, vsize.y)), terrain));
        let br = round(self.unproject(&(vtl + vsize), terrain));

        let mut area = Rect::new();
        area.x = min::<i32>(tl.x.into(), min(tr.x.into(), min(bl.x.into(), br.x.into())));
        area.y = min::<i32>(tl.y.into(), min(tr.y.into(), min(bl.y.into(), br.y.into())));
        area.w = max::<i32>(tl.x.into(), max(tr.x.into(), max(bl.x.into(), br.x.into()))) - area.x;
        area.h = max::<i32>(tl.y.into(), max(tr.y.into(), max(bl.y.into(), br.y.into()))) - area.y;
        area
    }
}

#[cfg(test)]
mod tests {
    use dat::{EmpiresDb, EmpiresDbRef};
    use ecs::resource::{Terrain, Tile};
    use nalgebra::Vector2;
    use super::ViewProjector;
    use types::Vector3;

    #[test]
    fn test_project_z() {
        let projector = ViewProjector::new(32, 16);
        let world_coord = Vector3::new(10.into(), 15.into(), 3.into());

        let screen_coord = projector.project(&world_coord);
        assert_eq!(Vector2::new(800i32, 32i32), screen_coord);
    }

    fn round_trip_coord(projector: &ViewProjector, terrain: &Terrain, coord: Vector2<f32>) -> Vector3 {
        let tile = terrain.tile_at(Vector3::new(coord.x.into(), coord.y.into(), 0.into()));
        let world_coord = Vector3::new(coord.x.into(), coord.y.into(), tile.elevation.into());
        let projected = projector.project(&world_coord);
        projector.unproject(&projected, &terrain)
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
        let v3 = |x: f32, y: f32, z: f32| Vector3::new(x.into(), y.into(), z.into());
        let v2 = |x: f32, y: f32| Vector2::new(x, y);

        let terrain = test_terrain();
        let projector = ViewProjector::new(32, 16);

        assert_eq!(v3(0.5, 0.5, 0.0),
                   round_trip_coord(&projector, &terrain, v2(0.5, 0.5)));
        // The elevation of the middle tile causes that tile to be selected instead of (1, 0)
        assert_eq!(v3(1.0, 1.0, 1.0),
                   round_trip_coord(&projector, &terrain, v2(1.5, 0.5)));
        assert_eq!(v3(2.5, 0.5, 0.0),
                   round_trip_coord(&projector, &terrain, v2(2.5, 0.5)));

        assert_eq!(v3(0.0, 2.0, 1.0),
                   round_trip_coord(&projector, &terrain, v2(0.5, 1.5)));
        assert_eq!(v3(1.0, 2.0, 2.0),
                   round_trip_coord(&projector, &terrain, v2(1.5, 1.5)));
        assert_eq!(v3(1.5, 2.5, 2.0),
                   round_trip_coord(&projector, &terrain, v2(2.5, 1.5)));

        assert_eq!(v3(0.5, 2.5, 1.0),
                   round_trip_coord(&projector, &terrain, v2(0.5, 2.5)));
        assert_eq!(v3(1.5, 2.5, 2.0),
                   round_trip_coord(&projector, &terrain, v2(1.5, 2.5)));
        assert_eq!(v3(2.5, 2.5, 1.0),
                   round_trip_coord(&projector, &terrain, v2(2.5, 2.5)));
    }
}
