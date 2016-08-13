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


use std::fmt;
use super::dir;

#[derive(Copy, Clone)]
pub struct BorderMatrix(u16);

impl BorderMatrix {
    pub fn new() -> BorderMatrix {
        BorderMatrix(0u16)
    }

    pub fn set_at(&mut self, direction: usize, border: bool, current: bool) {
        if border {
            self.0 = self.0 | (1 << shift_val(direction));
        } else if current {
            self.0 = self.0 | (0x100 << shift_val(direction));
        }
    }
}

impl fmt::Debug for BorderMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use super::dir::*;
        let at = &|direction| {
            let shift = shift_val(direction);
            if self.0 >> shift & 1 == 1 {
                "b"
            } else if self.0 >> (shift + 8) & 1 == 1 {
                "m"
            } else {
                "?"
            }
        };
        write!(f,
               "[{}, {}, {},\n {}, m, {},\n {}, {}, {}] = {:016b}",
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

/// Struct that matches a local terrain pattern and indicates which terrain border indices
/// should be used for that given match. The matcher itself is a local terrain matrix [3x3]
/// encoded in a u16 with the format [CCCCCCCC BBBBBBBB] where C indicates the tile at that
/// location matches the tile in the center, and B indicates there is a border tile at that
/// location (border tile being a tile that doesn't match the center tile).
pub struct BorderMatch {
    matcher: u16,
    pub border_indices: Vec<u16>,
}

impl BorderMatch {
    fn new(matcher: u16, border_indices: Vec<u16>) -> BorderMatch {
        BorderMatch {
            matcher: matcher,
            border_indices: border_indices,
        }
    }

    pub fn find_match(border_style: i16, border_matrix: BorderMatrix) -> Option<&'static BorderMatch> {
        let border_style = match border_style {
            0 => &*BORDER_STYLE_0,
            1 => &*BORDER_STYLE_1,
            _ => panic!("Unknown border style {}", border_style),
        };

        for border_match in border_style {
            if border_match.matcher & border_matrix.0 == border_match.matcher {
                return Some(border_match);
            }
        }
        None
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)] // rustfmt doesn't understand comments inside macros
lazy_static! {
    static ref BORDER_STYLE_0: Vec<BorderMatch> = {
        use super::dir::*;
        let mut matches = Vec::new();

        //
        // Order must remain most specific -> least specific
        //

        // Corners
        matches.push(BorderMatch::new(border_matcher(&[S, SW, W, NW, N], &[SE, NE]), vec![0]));
        matches.push(BorderMatch::new(border_matcher(&[W, NW, N, NE, E], &[SW, SE]), vec![1]));
        matches.push(BorderMatch::new(border_matcher(&[W, SW, S, SE, E], &[NW, NE]), vec![2]));
        matches.push(BorderMatch::new(border_matcher(&[S, SE, E, NE, N], &[SW, NW]), vec![3]));

        // Sides
        matches.push(BorderMatch::new(border_matcher(&[W, NW, N], &[SW, NE]), vec![8]));
        matches.push(BorderMatch::new(border_matcher(&[S, SE, E], &[SW, NE]), vec![9]));
        matches.push(BorderMatch::new(border_matcher(&[W, SW, S], &[NW, SE]), vec![10]));
        matches.push(BorderMatch::new(border_matcher(&[N, NE, E], &[NW, SE]), vec![11]));

        // Sides
        matches.push(BorderMatch::new(border_matcher(&[NW, W], &[SW, NE]), vec![8]));
        matches.push(BorderMatch::new(border_matcher(&[NW, N], &[SW, NE]), vec![8]));
        matches.push(BorderMatch::new(border_matcher(&[SE, S], &[SW, NE]), vec![9]));
        matches.push(BorderMatch::new(border_matcher(&[SE, E], &[SW, NE]), vec![9]));
        matches.push(BorderMatch::new(border_matcher(&[SW, W], &[NW, SE]), vec![10]));
        matches.push(BorderMatch::new(border_matcher(&[SW, S], &[NW, SE]), vec![10]));
        matches.push(BorderMatch::new(border_matcher(&[NE, N], &[NW, SE]), vec![11]));
        matches.push(BorderMatch::new(border_matcher(&[NE, E], &[NW, SE]), vec![11]));

        // Corners
        matches.push(BorderMatch::new(border_matcher(&[W], &[SW, NW]), vec![4]));
        matches.push(BorderMatch::new(border_matcher(&[N], &[NE, NW]), vec![5]));
        matches.push(BorderMatch::new(border_matcher(&[S], &[SW, SE]), vec![6]));
        matches.push(BorderMatch::new(border_matcher(&[E], &[NE, SE]), vec![7]));

        // Sides
        matches.push(BorderMatch::new(border_matcher(&[NW], &[W, SW, NE, N]), vec![8]));
        matches.push(BorderMatch::new(border_matcher(&[SE], &[S, SW, NE, E]), vec![9]));
        matches.push(BorderMatch::new(border_matcher(&[SW], &[W, NW, S, SE]), vec![10]));
        matches.push(BorderMatch::new(border_matcher(&[NE], &[N, NW, SE, E]), vec![11]));

        // Corners
        matches.push(BorderMatch::new(border_matcher(&[W, SW, NW], &[S, N]), vec![0]));
        matches.push(BorderMatch::new(border_matcher(&[N, NW, NE], &[W, E]), vec![1]));
        matches.push(BorderMatch::new(border_matcher(&[S, SW, SE], &[W, E]), vec![2]));
        matches.push(BorderMatch::new(border_matcher(&[E, NE, SE], &[S, N]), vec![3]));

        // Corners
        matches.push(BorderMatch::new(border_matcher(&[SW, NW], &[W, S, N]), vec![0]));
        matches.push(BorderMatch::new(border_matcher(&[NW, NE], &[N, W, E]), vec![1]));
        matches.push(BorderMatch::new(border_matcher(&[SW, SE], &[S, W, E]), vec![2]));
        matches.push(BorderMatch::new(border_matcher(&[NE, SE], &[E, S, N]), vec![3]));

        matches
    };

    static ref BORDER_STYLE_1: Vec<BorderMatch> = {
        use super::dir::*;
        let mut matches = Vec::new();

        // Order must remain most specific -> least specific
        matches.push(BorderMatch::new(border_matcher(&[W, NW, N, S, SE, E], &[SW, NE]),
            vec![0, 3]));
        matches.push(BorderMatch::new(border_matcher(&[N, NE, E, W, SW, S], &[NW, SE]),
            vec![1, 2]));

        matches.push(BorderMatch::new(border_matcher(&[NW, N, SE, E], &[SW, NE]), vec![0, 3]));
        matches.push(BorderMatch::new(border_matcher(&[W, NW, S, SE], &[SW, NE]), vec![0, 3]));
        matches.push(BorderMatch::new(border_matcher(&[NE, E, SW, S], &[NW, SE]), vec![1, 2]));
        matches.push(BorderMatch::new(border_matcher(&[N, NE, W, SW], &[NW, SE]), vec![1, 2]));

        matches.push(BorderMatch::new(border_matcher(&[W, NW, N], &[SW, NE]), vec![0]));
        matches.push(BorderMatch::new(border_matcher(&[N, NE, E], &[NW, SE]), vec![1]));
        matches.push(BorderMatch::new(border_matcher(&[W, SW, S], &[NW, SE]), vec![2]));
        matches.push(BorderMatch::new(border_matcher(&[S, SE, E], &[SW, NE]), vec![3]));

        matches.push(BorderMatch::new(border_matcher(&[NW, SW, SE], &[NE]), vec![0, 2, 3]));
        matches.push(BorderMatch::new(border_matcher(&[SW, SE, NE], &[NW]), vec![2, 3, 1]));
        matches.push(BorderMatch::new(border_matcher(&[SE, NE, NW], &[SW]), vec![3, 1, 0]));
        matches.push(BorderMatch::new(border_matcher(&[NE, NW, SW], &[SE]), vec![1, 0, 2]));

        matches.push(BorderMatch::new(border_matcher(&[NW, W], &[SW, NE]), vec![0]));
        matches.push(BorderMatch::new(border_matcher(&[NW, N], &[SW, NE]), vec![0]));
        matches.push(BorderMatch::new(border_matcher(&[NE, N], &[NW, SE]), vec![1]));
        matches.push(BorderMatch::new(border_matcher(&[NE, E], &[NW, SE]), vec![1]));
        matches.push(BorderMatch::new(border_matcher(&[SW, W], &[NW, SE]), vec![2]));
        matches.push(BorderMatch::new(border_matcher(&[SW, S], &[NW, SE]), vec![2]));
        matches.push(BorderMatch::new(border_matcher(&[SE, S], &[SW, NE]), vec![3]));
        matches.push(BorderMatch::new(border_matcher(&[SE, E], &[SW, NE]), vec![3]));

        matches.push(BorderMatch::new(border_matcher(&[NW], &[W, SW, NE, N]), vec![0]));
        matches.push(BorderMatch::new(border_matcher(&[NE], &[N, NW, SE, E]), vec![1]));
        matches.push(BorderMatch::new(border_matcher(&[SW], &[W, NW, S, SE]), vec![2]));
        matches.push(BorderMatch::new(border_matcher(&[SE], &[S, SW, NE, E]), vec![3]));

        matches.push(BorderMatch::new(border_matcher(&[W, SW, NW], &[]), vec![0, 2]));
        matches.push(BorderMatch::new(border_matcher(&[N, NW, NE], &[]), vec![0, 1]));
        matches.push(BorderMatch::new(border_matcher(&[S, SW, SE], &[]), vec![2, 3]));
        matches.push(BorderMatch::new(border_matcher(&[E, NE, SE], &[]), vec![1, 3]));

        matches.push(BorderMatch::new(border_matcher(&[N], &[NW, NE]), vec![]));
        matches.push(BorderMatch::new(border_matcher(&[E], &[NE, SE]), vec![]));
        matches.push(BorderMatch::new(border_matcher(&[S], &[SW, SE]), vec![]));
        matches.push(BorderMatch::new(border_matcher(&[W], &[NW, SW]), vec![]));

        matches
    };
}

#[inline(always)]
fn shift_val(val: usize) -> usize {
    if val > dir::SW { val - 1 } else { val }
}

fn border_matcher(borders: &[usize], current: &[usize]) -> u16 {
    let mut result: u16 = 0;
    for border in borders {
        result = result | (1 << shift_val(*border));
    }
    for cur in current {
        result = result | (0x100 << shift_val(*cur));
    }
    result
}
