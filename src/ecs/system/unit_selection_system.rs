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
use dat;
use ecs::{SelectedUnitComponent, TransformComponent, UnitComponent, OnScreenComponent, DecalComponent};
use ecs::resource::*;
use media::{KeyState, MouseButton};
use resource::DrsKey;
use specs::{self, Join};
use super::System;
use types::{Fixed, Vector3};
use util::unit;

pub struct UnitSelectionSystem {
    empires: dat::EmpiresDbRef,
}

impl UnitSelectionSystem {
    pub fn new(empires: dat::EmpiresDbRef) -> UnitSelectionSystem {
        UnitSelectionSystem { empires: empires }
    }
}

impl System for UnitSelectionSystem {
    fn update(&mut self, arg: specs::RunArg, _time_step: Fixed) {
        fetch_components!(arg, entities, [
            components(on_screen: OnScreenComponent),
            components(units: UnitComponent),
            mut components(decals: DecalComponent),
            mut components(selected_units: SelectedUnitComponent),
            mut components(transforms: TransformComponent),
            resource(mouse_state: MouseState),
            resource(path_finder: PathFinder),
            resource(players: Players),
            resource(view_projector: ViewProjector),
            resource(viewport: Viewport),
            resource(occupied_tiles: OccupiedTiles),
            resource(terrain: Terrain),
            mut resource(action_batcher: ActionBatcher),
        ]);

        if mouse_state.key_states.key_state(MouseButton::Left) == KeyState::TransitionUp {
            selected_units.clear();

            let mouse_ray = calculate_mouse_ray(&viewport, &mouse_state, &view_projector, &terrain);
            for (entity, _, unit, transform) in (&entities, &on_screen, &units, &transforms).iter() {
                let unit_info = self.empires.unit(unit.civilization_id, unit.unit_id);
                if unit_info.interaction_mode != dat::InteractionMode::NonInteracting {
                    let unit_box = unit::selection_box(unit_info, transform);

                    // Cast a ray from the mouse position through to the terrain and select any unit
                    // whose axis-aligned box intersects the ray.
                    if unit_box.intersects_ray(&mouse_ray.origin, &mouse_ray.direction) {
                        selected_units.insert(entity, SelectedUnitComponent);
                        break;
                    }
                }
            }
        }

        if mouse_state.key_states.key_state(MouseButton::Right) == KeyState::TransitionUp {
            let mouse_ray = calculate_mouse_ray(&viewport, &mouse_state, &view_projector, &terrain);
            let mut moving_unit = false;
            for (entity, transform, unit, _selected_unit) in (&entities,
                                                              &transforms,
                                                              &units,
                                                              &selected_units)
                .iter() {
                if unit.player_id == players.local_player().player_id {
                    let unit_info = self.empires.unit(unit.civilization_id, unit.unit_id);
                    let path = path_finder.find_path(&*terrain,
                                                     &*occupied_tiles,
                                                     transform.position(),
                                                     &mouse_ray.world_coord,
                                                     unit_info.terrain_restriction);
                    action_batcher.queue_for_entity(entity.get_id(), Action::ClearQueue);
                    action_batcher.queue_for_entity(entity.get_id(),
                                                    Action::MoveToPosition(MoveToPositionParams::new(path)));
                    moving_unit = true;
                }
            }

            if moving_unit {
                let decal = arg.create();
                transforms.insert(decal,
                                  TransformComponent::new(mouse_ray.world_coord, 0.into()));
                decals.insert(decal,
                              DecalComponent::new(0.into(), DrsKey::Interfac, 50405.into()));
            }
        }
    }
}

struct MouseRay {
    world_coord: Vector3,
    origin: Vector3,
    direction: Vector3,
}

fn calculate_mouse_ray(viewport: &Viewport,
                       mouse_state: &MouseState,
                       view_projector: &ViewProjector,
                       terrain: &Terrain)
                       -> MouseRay {
    let viewport_pos = viewport.top_left_i32();
    let mouse_pos = mouse_state.position + viewport_pos;

    // "Origin elevation" just needs to be a bit taller than the max terrain elevation
    let origin_elevation: Fixed = Fixed::from(terrain.elevation_range().1) * 2.into();
    let world_coord = view_projector.unproject(&mouse_pos, &*terrain);
    let origin = view_projector.unproject_at_elevation(&mouse_pos, origin_elevation);
    let direction = world_coord - origin;

    MouseRay {
        world_coord: world_coord,
        origin: origin,
        direction: direction,
    }
}
