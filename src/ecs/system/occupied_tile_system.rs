// Chariot: An open source reimplementation of Age of Empires (1997)
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
use ecs::{TransformComponent, UnitComponent};
use ecs::resource::OccupiedTiles;
use specs::{self, Join};
use super::System;
use types::{Fixed, ToPrimitive};
use util::unit;

pub struct OccupiedTileSystem {
    empires: dat::EmpiresDbRef,
}

impl OccupiedTileSystem {
    pub fn new(empires: dat::EmpiresDbRef) -> OccupiedTileSystem {
        OccupiedTileSystem { empires: empires }
    }
}

impl System for OccupiedTileSystem {
    fn update(&mut self, arg: specs::RunArg, _time_step: Fixed) {
        fetch_components!(arg, _entities, [
            components(transforms: TransformComponent),
            components(units: UnitComponent),
            mut resource(occupied_tiles: OccupiedTiles),
        ]);

        occupied_tiles.tiles.clear();
        for (transform, unit) in (&transforms, &units).iter() {
            let unit_info = self.empires.unit(unit.civilization_id, unit.unit_id);
            let unit_blocks_tiles = match unit_info.interaction_mode {
                dat::InteractionMode::Building => true,
                dat::InteractionMode::Resource => {
                    match unit_info.motion_params {
                        Some(ref params) => params.speed < 0.001f32,
                        None => true,
                    }
                }
                _ => false,
            };

            if unit_blocks_tiles {
                let collision_box = unit::collision_box(unit_info, transform);

                let (start_row, end_row) = (collision_box.min.y.to_i32().unwrap(),
                                            collision_box.max.y.to_i32().unwrap());
                let (start_col, end_col) = (collision_box.min.x.to_i32().unwrap(),
                                            collision_box.max.x.to_i32().unwrap());
                for row in start_row..(end_row + 1) {
                    for col in start_col..(end_col + 1) {
                        occupied_tiles.tiles.insert((row, col));
                    }
                }
            }
        }
    }
}
