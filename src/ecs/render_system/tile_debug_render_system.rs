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

use ecs::resource::{MouseState, Terrain, ViewProjector, Viewport};

use media::MediaRef;
use resource::{DrsKey, ShapeKey, ShapeManagerRef};
use identifier::{PlayerColorId, SlpFileId};

use nalgebra::{Cast, Vector2};
use specs;

/// Used for debugging tile positions and tile picking
pub struct TileDebugRenderSystem {
    media: MediaRef,
    shape_manager: ShapeManagerRef,
}

impl TileDebugRenderSystem {
    pub fn new(media: MediaRef, shape_manager: ShapeManagerRef) -> TileDebugRenderSystem {
        TileDebugRenderSystem {
            media: media,
            shape_manager: shape_manager,
        }
    }

    pub fn render(&self, world: &mut specs::World) {
        let (mouse_state, view_projector, viewport, terrain) =
            (world.read_resource::<MouseState>(),
             world.read_resource::<ViewProjector>(),
             world.read_resource::<Viewport>(),
             world.read_resource::<Terrain>());

        let viewport_top_left: Vector2<i32> = Cast::from(*viewport.top_left());
        let tile_pos = view_projector.unproject(&(mouse_state.position + viewport_top_left));

        // Uncomment to see elevation at mouse cursor:
        // let actual_tile = terrain.tile_at(tile_pos);
        // println!("tile elevation: {}", actual_tile.elevation);

        let debug_pos = view_projector.project(&tile_pos);

        // TODO: Render tile position/elevation onto the screen (when font rendering is available)

        // Draw a cactus at the tile's position
        let mut media = self.media.borrow_mut();
        let renderer = media.renderer();
        let shape_key = ShapeKey::new(DrsKey::Graphics, SlpFileId(275), PlayerColorId(0));
        self.shape_manager
            .borrow_mut()
            .get(&shape_key, renderer)
            .expect("failed to get debug shape")
            .render_frame(renderer, 0, &debug_pos);
    }
}
