// Chariot: An open source reimplementation of Age of Empires (1997)
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

use nalgebra;
use super::Fixed;

pub type Vector3 = nalgebra::Vector3<Fixed>;

pub trait Norm {
    fn length_squared(&self) -> Fixed;
    fn length(&self) -> Fixed;

    fn normalized(&self) -> Vector3;
    fn normalize(&mut self) -> Fixed;
}

// Couldn't realistically use nalgebra's Norm trait because it requires the Float trait
// which just doesn't make sense to implement for fixed point numbers. Hence, roll our own.
impl Norm for Vector3 {
    #[inline]
    fn length_squared(&self) -> Fixed {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    fn length(&self) -> Fixed {
        self.length_squared().sqrt()
    }

    fn normalized(&self) -> Vector3 {
        let len = self.length();
        *self / len
    }

    fn normalize(&mut self) -> Fixed {
        let len = self.length();
        *self /= len;
        len
    }
}

#[cfg(test)]
mod test {
    use num::ToPrimitive;
    use super::*;
    use super::super::Fixed;

    fn vec3_string(vec3: &Vector3) -> String {
        format!("({}, {}, {})",
            vec3.x.to_f32().unwrap(),
            vec3.y.to_f32().unwrap(),
            vec3.z.to_f32().unwrap()
        )
    }

    #[test]
    fn test_typical_normalize_use_case() {
        let position: Vector3 = Vector3::new(100.into(), 100.into(), 1.into());
        let velocity: Vector3 = Vector3::new(1.into(), 2.into(), 0.5.into());
        let time: Fixed = 0.0166666667.into();

        let new_position = position + velocity * time;
        let delta = new_position - position;
        let direction = delta.normalized();
        let direction_length = direction.length();

        assert_eq!("(100.01667, 100.03333, 1.0083333)", &vec3_string(&new_position));
        assert_eq!("(0.01666665, 0.0333333, 0.008333325)", &vec3_string(&delta));
        assert_eq!("(0.4364425, 0.87288505, 0.21822125)", &vec3_string(&direction));
        assert_eq!(1, direction_length.to_i32().unwrap());
    }

    // Commented out benchmarks since they don't compile on stable rustc
    // Wrapped in a function so that rustfmt doesn't touch the comment formatting
    // use test::{self, Bencher};
    // use nalgebra;
    // use num::FromPrimitive;
    #[cfg_attr(rustfmt, rustfmt_skip)]
    #[test]
    fn test_bench_wrapper() {
        /*
        macro_rules! bench_typical {
            ($name:ident, $typ:ident, $norm:path, $len:path) => {
                #[bench]
                fn $name(bench: &mut Bencher) {
                    type NVec3 = nalgebra::Vector3<$typ>;

                    let position: NVec3 = NVec3::new($typ::from_i32(100).unwrap(),
                                                     $typ::from_i32(100).unwrap(),
                                                     $typ::from_i32(1).unwrap());
                    let velocity: NVec3 = NVec3::new($typ::from_i32(1).unwrap(),
                                                     $typ::from_i32(2).unwrap(),
                                                     $typ::from_f64(0.5).unwrap());
                    let time = $typ::from_f64(0.0166666667).unwrap();

                    bench.iter(|| {
                        let new_position = test::black_box(position) +
                            test::black_box(velocity) * test::black_box(time);
                        let delta = new_position - test::black_box(position);
                        let direction = $norm(&delta);
                        let direction_length = $len(&direction);
                        direction_length
                    });
                }
            }
        }

        bench_typical!(bench_fixed_typical_vector_use_case,
                       Fixed,
                       Norm::normalized,
                       Norm::length);
        bench_typical!(bench_f32_typical_vector_use_case,
                       f32,
                       nalgebra::Norm::normalize,
                       nalgebra::Norm::norm);
        bench_typical!(bench_f64_typical_vector_use_case,
                       f64,
                       nalgebra::Norm::normalize,
                       nalgebra::Norm::norm);
       */
    }
}
