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

use super::dir;

pub struct ElevationMatrix(u16);

impl ElevationMatrix {
    pub fn new() -> ElevationMatrix {
        ElevationMatrix(0u16)
    }

    pub fn set_elevation_at(&mut self, direction: usize, relative_elevation: i8) {
        let val = if relative_elevation == 0 {
            1
        } else if relative_elevation < 0 {
            0
        } else {
            2
        };
        self.0 = self.0 | (val << shift_val(direction));
    }
}

pub struct ElevationMatch {
    matcher: u16,
    pub elevation_indices: Vec<u8>,
}

impl ElevationMatch {
    fn new(matcher: u16, elevation_indices: Vec<u8>) -> ElevationMatch {
        ElevationMatch {
            matcher: matcher,
            elevation_indices: elevation_indices,
        }
    }

    pub fn find_match(elevation_matrix: ElevationMatrix) -> Option<&'static ElevationMatch> {
        for elevation_match in &*ELEVATION_MATCHES {
            if elevation_match.matcher & elevation_matrix.0 == elevation_match.matcher {
                return Some(elevation_match);
            }
        }
        None
    }
}

lazy_static! {
    static ref ELEVATION_MATCHES: Vec<ElevationMatch> = {
        let mut matches = Vec::new();
        // TODO
        matches
    };
}

#[inline(always)]
fn shift_val(val: usize) -> usize {
    2 *
    (if val > dir::SW {
        val - 1
    } else {
        val
    })
}
