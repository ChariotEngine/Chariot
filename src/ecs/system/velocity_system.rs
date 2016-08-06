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
use ecs::{TransformComponent, VelocityComponent};
use partition::GridPartition;

use nalgebra::{Norm, Vector2};

use specs::{self, Join};

pub struct VelocitySystem;

impl VelocitySystem {
    pub fn new() -> VelocitySystem {
        VelocitySystem
    }
}

impl System for VelocitySystem {
    fn update(&mut self, arg: specs::RunArg, time_step: f32) {
        let (entities, mut transforms, velocities, mut grid) = arg.fetch(|w| {
            (w.entities(),
             w.write::<TransformComponent>(),
             w.read::<VelocityComponent>(),
             w.write_resource::<GridPartition>())
        });

        for (entity, transform, velocity) in (&entities, &mut transforms, &velocities).iter() {
            if !grid.contains(entity.get_id()) || velocity.velocity.norm_squared() > 0.001 {
                let new_pos = *transform.position() + velocity.velocity * time_step;
                transform.set_position(new_pos);

                grid.update_entity(entity.get_id(),
                                   &Vector2::new(new_pos.x as i32, new_pos.y as i32));
            } else {
                // This ensures that both the previous and current position match
                // so that entities can't stutter back and forth between those values
                // on sub-update render interpolation.
                let position = *transform.position();
                transform.set_position(position);
            }
        }
    }
}
