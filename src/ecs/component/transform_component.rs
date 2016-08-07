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

use specs;
use types::{Fixed, Vector3};

#[derive(Clone, Debug)]
pub struct TransformComponent {
    current_position: Vector3,
    last_position: Vector3,
    pub rotation: Fixed,
}

impl specs::Component for TransformComponent {
    type Storage = specs::VecStorage<TransformComponent>;
}

impl TransformComponent {
    pub fn new(position: Vector3, rotation: Fixed) -> TransformComponent {
        TransformComponent {
            current_position: position,
            last_position: position,
            rotation: rotation,
        }
    }

    pub fn position<'a>(&self) -> &Vector3 {
        &self.current_position
    }

    pub fn set_position(&mut self, position: Vector3) {
        self.last_position = self.current_position;
        self.current_position = position;
    }

    pub fn lerped_position(&self, lerp: Fixed) -> Vector3 {
        self.current_position + (self.current_position - self.last_position) * lerp
    }
}
