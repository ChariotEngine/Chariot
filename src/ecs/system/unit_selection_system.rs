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

use action::{Action, MoveToPositionParams};
use ecs::{SelectedUnitComponent, TransformComponent, UnitComponent, VisibleUnitComponent};
use ecs::resource::*;
use super::System;

use dat;
use media::{KeyState, MouseButton};
use util::unit;

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
             mut selected_units,
             mouse_state,
             view_projector,
             viewport,
             terrain,
             mut action_batcher) = arg.fetch(|w| {
            (w.entities(),
             w.read::<VisibleUnitComponent>(),
             w.read::<UnitComponent>(),
             w.read::<TransformComponent>(),
             w.write::<SelectedUnitComponent>(),
             w.read_resource::<MouseState>(),
             w.read_resource::<ViewProjector>(),
             w.read_resource::<Viewport>(),
             w.read_resource::<Terrain>(),
             w.write_resource::<ActionBatcher>())
        });

        if mouse_state.key_states.key_state(MouseButton::Left) == KeyState::TransitionUp {
            selected_units.clear();

            let mouse_ray = calculate_mouse_ray(&viewport, &mouse_state, &view_projector, &terrain);
            for (entity, _, unit, transform) in (&entities, &visible, &units, &transforms).iter() {
                let unit_info = self.empires.unit(unit.civilization_id, unit.unit_id);
                let unit_box = unit::selection_box(unit_info, transform);

                // Cast a ray from the mouse position through to the terrain and select any unit
                // whose axis-aligned box intersects the ray.
                if unit_box.intersects_ray(&mouse_ray.origin, &mouse_ray.direction) {
                    selected_units.insert(entity, SelectedUnitComponent);
                    break;
                }
            }
        }

        if mouse_state.key_states.key_state(MouseButton::Right) == KeyState::TransitionUp {
            let mouse_ray = calculate_mouse_ray(&viewport, &mouse_state, &view_projector, &terrain);
            for (entity, _selected_unit) in (&entities, &selected_units).iter() {
                action_batcher.queue_for_entity(entity.get_id(),
                    Action::MoveToPosition(MoveToPositionParams::new(mouse_ray.world_coord)));
            }
        }
    }
}

struct MouseRay {
    world_coord: Vector3<f32>,
    origin: Vector3<f32>,
    direction: Vector3<f32>,
}

fn calculate_mouse_ray(viewport: &Viewport,
                       mouse_state: &MouseState,
                       view_projector: &ViewProjector,
                       terrain: &Terrain)
                       -> MouseRay {
    let viewport_pos: Vector2<i32> = Cast::from(*viewport.top_left());
    let mouse_pos = mouse_state.position + viewport_pos;

    // "Origin elevation" just needs to be a bit taller than the max terrain elevation
    let origin_elevation = terrain.elevation_range().1 as f32 * 2.0;
    let world_coord = view_projector.unproject(&mouse_pos, &*terrain);
    let origin = view_projector.unproject_at_elevation(&mouse_pos, origin_elevation);
    let direction = world_coord - origin;

    MouseRay {
        world_coord: world_coord,
        origin: origin,
        direction: direction,
    }
}
