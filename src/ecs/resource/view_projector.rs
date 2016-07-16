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

use nalgebra::{Vector2, Vector3};

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
                     ((world_coord.y - world_coord.x - world_coord.z) *
                         self.tile_half_height) as i32)
    }

    /// Unprojects screen coordinates back into world coordinates
    /// Can't determine the z-coordinate; that will always come back out as zero.
    pub fn unproject(&self, screen_coord: &Vector2<i32>) -> Vector3<f32> {
        let world_y = (screen_coord.x as f32 / self.tile_half_width) / 2. +
            (screen_coord.y as f32 / self.tile_half_height) / 2.;
        Vector3::new(screen_coord.x as f32 / self.tile_half_width - world_y,
                     world_y,
                     0.)
    }
}

#[cfg(test)]
mod tests {
    use super::ViewProjector;
    use nalgebra::{Vector2, Vector3};

    #[test]
    fn test_project_z() {
        let projector = ViewProjector::new(32, 16);
        let world_coord = Vector3::new(10f32, 15f32, 3f32);

        let screen_coord = projector.project(&world_coord);
        assert_eq!(Vector2::new(800i32, 32i32), screen_coord);
    }

    #[test]
    fn test_project_unproject() {
        let projector = ViewProjector::new(32, 16);
        let world_coord = Vector3::new(10f32, 15f32, 0f32);

        let screen_coord = projector.project(&world_coord);
        assert_eq!(Vector2::new(800i32, 80i32), screen_coord);

        let full_cycle = projector.unproject(&screen_coord);
        assert_eq!(world_coord, full_cycle);
    }
}
