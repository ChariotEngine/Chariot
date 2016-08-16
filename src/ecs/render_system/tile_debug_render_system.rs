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

use ecs::resource::{KeyboardKeyStates, MouseState, RenderCommands, Terrain, ViewProjector, Viewport};
use media::{Key, KeyState};
use resource::{DrsKey, RenderCommand, ShapeKey};
use specs;
use super::RenderSystem;
use types::Fixed;

/// Used for debugging tile positions and tile picking
pub struct TileDebugRenderSystem;

impl TileDebugRenderSystem {
    pub fn new() -> TileDebugRenderSystem {
        TileDebugRenderSystem
    }
}

impl RenderSystem for TileDebugRenderSystem {
    fn render(&mut self, arg: specs::RunArg, _lerp: Fixed) {
        fetch_components!(arg, entities, [
            resource(mouse_state: MouseState),
            resource(view_projector: ViewProjector),
            resource(viewport: Viewport),
            resource(keyboard_key_states: KeyboardKeyStates),
            mut resource(terrain: Terrain),
            mut resource(render_commands: RenderCommands),
        ]);

        let viewport_top_left = viewport.top_left_i32();
        let tile_pos = view_projector.unproject(&(mouse_state.position + viewport_top_left), &*terrain);

        if keyboard_key_states.key_state(Key::Space) == KeyState::TransitionUp {
            let row: i32 = tile_pos.y.round().into();
            let col: i32 = tile_pos.x.round().into();
            let actual_tile = *terrain.tile_at(tile_pos);
            let blend_info = *terrain.blend_at(row, col);
            println!("\nTile under cursor ({}, {}):\n{:?}\n{:#?}\n",
                     row,
                     col,
                     actual_tile,
                     blend_info);
        }

        // Draw a cactus at the tile's position
        let debug_pos = view_projector.project(&tile_pos);
        let shape_key = ShapeKey::new(DrsKey::Graphics, 275.into(), 0.into());
        render_commands.push(RenderCommand::new_shape(1000, 0, shape_key, 0, debug_pos, false, false));
    }
}
