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

use std::fmt;

/// Relative elevation levels encoded in a u16 in this format:
/// [WW NW NN SW NE SS SE EE]
/// 0 = lower terrain,
/// 1 = current terrain level,
/// 2 = higher terrain
/// That way, if you subtract one, you get a nice [-1, 0, 1] gradient.
#[derive(Copy, Clone)]
pub struct ElevationMatrix(u16);

impl ElevationMatrix {
    pub fn new() -> ElevationMatrix {
        ElevationMatrix(0x5555u16) // All elevation levels 01 (to indicate current level)
    }

    pub fn set_elevation_at(&mut self, direction: usize, relative_elevation: i8) {
        let val = if relative_elevation == 0 {
            1
        } else if relative_elevation < 0 {
            0
        } else {
            2
        };
        let shift = shift_val(direction);
        self.0 = self.0 & !(3 << shift);
        self.0 = self.0 | (val << shift);
    }
}

impl fmt::Debug for ElevationMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use super::dir::*;
        let at = &|direction| ((self.0 >> shift_val(direction)) & 3) as isize - 1;
        write!(f,
               "[{}, {}, {},\n {}, 0, {},\n {}, {}, {}] = {:016b}",
               at(W),
               at(NW),
               at(N),
               at(SW),
               at(NE),
               at(S),
               at(SE),
               at(E),
               self.0)
    }
}

#[derive(Clone)]
pub struct ElevationGraphic {
    pub index: u8,
    pub render_offset: f32,
}

impl ElevationGraphic {
    pub fn new(index: u8, render_offset: f32) -> ElevationGraphic {
        ElevationGraphic {
            index: index,
            render_offset: render_offset,
        }
    }
}

pub struct ElevationMatch {
    matcher: u16,
    pub elevation_graphics: Vec<ElevationGraphic>,
}

impl ElevationMatch {
    pub fn new(matcher: u16, elevation_graphics: Vec<ElevationGraphic>) -> ElevationMatch {
        ElevationMatch {
            matcher: matcher,
            elevation_graphics: elevation_graphics,
        }
    }

    pub fn find_match(elevation_matrix: ElevationMatrix) -> Option<&'static ElevationMatch> {
        for elevation_match in &*ELEVATION_MATCHES {
            if elevation_matrix.0 == elevation_match.matcher {
                return Some(elevation_match);
            }
        }
        None
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)] // rustfmt doesn't understand comments inside macros
lazy_static! {
    static ref ELEVATION_MATCHES: Vec<ElevationMatch> = {
        use super::dir::*;
        let mut matches = Vec::new();

        let m = |matcher, graphics| ElevationMatch::new(matcher, graphics);
        let g = |index, offset| ElevationGraphic::new(index, offset);

        // TODO: I think there should be 56 of these to cover every possible permutation
        // TODO: May be a way to simplify this too

        // Flat terrain
        matches.push(m(matcher(&[], &[]), vec![g(0, 0.)]));

        // North corners
        matches.push(ElevationMatch::new(matcher(&[], &[N]), vec![g(2, 1.)]));
        // There isn't a complete graphic for an inset North corner
        // It looks like the game smears a bunch of other graphics together to fake it
        let north_inset_corner = vec![g(0, 0.5), g(0, 0.75), g(0, 1.), g(10, 0.5)];
        matches.push(ElevationMatch::new(matcher(&[], &[W, NW, N, NE, E]), north_inset_corner.clone()));
        matches.push(ElevationMatch::new(matcher(&[], &[W, NW, N, NE]), north_inset_corner.clone()));

        // South corners
        matches.push(ElevationMatch::new(matcher(&[], &[S]), vec![g(3, 1.)]));

        // East corners
        matches.push(ElevationMatch::new(matcher(&[], &[E]), vec![g(4, 0.)]));
        matches.push(ElevationMatch::new(matcher(&[], &[N, NE, E, SE, S]), vec![g(4, 0.)]));

        // South West sides
        matches.push(ElevationMatch::new(matcher(&[], &[SW, S]), vec![g(5, 0.)]));
        matches.push(ElevationMatch::new(matcher(&[], &[W, SW]), vec![g(5, 0.)]));
        matches.push(ElevationMatch::new(matcher(&[], &[W, SW, S]), vec![g(5, 0.)]));

        // North West sides
        matches.push(ElevationMatch::new(matcher(&[], &[W, N]), vec![g(6, 1.)]));
        matches.push(ElevationMatch::new(matcher(&[], &[NW, N]), vec![g(6, 1.)]));
        matches.push(ElevationMatch::new(matcher(&[], &[W, NW, N]), vec![g(6, 1.)]));

        // South East sides
        matches.push(ElevationMatch::new(matcher(&[], &[S, SE]), vec![g(7, 0.)]));
        matches.push(ElevationMatch::new(matcher(&[], &[SE, E]), vec![g(7, 0.)]));
        matches.push(ElevationMatch::new(matcher(&[], &[S, SE, E]), vec![g(7, 0.)]));

        // North East sides
        matches.push(ElevationMatch::new(matcher(&[], &[NE, E]), vec![g(8, 1.)]));
        matches.push(ElevationMatch::new(matcher(&[], &[N, NE]), vec![g(8, 1.)]));
        matches.push(ElevationMatch::new(matcher(&[], &[N, NE, E]), vec![g(8, 1.)]));

        matches
    };
}

fn matcher(low: &[usize], high: &[usize]) -> u16 {
    let mut matrix = ElevationMatrix::new();
    for direction in low {
        matrix.set_elevation_at(*direction, -1);
    }
    for direction in high {
        matrix.set_elevation_at(*direction, 1);
    }
    matrix.0
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
