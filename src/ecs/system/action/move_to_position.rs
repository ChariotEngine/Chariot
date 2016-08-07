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

use ecs::component::*;
use specs::{self, Join};
use super::super::System;
use types::{Fixed, Norm, Vector3};

const THRESHOLD: Fixed = fixed_const!(0.1);

pub struct MoveToPositionActionSystem {
}

impl MoveToPositionActionSystem {
    pub fn new() -> MoveToPositionActionSystem {
        MoveToPositionActionSystem {}
    }
}

impl System for MoveToPositionActionSystem {
    fn update(&mut self, arg: specs::RunArg, _time_step: Fixed) {
        let (mut velocities, transforms, mtps, mut action_queues) = arg.fetch(|w| {
            (w.write::<VelocityComponent>(),
             w.read::<TransformComponent>(),
             w.read::<MoveToPositionActionComponent>(),
             w.write::<ActionQueueComponent>())
        });

        for (mut velocity, transform, mtps, mut action_queue) in (&mut velocities,
                                                                  &transforms,
                                                                  &mtps,
                                                                  &mut action_queues)
            .iter() {
            let mut direction = mtps.target - *transform.position();
            let distance = direction.normalize();

            if distance <= THRESHOLD {
                velocity.velocity = Vector3::new(0.into(), 0.into(), 0.into());
                action_queue.mark_current_done();
            } else {
                // TODO: Plug in the action speed numbers
                let speed: Fixed = 2.into();
                velocity.velocity = direction * speed;
            }
        }
    }
}
