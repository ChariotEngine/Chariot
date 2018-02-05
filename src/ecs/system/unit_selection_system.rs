// Chariot: An open source reimplementation of Age of Empires (1997)
// Copyright (c) 2016 Kevin Fuller
// Copyright (c) 2018 Taryn Hill
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

//! This system is responsible for unit selection and queuing up a MoveToPosition action.

use std::collections::HashSet;
use action::{Action, MoveToPositionParams};
use dat;
use ecs::{DecalComponent, OnScreenComponent, SelectedUnitComponent, TransformComponent, UnitComponent};

use ecs::resource::{
    MouseState,
    KeyboardKeyStates,
    PathFinder,
    Players,
    ViewProjector,
    Viewport,
    OccupiedTiles,
    Terrain,
    ActionBatcher,
};

use media::{KeyState, MouseButton, Key};
use resource::DrsKey;
use specs::{self, Join};
use super::System;
use types::{Fixed, Vector3};
use util::unit as unit_util;

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
            components(on_screen_comp: OnScreenComponent),
            components(units_comp: UnitComponent),
        mut components(decals_comp: DecalComponent),
        mut components(selected_units_comp: SelectedUnitComponent),
        mut components(transforms_comp: TransformComponent),

            resource(keyboard_state_rc: KeyboardKeyStates),
            resource(mouse_state_rc: MouseState),
            resource(path_finder_rc: PathFinder),
            resource(players_rc: Players),
            resource(view_projector_rc: ViewProjector),
            resource(viewport_rc: Viewport),
            resource(occupied_tiles_rc: OccupiedTiles),
            resource(terrain_rc: Terrain),
        mut resource(action_batcher_rc: ActionBatcher),
        ]);

        let mouse_ray = calculate_mouse_ray(&viewport_rc, &mouse_state_rc, &view_projector_rc, &terrain_rc);

        // TEMP
        let mut should_render_attack_cursor = false;

        'outer: for (entity, transform_comp, unit_comp) in (&entities, &transforms_comp, &units_comp).iter() {
            if unit_comp.player_id == players_rc.local_player().player_id {
                continue;
            }

            let unit_info_hovered = self.empires.unit(unit_comp.civilization_id, unit_comp.unit_id);
            if unit_info_hovered.interaction_mode == dat::InteractionMode::NonInteracting {
                continue;
            }

            let selection_box = unit_util::selection_box(unit_info_hovered, transform_comp);
            if !selection_box.intersects_ray(&mouse_ray.origin, &mouse_ray.direction) {
                continue;
            }

            if let Some(ref hovered_battle_params) = unit_info_hovered.battle_params {
                for (e2, _t2, u2, _selection) in (&entities, &transforms_comp, &units_comp, &selected_units_comp).iter() {
                    if entity.get_id() == e2.get_id() {
                        continue;
                    }

                    let unit_info_selected = self.empires.unit(u2.civilization_id, u2.unit_id);
                    if unit_info_selected.interaction_mode == dat::InteractionMode::NonInteracting {
                        continue;
                    }

                    if let Some(ref selected_battle_params) = unit_info_selected.battle_params {
                        let atks = selected_battle_params.attacks.iter().map(|&(atk, _)| atk).collect::<HashSet<_>>();
                        let arms = hovered_battle_params.armors.iter().map(|&(arm, _)| arm).collect::<HashSet<_>>();
                        if atks.intersection(&arms).any(|_| true) {
                            should_render_attack_cursor = true;
                            break 'outer;
                        }
                    }
                }
                // TODO:  Compare battle_params with the battle_params of the selected units
                // TOOD:  How do we query the selected units?
                // NOTE:  Adding `selected_units_comp` to the iter above means this will only
                //        iterate over selected units which is not what we want.
                // LINKS: http://aok.heavengames.com/university/modding/an-introduction-to-creating-units-with-age-2/
                //        http://aoe.heavengames.com/cgi-bin/aoecgi/display.cgi?action=ct&f=17,6156,125,all
                //        http://dogsofwarvu.com/forum/index.php?topic=98.45
                should_render_attack_cursor = true;
                break;
            }
        }

        if should_render_attack_cursor {
            let decal = arg.create();
            transforms_comp.insert(decal, TransformComponent::new(mouse_ray.world_coord, 0.into()));
            let decal_movement_cursor = DecalComponent::new(1.into(), DrsKey::Interfac, 51008.into());
            decals_comp.insert(decal, decal_movement_cursor);
        }

        if mouse_state_rc.key_states.key_state(MouseButton::Left) == KeyState::TransitionUp {
            // Holding the left shift key while left clicking a unit will add them to the current selection.
            if keyboard_state_rc.is_up(Key::ShiftLeft) {
                selected_units_comp.clear();
            }

            for (entity, _, unit, transform) in (&entities, &on_screen_comp, &units_comp, &transforms_comp).iter() {
                let unit_info = self.empires.unit(unit.civilization_id, unit.unit_id);
                if unit_info.interaction_mode != dat::InteractionMode::NonInteracting {
                    let unit_box = unit_util::selection_box(unit_info, transform);

                    // Cast a ray from the mouse position through to the terrain and select any unit
                    // whose axis-aligned box intersects the ray.
                    if unit_box.intersects_ray(&mouse_ray.origin, &mouse_ray.direction) {
                        selected_units_comp.insert(entity, SelectedUnitComponent);
                        break;
                    }
                }
            }
        }

        if mouse_state_rc.key_states.key_state(MouseButton::Right) == KeyState::TransitionUp {
            let mut moving_unit = false;
            for (entity, transform, unit, _selected_unit) in (&entities, &transforms_comp, &units_comp, &selected_units_comp).iter() {
                if unit.player_id != players_rc.local_player().player_id {
                    continue;
                }

                let unit_info = self.empires.unit(unit.civilization_id, unit.unit_id);
                let path = path_finder_rc.find_path(&*terrain_rc,
                                                    &*occupied_tiles_rc,
                                                    transform.position(),
                                                    &mouse_ray.world_coord,
                                                    unit_info.terrain_restriction);
                // Enqueue sequential actions by holding left-control.
                if keyboard_state_rc.is_up(Key::CtrlLeft) {
                    action_batcher_rc.queue_for_entity(entity.get_id(), Action::ClearQueue);
                }

                let params = MoveToPositionParams::new(path);
                let action = Action::MoveToPosition(params);
                action_batcher_rc.queue_for_entity(entity.get_id(), action);

                moving_unit = true;
            }

            if moving_unit {
                let decal = arg.create();
                transforms_comp.insert(decal, TransformComponent::new(mouse_ray.world_coord, 0.into()));

                let decal_movement_cursor = DecalComponent::new(0.into(), DrsKey::Interfac, 50405.into());
                decals_comp.insert(decal, decal_movement_cursor);
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
