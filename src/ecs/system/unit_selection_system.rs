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
use ecs::resource::{MouseState, ViewProjector, Viewport};

use dat;
use media::{KeyState, MouseButton};
use types::Rect;

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
        let (entities, visible, units, transforms, mouse_state, view_projector, viewport) = arg.fetch(|w| {
                (w.entities(),
                 w.read::<VisibleUnitComponent>(),
                 w.read::<UnitComponent>(),
                 w.read::<TransformComponent>(),
                 w.read_resource::<MouseState>(),
                 w.read_resource::<ViewProjector>(),
                 w.read_resource::<Viewport>())
            });

        if mouse_state.key_states.key_state(MouseButton::Left) == KeyState::TransitionUp {
            let viewport_pos: Vector2<i32> = Cast::from(*viewport.top_left());
            let mouse_pos = mouse_state.position + viewport_pos;
            for (entity, _, unit, transform) in (&entities, &visible, &units, &transforms).iter() {
                let unit_info = self.empires.unit(unit.civilization_id, unit.unit_id);
                let unit_rect = unit_rect(unit_info, transform, &*view_projector);
                if unit_rect.x <= mouse_pos.x && unit_rect.w >= mouse_pos.x && unit_rect.y <= mouse_pos.y &&
                   unit_rect.h >= mouse_pos.y {
                    println!("Selecting unit with ID: {}", entity.get_id());
                }
            }
        }
    }
}

fn unit_rect(unit_info: &dat::Unit, transform: &TransformComponent, view_projector: &ViewProjector) -> Rect {
    let shape = Vector3::new(unit_info.selection_shape_size_x,
                             unit_info.selection_shape_size_y,
                             unit_info.selection_shape_size_z);
    let world_top_left = *transform.position() - shape;
    let world_bottom_right = *transform.position() + shape;

    let screen_top_left = view_projector.project(&world_top_left);
    let screen_bottom_right = view_projector.project(&world_bottom_right);

    Rect::of(screen_top_left.x,
             screen_bottom_right.y,
             screen_bottom_right.x,
             screen_top_left.y)
}
