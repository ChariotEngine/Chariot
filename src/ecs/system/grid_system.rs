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

use ecs::OnScreenComponent;
use ecs::resource::{Terrain, ViewProjector, Viewport};
use nalgebra::Vector2;
use partition::GridPartition;
use specs::{self, Join};
use super::System;
use types::Fixed;

/// System the updates the grid partition with the latest entity positions
pub struct GridSystem;

impl GridSystem {
    pub fn new() -> GridSystem {
        GridSystem
    }
}

impl System for GridSystem {
    fn update(&mut self, arg: specs::RunArg, _time_step: Fixed) {
        let (entities, mut on_screen, viewport, projector, grid, terrain) = arg.fetch(|w| {
            (w.entities(),
             w.write::<OnScreenComponent>(),
             w.read_resource::<Viewport>(),
             w.read_resource::<ViewProjector>(),
             w.read_resource::<GridPartition>(),
             w.read_resource::<Terrain>())
        });

        let visible_region = projector.calculate_visible_world_coords(&viewport, &*terrain);
        let start_region = Vector2::new(visible_region.x, visible_region.y);
        let end_region = start_region + Vector2::new(visible_region.w, visible_region.h);
        let visible_entities = grid.query(&start_region, &end_region);

        on_screen.clear();
        for entity in (&entities).iter() {
            if visible_entities.contains(&entity.get_id()) {
                on_screen.insert(entity, OnScreenComponent);
            }
        }
    }
}
