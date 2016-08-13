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

use nalgebra::Vector2;
use types::Fixed;

pub struct Viewport {
    current_top_left: Vector2<Fixed>,
    last_top_left: Vector2<Fixed>,
    pub size: Vector2<i32>,
}

impl Viewport {
    pub fn new(w: i32, h: i32) -> Viewport {
        Viewport {
            current_top_left: Vector2::new(0.into(), 0.into()),
            last_top_left: Vector2::new(0.into(), 0.into()),
            size: Vector2::new(w, h),
        }
    }

    pub fn top_left_i32(&self) -> Vector2<i32> {
        Vector2::new(self.current_top_left.x.into(),
                     self.current_top_left.y.into())
    }

    pub fn top_left<'a>(&'a self) -> &'a Vector2<Fixed> {
        &self.current_top_left
    }

    pub fn set_top_left(&mut self, top_left: Vector2<Fixed>) {
        self.last_top_left = self.current_top_left;
        self.current_top_left = top_left;
    }

    pub fn lerped_top_left(&self, lerp: Fixed) -> Vector2<i32> {
        let lerped = self.current_top_left + (self.current_top_left - self.last_top_left) * lerp;
        Vector2::new(lerped.x.into(), lerped.y.into())
    }
}
