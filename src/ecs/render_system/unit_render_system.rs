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

use ecs::{TransformComponent, UnitComponent, VisibleUnitComponent};
use ecs::resource::ViewProjector;

use dat;
use media::MediaRef;
use resource::{DrsKey, ShapeKey, ShapeManagerRef};

use nalgebra::Vector2;
use specs::{self, Join};

struct RenderOp {
    layer: u8,
    position: Vector2<i32>,
    shape_key: ShapeKey,
    frame: u32,
}

pub struct UnitRenderSystem {
    media: MediaRef,
    shape_manager: ShapeManagerRef,
    empires: dat::EmpiresDbRef,
    render_ops: Vec<RenderOp>,
}

impl UnitRenderSystem {
    pub fn new(media: MediaRef,
               shape_manager: ShapeManagerRef,
               empires: dat::EmpiresDbRef)
               -> UnitRenderSystem {
        UnitRenderSystem {
            media: media,
            shape_manager: shape_manager,
            empires: empires,
            render_ops: Vec::new(),
        }
    }

    pub fn render(&mut self, world: &mut specs::World, lerp: f32) {
        let (transforms, units, visible_units, projector) = (world.read::<TransformComponent>(),
                                                             world.read::<UnitComponent>(),
                                                             world.read::<VisibleUnitComponent>(),
                                                             world.read_resource::<ViewProjector>());
        let mut media = self.media.borrow_mut();
        let renderer = media.renderer();

        for (transform, unit, _visible_units) in (&transforms, &units, &visible_units).iter() {
            if let Some(graphic_id) = unit.graphic_id {
                let graphic = self.empires.graphic(graphic_id);
                if let Some(slp_id) = graphic.slp_id {
                    self.render_ops.push(RenderOp {
                        layer: graphic.layer,
                        position: projector.project(&transform.lerped_position(lerp)),
                        shape_key: ShapeKey::new(DrsKey::Graphics, slp_id, unit.player_id.into()),
                        frame: unit.frame,
                    });
                }
            }
        }

        self.render_ops.sort_by(|left, right| {
            use std::cmp::Ordering::*;
            match left.layer.cmp(&right.layer) {
                Equal => left.position.y.cmp(&right.position.y),
                v @ _ => v,
            }
        });

        for render_op in &self.render_ops {
            // TODO: Render the unit's rotation as well
            self.shape_manager
                .borrow_mut()
                .get(&render_op.shape_key, renderer)
                .expect("failed to get shape for unit rendering")
                .render_frame(renderer, render_op.frame as usize, &render_op.position);
        }
        self.render_ops.clear();
    }
}
