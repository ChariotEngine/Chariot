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

use ecs::{TransformComponent, VelocityComponent};
use nalgebra::Vector2;
use partition::GridPartition;
use specs::{self, Join};
use super::System;
use types::{Fixed, Norm};

pub struct VelocitySystem;

const MOVEMENT_THRESHOLD: Fixed = fixed_const!(0.001);

impl VelocitySystem {
    pub fn new() -> VelocitySystem {
        VelocitySystem
    }
}

impl System for VelocitySystem {
    fn update(&mut self, arg: specs::RunArg, time_step: Fixed) {
        fetch_components!(arg, entities, [
            components(velocities: VelocityComponent),
            mut components(transforms: TransformComponent),
            mut resource(grid: GridPartition),
        ]);

        for (entity, transform, velocity) in (&entities, &mut transforms, &velocities).iter() {
            if !grid.contains(entity.get_id()) || velocity.velocity.length_squared() > MOVEMENT_THRESHOLD {
                let new_pos = *transform.position() + velocity.velocity * time_step;
                transform.set_position(new_pos);

                transform.rotation = {
                    // As long as the direction isn't used for anything more than rendering,
                    // it should be fine to use the floating point arc tangent.
                    // Otherwise, we'll need to implement that in the fixed point code.
                    let vel_x_floating: f32 = velocity.velocity.x.into();
                    let vel_y_floating: f32 = velocity.velocity.y.into();
                    vel_y_floating.atan2(vel_x_floating).into()
                };

                grid.update_entity(entity.get_id(),
                                   &Vector2::new(new_pos.x.into(), new_pos.y.into()));
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
