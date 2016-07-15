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

use ecs::TransformComponent;
use partition::GridPartition;

use specs::{self, Join};

use std::sync::{Arc, RwLock};

/// System the updates the grid partition with the latest entity positions
pub struct GridSystem {
    grid: Arc<RwLock<GridPartition>>,
}

impl GridSystem {
    pub fn new(grid: Arc<RwLock<GridPartition>>) -> GridSystem {
        GridSystem { grid: grid }
    }
}

impl specs::System<()> for GridSystem {
    fn run(&mut self, arg: specs::RunArg, _context: ()) {
        let (entities, transforms) = arg.fetch(|w| (w.entities(), w.read::<TransformComponent>()));

        // TODO: Mark units with the VisibleUnitComponent if on screen

        let mut grid = self.grid.write().unwrap();
        for (entity, transform) in (&entities, &transforms).iter() {
            grid.update_entity(entity.get_id(), (transform.position.x as i32, transform.position.y as i32));
        }
    }
}
