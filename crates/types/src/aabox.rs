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

use nalgebra::Vector3;

/// Axis-aligned box
#[derive(Copy, Clone, Debug)]
pub struct AABox {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
}

impl AABox {
    pub fn new(first: Vector3<f32>, second: Vector3<f32>) -> AABox {
        // Because f32 isn't Ord, we can't use std::cmp::{min, max}
        let min = |a, b| if a < b { a } else { b };
        let max = |a, b| if a > b { a } else { b };

        AABox {
            min: Vector3::new(min(first.x, second.x),
                              min(first.y, second.y),
                              min(first.z, second.z)),
            max: Vector3::new(max(first.x, second.x),
                              max(first.y, second.y),
                              max(first.z, second.z)),
        }
    }

    pub fn contains(&self, point: &Vector3<f32>) -> bool {
        self.min.x <= point.x && self.min.y <= point.y && self.min.z <= point.z && self.max.x >= point.x &&
        self.max.y >= point.y && self.max.z >= point.z
    }
}

#[cfg(test)]
mod tests {
    use super::AABox;
    use nalgebra::Vector3;

    #[test]
    fn test_contains() {
        let aabox = AABox::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 2.0, 3.0));

        assert!(aabox.contains(&Vector3::new(0.0, 0.0, 0.0)));
        assert!(aabox.contains(&Vector3::new(0.5, 1.0, 0.0)));
        assert!(aabox.contains(&Vector3::new(0.5, 1.0, 1.0)));
        assert!(aabox.contains(&Vector3::new(1.0, 2.0, 3.0)));

        assert!(!aabox.contains(&Vector3::new(-0.1, 0.0, 0.0)));
        assert!(!aabox.contains(&Vector3::new(0.0, -0.1, 0.0)));
        assert!(!aabox.contains(&Vector3::new(0.0, 0.0, -0.1)));
        assert!(!aabox.contains(&Vector3::new(1.1, 2.0, 3.0)));
        assert!(!aabox.contains(&Vector3::new(1.0, 2.1, 3.0)));
        assert!(!aabox.contains(&Vector3::new(1.0, 2.0, 3.1)));
    }
}
