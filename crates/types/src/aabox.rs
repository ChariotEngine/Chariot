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

use num::Zero;
use super::{Fixed, ToFixed, Vector3};
use std::cmp;

/// Axis-aligned box
#[derive(Copy, Clone, Debug)]
pub struct AABox {
    pub min: Vector3,
    pub max: Vector3,
}

impl AABox {
    pub fn new(first: Vector3, second: Vector3) -> AABox {
        AABox {
            min: Vector3::new(cmp::min(first.x, second.x),
                              cmp::min(first.y, second.y),
                              cmp::min(first.z, second.z)),
            max: Vector3::new(cmp::max(first.x, second.x),
                              cmp::max(first.y, second.y),
                              cmp::max(first.z, second.z)),
        }
    }

    pub fn contains(&self, point: &Vector3) -> bool {
        self.min.x <= point.x && self.min.y <= point.y && self.min.z <= point.z && self.max.x >= point.x &&
        self.max.y >= point.y && self.max.z >= point.z
    }

    pub fn intersects_ray(&self, origin: &Vector3, direction: &Vector3) -> bool {
        // Implementation ported from:
        // https://github.com/erich666/GraphicsGems/blob/master/gems/RayBox.c
        // Credit goes to the authors of Graphics Gems
        let (left, right, middle) = (1, 0, 2);
        let mut quadrants = [0u8; 3];
        let mut max_t = [Fixed::zero(); 3];
        let mut selected_plane = 0;
        let mut candidate_planes = [Fixed::zero(); 3];
        let mut inside = true;

        // Find candidate planes and determine if the origin is inside of the box
        for i in 0..3 {
            if origin[i] < self.min[i] {
                quadrants[i] = left;
                candidate_planes[i] = self.min[i];
                inside = false;
            } else if origin[i] > self.max[i] {
                quadrants[i] = right;
                candidate_planes[i] = self.max[i];
                inside = false;
            } else {
                quadrants[i] = middle;
            }
        }

        if !inside {
            // Caclulate T distances to candidate planes
            for i in 0..3 {
                if quadrants[i] != middle && direction[i] != 0.to_fixed() {
                    max_t[i] = (candidate_planes[i] - origin[i]) / direction[i];
                } else {
                    max_t[i] = -1.to_fixed();
                }
            }

            // Find largest max_t for final choice of intersection
            for i in 1..3 {
                if max_t[selected_plane] < max_t[i] {
                    selected_plane = i;
                }
            }

            // Check if the final candidate is actually inside the box
            if max_t[selected_plane] < 0.to_fixed() {
                return false;
            }
            for i in 0..3 {
                if selected_plane != i {
                    let coord = origin[i] + max_t[selected_plane] * direction[i];
                    if coord < self.min[i] || coord > self.max[i] {
                        return false;
                    }
                }
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::AABox;
    use super::super::Vector3;

    #[test]
    fn test_contains() {
        let aabox = AABox::new(Vector3::new(0.into(), 0.into(), 0.into()),
                               Vector3::new(1.into(), 2.into(), 3.into()));

        assert!(aabox.contains(&Vector3::new(0.0.into(), 0.0.into(), 0.0.into())));
        assert!(aabox.contains(&Vector3::new(0.5.into(), 1.0.into(), 0.0.into())));
        assert!(aabox.contains(&Vector3::new(0.5.into(), 1.0.into(), 1.0.into())));
        assert!(aabox.contains(&Vector3::new(1.0.into(), 2.0.into(), 3.0.into())));

        assert!(!aabox.contains(&Vector3::new((-0.1).into(), 0.0.into(), 0.0.into())));
        assert!(!aabox.contains(&Vector3::new(0.0.into(), (-0.1).into(), 0.0.into())));
        assert!(!aabox.contains(&Vector3::new(0.0.into(), 0.0.into(), (-0.1).into())));
        assert!(!aabox.contains(&Vector3::new(1.1.into(), 2.0.into(), 3.0.into())));
        assert!(!aabox.contains(&Vector3::new(1.0.into(), 2.1.into(), 3.0.into())));
        assert!(!aabox.contains(&Vector3::new(1.0.into(), 2.0.into(), 3.1.into())));
    }

    #[test]
    fn test_intersects_ray() {
        let aabox = AABox::new(Vector3::new(0.into(), 0.into(), 0.into()),
                               Vector3::new(1.into(), 1.into(), 1.into()));

        let origin = Vector3::new(0.5.into(), 1.2.into(), 1.5.into());
        let end = Vector3::new(0.5.into(), (-0.2).into(), 0.5.into());
        let dir = end - origin;
        assert!(aabox.intersects_ray(&origin, &dir));

        let origin = Vector3::new(1.5.into(), 1.2.into(), 1.5.into());
        let end = Vector3::new(1.5.into(), (-0.2).into(), 0.5.into());
        let dir = end - origin;
        assert!(!aabox.intersects_ray(&origin, &dir));
    }
}
