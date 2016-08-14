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

use ecs::{TransformComponent, DecalComponent};
use ecs::resource::{RenderCommands, ViewProjector};
use identifier::PlayerColorId;
use resource::{RenderCommand, ShapeKey};
use specs::{self, Join};
use super::RenderSystem;
use types::Fixed;

pub struct DecalRenderSystem;

impl DecalRenderSystem {
    pub fn new() -> DecalRenderSystem {
        DecalRenderSystem
    }
}

impl RenderSystem for DecalRenderSystem {
    fn render(&mut self, arg: specs::RunArg, lerp: Fixed) {
        let (transforms, decals, projector, mut render_commands) = arg.fetch(|w| {
            (w.read::<TransformComponent>(),
             w.read::<DecalComponent>(),
             w.read_resource::<ViewProjector>(),
             w.write_resource::<RenderCommands>())
        });


        for (transform, decal) in (&transforms, &decals).iter() {
            let position = projector.project(&transform.lerped_position(lerp));
            let shape_key = ShapeKey::new(decal.drs_key,
                                          decal.slp_file_id,
                                          decal.player_color_id.into());
            render_commands.push(RenderCommand::new_shape(20,
                                                          position.y,
                                                          shape_key,
                                                          decal.frame,
                                                          position,
                                                          false,
                                                          false));
        }
    }
}
