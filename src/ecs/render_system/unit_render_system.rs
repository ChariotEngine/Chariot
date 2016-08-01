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

use super::RenderSystem;
use ecs::{SelectedUnitComponent, TransformComponent, UnitComponent, VisibleUnitComponent};
use ecs::resource::{RenderCommands, ViewProjector};
use util::unit;

use dat;
use resource::{DrsKey, RenderCommand, ShapeKey};
use types::Color;

use nalgebra::Vector3;
use specs::{self, Join};

pub struct UnitRenderSystem {
    empires: dat::EmpiresDbRef,
}

impl UnitRenderSystem {
    pub fn new(empires: dat::EmpiresDbRef) -> UnitRenderSystem {
        UnitRenderSystem { empires: empires }
    }
}

impl RenderSystem for UnitRenderSystem {
    fn render(&mut self, arg: specs::RunArg, lerp: f32) {
        let (transforms, units, visible_units, selected_units, projector, mut render_commands) =
            arg.fetch(|w| {
                (w.read::<TransformComponent>(),
                 w.read::<UnitComponent>(),
                 w.read::<VisibleUnitComponent>(),
                 w.read::<SelectedUnitComponent>(),
                 w.read_resource::<ViewProjector>(),
                 w.write_resource::<RenderCommands>())
            });

        for (transform, unit, _selected_unit) in (&transforms, &units, &selected_units).iter() {
            let unit_info = self.empires.unit(unit.civilization_id, unit.unit_id);
            let unit_box = unit::selection_box(unit_info, transform);
            let position = projector.project(&transform.lerped_position(lerp));

            let color = Color::rgb(255, 255, 255);
            let points: [Vector3<f32>; 4] = [unit_box.min,
                                             Vector3::new(unit_box.max.x, unit_box.min.y, unit_box.min.z),
                                             Vector3::new(unit_box.max.x, unit_box.max.y, unit_box.min.z),
                                             Vector3::new(unit_box.min.x, unit_box.max.y, unit_box.min.z)];
            for i in 0..4 {
                render_commands.push(RenderCommand::new_line(1,
                                                             position.y,
                                                             color,
                                                             projector.project(&points[i]),
                                                             projector.project(&points[(i + 1) % 4])));
            }
        }

        for (transform, unit, _visible_units) in (&transforms, &units, &visible_units).iter() {
            if let Some(graphic_id) = unit.graphic_id {
                let graphic = self.empires.graphic(graphic_id);
                let position = projector.project(&transform.lerped_position(lerp));
                if let Some(slp_id) = graphic.slp_id {
                    let shape_key = ShapeKey::new(DrsKey::Graphics, slp_id, unit.player_id.into());
                    render_commands.push(RenderCommand::new_shape(graphic.layer as u16,
                                                                  position.y,
                                                                  shape_key,
                                                                  unit.frame,
                                                                  position,
                                                                  unit.flip_horizontal,
                                                                  unit.flip_vertical));
                }
            }
        }
    }
}
