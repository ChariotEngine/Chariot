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

/// Relative elevation levels encoded in a u8 in this format:
/// [W NW N SW NE S SE E]
/// 0 = current/lower terrain,
/// 1 = higher terrain
#[derive(Copy, Clone)]
pub struct ElevationMatrix(u8);

impl ElevationMatrix {
    pub fn new() -> ElevationMatrix {
        ElevationMatrix(0)
    }

    pub fn set_elevation_at(&mut self, direction: usize, relative_elevation: i8) {
        let val = if relative_elevation <= 0 { 0 } else { 1 };
        self.0 = self.0 | (val << shift_val(direction));
    }
}

impl fmt::Debug for ElevationMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use super::dir::*;
        let at = &|direction| ((self.0 >> shift_val(direction)) & 1) as isize;
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
    pub render_offset_y: f32,
}

impl ElevationGraphic {
    pub fn new(index: u8, render_offset_y: f32) -> ElevationGraphic {
        ElevationGraphic {
            index: index,
            render_offset_y: render_offset_y,
        }
    }
}

pub struct ElevationMatch {
    matcher: u8,
    pub elevation_graphics: Vec<ElevationGraphic>,
}

impl ElevationMatch {
    pub fn new(matcher: u8, elevation_graphics: Vec<ElevationGraphic>) -> ElevationMatch {
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
        let g = |index, offset_y| ElevationGraphic::new(index, offset_y);

        let flat = vec![g(0, 0.)];
        matches.push(m(matcher(&[]), flat.clone()));

        let north_corner = vec![g(2, 1.)];
        matches.push(m(matcher(&[N]), north_corner.clone()));

        let north_inset_corner = vec![g(14, 1.)];
        matches.push(m(matcher(&[NW, N, NE]), north_inset_corner.clone()));
        matches.push(m(matcher(&[W, NW, N, NE]), north_inset_corner.clone()));
        matches.push(m(matcher(&[NW, N, NE, E]), north_inset_corner.clone()));
        matches.push(m(matcher(&[W, NW, N, NE, E]), north_inset_corner.clone()));

        let south_corner = vec![g(1, 0.)];
        matches.push(m(matcher(&[S]), south_corner.clone()));

        let south_inset_corner = vec![g(13, 0.)];
        matches.push(m(matcher(&[SE, S, SW]), south_inset_corner.clone()));
        matches.push(m(matcher(&[SE, S, SW, W]), south_inset_corner.clone()));
        matches.push(m(matcher(&[E, SE, S, SW]), south_inset_corner.clone()));
        matches.push(m(matcher(&[E, SE, S, SW, W]), south_inset_corner.clone()));

        let east_corner = vec![g(4, 0.)];
        matches.push(m(matcher(&[E]), east_corner.clone()));

        let east_inset_corner = vec![g(15, 1.)];
        matches.push(m(matcher(&[NE, E, SE]), east_inset_corner.clone()));
        matches.push(m(matcher(&[N, NE, E, SE]), east_inset_corner.clone()));
        matches.push(m(matcher(&[NE, E, SE, S]), east_inset_corner.clone()));
        matches.push(m(matcher(&[N, NE, E, SE, S]), east_inset_corner.clone()));

        let west_corner = vec![g(11, 0.)];
        matches.push(m(matcher(&[W]), west_corner.clone()));

        let west_inset_corner = vec![g(16, 1.)];
        matches.push(m(matcher(&[SW, W, NW]), west_inset_corner.clone()));
        matches.push(m(matcher(&[SW, W, NW, N]), west_inset_corner.clone()));
        matches.push(m(matcher(&[S, SW, W, NW]), west_inset_corner.clone()));
        matches.push(m(matcher(&[S, SW, W, NW, N]), west_inset_corner.clone()));

        let south_west_side = vec![g(5, 0.)];
        matches.push(m(matcher(&[SW]), south_west_side.clone()));
        matches.push(m(matcher(&[SW, S]), south_west_side.clone()));
        matches.push(m(matcher(&[W, SW]), south_west_side.clone()));
        matches.push(m(matcher(&[W, SW, S]), south_west_side.clone()));

        let north_west_side = vec![g(6, 1.)];
        matches.push(m(matcher(&[NW]), north_west_side.clone()));
        matches.push(m(matcher(&[W, NW]), north_west_side.clone()));
        matches.push(m(matcher(&[NW, N]), north_west_side.clone()));
        matches.push(m(matcher(&[W, NW, N]), north_west_side.clone()));

        let south_east_side = vec![g(7, 0.)];
        matches.push(m(matcher(&[SE]), south_east_side.clone()));
        matches.push(m(matcher(&[S, SE]), south_east_side.clone()));
        matches.push(m(matcher(&[SE, E]), south_east_side.clone()));
        matches.push(m(matcher(&[S, SE, E]), south_east_side.clone()));

        let north_east_side = vec![g(8, 1.)];
        matches.push(m(matcher(&[NE]), north_east_side.clone()));
        matches.push(m(matcher(&[NE, E]), north_east_side.clone()));
        matches.push(m(matcher(&[N, NE]), north_east_side.clone()));
        matches.push(m(matcher(&[N, NE, E]), north_east_side.clone()));

        matches
    };
}

fn matcher(high_elevation: &[usize]) -> u8 {
    let mut matrix = ElevationMatrix::new();
    for direction in high_elevation {
        matrix.set_elevation_at(*direction, 1);
    }
    matrix.0
}

#[inline(always)]
fn shift_val(val: usize) -> usize {
    if val > dir::SW { val - 1 } else { val }
}
