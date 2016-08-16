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
use ecs::{SelectedUnitComponent, TransformComponent, UnitComponent, OnScreenComponent};
use ecs::resource::{RenderCommands, ViewProjector};
use resource::RenderCommand;
use specs::{self, Join};
use super::RenderSystem;
use types::{Color, Fixed, Vector3};
use util::unit;

pub struct UnitSelectionRenderSystem {
    empires: dat::EmpiresDbRef,
}

impl UnitSelectionRenderSystem {
    pub fn new(empires: dat::EmpiresDbRef) -> UnitSelectionRenderSystem {
        UnitSelectionRenderSystem { empires: empires }
    }
}

impl RenderSystem for UnitSelectionRenderSystem {
    fn render(&mut self, arg: specs::RunArg, lerp: Fixed) {
        fetch_components!(arg, entities, [
            components(transforms: TransformComponent),
            components(units: UnitComponent),
            components(on_screen: OnScreenComponent),
            components(selected_units: SelectedUnitComponent),
            resource(projector: ViewProjector),
            mut resource(render_commands: RenderCommands),
        ]);

        let items = (&transforms, &units, &selected_units, &on_screen);
        for (transform, unit, _selected_unit, _on_screen) in items.iter() {
            let unit_info = self.empires.unit(unit.civilization_id, unit.unit_id);
            let unit_box = unit::selection_box(unit_info, transform);
            let position = projector.project(&transform.lerped_position(lerp));

            let color = Color::rgb(255, 255, 255);
            let points: [Vector3; 4] = [unit_box.min,
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
    }
}
