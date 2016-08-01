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

use super::System;
use ecs::{TransformComponent, UnitComponent, VisibleUnitComponent};
use ecs::resource::{MouseState, RenderCommands, Terrain, ViewProjector, Viewport};

use dat;
use media::{KeyState, MouseButton};
use types::{AABox, Color, Rect};
use resource::RenderCommand;

use nalgebra::{Cast, Vector2, Vector3};
use specs::{self, Join};

pub struct UnitSelectionSystem {
    empires: dat::EmpiresDbRef,
}

impl UnitSelectionSystem {
    pub fn new(empires: dat::EmpiresDbRef) -> UnitSelectionSystem {
        UnitSelectionSystem { empires: empires }
    }
}

impl System for UnitSelectionSystem {
    fn update(&mut self, arg: specs::RunArg, _time_step: f32) {
        let (entities,
             visible,
             units,
             transforms,
             mouse_state,
             view_projector,
             viewport,
             terrain,
             mut render_commands) = arg.fetch(|w| {
            (w.entities(),
             w.read::<VisibleUnitComponent>(),
             w.read::<UnitComponent>(),
             w.read::<TransformComponent>(),
             w.read_resource::<MouseState>(),
             w.read_resource::<ViewProjector>(),
             w.read_resource::<Viewport>(),
             w.read_resource::<Terrain>(),
             w.write_resource::<RenderCommands>())
        });

        for (entity, _, unit, transform) in (&entities, &visible, &units, &transforms).iter() {
            let unit_info = self.empires.unit(unit.civilization_id, unit.unit_id);
            let unit_box = unit_box(unit_info, transform);
            render_aabox(&mut *render_commands, &*view_projector, &unit_box);
        }

        if mouse_state.key_states.key_state(MouseButton::Left) == KeyState::TransitionUp {
            let viewport_pos: Vector2<i32> = Cast::from(*viewport.top_left());
            let mouse_pos = mouse_state.position + viewport_pos;
            let world_coord = view_projector.unproject(&mouse_pos, &*terrain);

            for (entity, _, unit, transform) in (&entities, &visible, &units, &transforms).iter() {
                let unit_info = self.empires.unit(unit.civilization_id, unit.unit_id);
                let unit_box = unit_box(unit_info, transform);
                if unit_box.contains(&world_coord) {
                    println!("Selecting unit with ID: {}", entity.get_id());
                }
            }
        }
    }
}

fn render_aabox(render_commands: &mut RenderCommands, view_projector: &ViewProjector, aabox: &AABox) {
    let red = Color::rgb(255, 0, 0);
    let draw_line = &mut |a, b| {
        render_commands.push(RenderCommand::new_debug_line(100000,
                                                           0,
                                                           red,
                                                           view_projector.project(&a),
                                                           view_projector.project(&b)));
    };
    let (min, max) = (aabox.min, aabox.max);

    let layer0: [Vector3<f32>; 4] = [min,
                                     Vector3::new(max.x, min.y, min.z),
                                     Vector3::new(max.x, max.y, min.z),
                                     Vector3::new(min.x, max.y, min.z)];
    let layer1: [Vector3<f32>; 4] = [Vector3::new(min.x, min.y, max.z),
                                     Vector3::new(max.x, min.y, max.z),
                                     max,
                                     Vector3::new(min.x, max.y, max.z)];

    for i in 0..4 {
        draw_line(layer0[i], layer1[i]);
        draw_line(layer0[i], layer0[(i + 1) % 4]);
        draw_line(layer1[i], layer1[(i + 1) % 4]);
    }
}

fn unit_box(unit_info: &dat::Unit, transform: &TransformComponent) -> AABox {
    let position = transform.position();
    AABox::new(Vector3::new(position.x - unit_info.selection_shape_size_x,
                            position.y - unit_info.selection_shape_size_y,
                            position.z + unit_info.selection_shape_size_z),
               Vector3::new(position.x + unit_info.selection_shape_size_x,
                            position.y + unit_info.selection_shape_size_y,
                            position.z))
}
