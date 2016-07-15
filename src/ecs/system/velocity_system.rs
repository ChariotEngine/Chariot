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

use specs::{self, Join};

pub struct VelocitySystem;

impl VelocitySystem {
    pub fn new() -> VelocitySystem {
        VelocitySystem
    }
}

impl specs::System<()> for VelocitySystem {
    fn run(&mut self, arg: specs::RunArg, _context: ()) {
        let (mut transforms, velocities) =
            arg.fetch(|w| (w.write::<TransformComponent>(), w.read::<VelocityComponent>()));

        for (transform, velocity) in (&mut transforms, &velocities).iter() {
            // TODO: Factor in time delta
            (*transform).position.x += velocity.velocity.x;
            (*transform).position.y += velocity.velocity.y;
            (*transform).position.z += velocity.velocity.z;
        }
    }
}
