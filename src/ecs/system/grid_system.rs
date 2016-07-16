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

use ecs::{TransformComponent, VisibleUnitComponent};
use ecs::resource::{ViewProjector, Viewport};
use partition::GridPartition;

use nalgebra::{Cast, Vector2};

use specs::{self, Join};

/// System the updates the grid partition with the latest entity positions
pub struct GridSystem;

impl GridSystem {
    pub fn new() -> GridSystem {
        GridSystem
    }
}

impl specs::System<()> for GridSystem {
    fn run(&mut self, arg: specs::RunArg, _context: ()) {
        let (entities, transforms, mut visible_units, viewport, projector, mut grid) =
            arg.fetch(|w| {
                (w.entities(),
                 w.read::<TransformComponent>(),
                 w.write::<VisibleUnitComponent>(),
                 w.read_resource::<Viewport>(),
                 w.read_resource::<ViewProjector>(),
                 w.write_resource::<GridPartition>())
            });

        let (start_region, end_region) = determine_visible_region(&viewport, &projector);
        let visible_entities = grid.query(&start_region, &end_region);

        visible_units.clear();
        for (entity, transform) in (&entities, &transforms).iter() {
            grid.update_entity(entity.get_id(),
                               &Vector2::new(transform.position.x as i32,
                                             transform.position.y as i32));

            if visible_entities.contains(&entity.get_id()) {
                visible_units.insert(entity, VisibleUnitComponent);
            }
        }
    }
}

fn determine_visible_region(viewport: &Viewport,
                            projector: &ViewProjector)
                            -> (Vector2<i32>, Vector2<i32>) {
    // Because std::cmp::min requires Ord, and f32 doesn't implement Ord
    let fmin = |a, b| if a < b { a } else { b };
    let fmax = |a, b| if a > b { a } else { b };

    let top_left = projector.unproject(&Cast::from(viewport.top_left));
    let top_right =
        projector.unproject(&Cast::from(viewport.top_left + Vector2::new(viewport.size.x, 0f32)));
    let bottom_left =
        projector.unproject(&Cast::from(viewport.top_left + Vector2::new(0f32, viewport.size.y)));
    let bottom_right = projector.unproject(&Cast::from((viewport.top_left + viewport.size)));

    let min_world: Vector2<i32> =
        Cast::from(Vector2::new(fmin(top_left.x,
                                     fmin(top_right.x, fmin(bottom_left.x, bottom_right.x))),
                                fmin(top_left.y,
                                     fmin(top_right.y, fmin(bottom_left.y, bottom_right.y)))));
    let max_world: Vector2<i32> =
        Cast::from(Vector2::new(fmax(top_left.x,
                                     fmax(top_right.x, fmax(bottom_left.x, bottom_right.x))),
                                fmax(top_left.y,
                                     fmax(top_right.y, fmax(bottom_left.y, bottom_right.y)))));
    (min_world, max_world)
}
