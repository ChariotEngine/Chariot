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
use ecs::{CameraComponent, VelocityComponent};
use ecs::resource::KeyboardKeyStates;

use media::Key;

use nalgebra::{Norm, Vector3};
use specs::{self, Join};

// TODO: Doesn't currently match the camera speed in the original game
const CAMERA_SPEED: f32 = 400f32;

pub struct CameraInputSystem;

impl CameraInputSystem {
    pub fn new() -> CameraInputSystem {
        CameraInputSystem
    }
}

impl System for CameraInputSystem {
    fn update(&mut self, arg: specs::RunArg, _time_step: f32) {
        let (mut velocities, cameras, keyboard_key_states) = arg.fetch(|w| {
            (w.write::<VelocityComponent>(),
             w.read::<CameraComponent>(),
             w.read_resource::<KeyboardKeyStates>())
        });

        for (velocity, _camera) in (&mut velocities, &cameras).iter() {
            let mut new_velocity = Vector3::new(0f32, 0f32, 0f32);

            if keyboard_key_states.key_state(Key::Up).is_down() {
                new_velocity.y = -1f32;
            } else if keyboard_key_states.key_state(Key::Down).is_down() {
                new_velocity.y = 1f32;
            }

            if keyboard_key_states.key_state(Key::Left).is_down() {
                new_velocity.x = -1f32;
            } else if keyboard_key_states.key_state(Key::Right).is_down() {
                new_velocity.x = 1f32;
            }

            if new_velocity.norm() > 0.0000001f32 {
                new_velocity.normalize_mut();
            }
            velocity.velocity = new_velocity * CAMERA_SPEED;
        }
    }
}
