//
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
//

use dat::{self, TerrainId};
use scn;

use std::cmp;

// Using module+constants instead of an enum because Rust is terrible at C-style enums
mod dir {
    pub const W: usize = 0;
    pub const NW: usize = 1;
    pub const N: usize = 2;
    pub const SW: usize = 3;
    pub const NE: usize = 5;
    pub const S: usize = 6;
    pub const SE: usize = 7;
    pub const E: usize = 8;

    pub const ALL: [usize; 8] = [W, NW, N, SW, NE, S, SE, E];
}

mod border {

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

        pub fn find_match(border_style: i16, border_matcher: u16) -> Option<&'static BorderMatch> {
            let border_style = match border_style {
                0 => &*BORDER_STYLE_0,
                1 => &*BORDER_STYLE_1,
                _ => panic!("Unknown border style {}", border_style),
            };

            for border_match in border_style {
                if border_match.matcher & border_matcher == border_match.matcher {
                    return Some(border_match)
                }
            }
            None
        }
    }

    lazy_static! {
        static ref BORDER_STYLE_0: Vec<BorderMatch> = {
            use super::dir::*;
            let mut matches = Vec::new();

            // Order must remain most specific -> least specific
            matches.push(BorderMatch::new(border_matcher(&[S, SW, W, NW, N], &[SE, NE]), vec![0]));
            matches.push(BorderMatch::new(border_matcher(&[W, NW, N, NE, E], &[SW, SE]), vec![1]));
            matches.push(BorderMatch::new(border_matcher(&[W, SW, S, SE, E], &[NW, NE]), vec![2]));
            matches.push(BorderMatch::new(border_matcher(&[S, SE, E, NE, N], &[SW, NW]), vec![3]));

            matches.push(BorderMatch::new(border_matcher(&[W, NW, N], &[SW, NE]), vec![8]));
            matches.push(BorderMatch::new(border_matcher(&[S, SE, E], &[SW, NE]), vec![9]));
            matches.push(BorderMatch::new(border_matcher(&[W, SW, S], &[NW, SE]), vec![10]));
            matches.push(BorderMatch::new(border_matcher(&[N, NE, E], &[NW, SE]), vec![11]));

            matches.push(BorderMatch::new(border_matcher(&[NW, W], &[SW, NE]), vec![8]));
            matches.push(BorderMatch::new(border_matcher(&[NW, N], &[SW, NE]), vec![8]));
            matches.push(BorderMatch::new(border_matcher(&[SE, S], &[SW, NE]), vec![9]));
            matches.push(BorderMatch::new(border_matcher(&[SE, E], &[SW, NE]), vec![9]));
            matches.push(BorderMatch::new(border_matcher(&[SW, W], &[NW, SE]), vec![10]));
            matches.push(BorderMatch::new(border_matcher(&[SW, S], &[NW, SE]), vec![10]));
            matches.push(BorderMatch::new(border_matcher(&[NE, N], &[NW, SE]), vec![11]));
            matches.push(BorderMatch::new(border_matcher(&[NE, E], &[NW, SE]), vec![11]));

            matches.push(BorderMatch::new(border_matcher(&[W], &[SW, NW]), vec![4]));
            matches.push(BorderMatch::new(border_matcher(&[N], &[NE, NW]), vec![5]));
            matches.push(BorderMatch::new(border_matcher(&[S], &[SW, SE]), vec![6]));
            matches.push(BorderMatch::new(border_matcher(&[E], &[NE, SE]), vec![7]));

            matches.push(BorderMatch::new(border_matcher(&[W, SW, NW], &[S, N]), vec![0]));
            matches.push(BorderMatch::new(border_matcher(&[N, NW, NE], &[W, E]), vec![1]));
            matches.push(BorderMatch::new(border_matcher(&[S, SW, SE], &[W, E]), vec![2]));
            matches.push(BorderMatch::new(border_matcher(&[E, NE, SE], &[S, N]), vec![3]));

            matches
        };

        static ref BORDER_STYLE_1: Vec<BorderMatch> = {
            use super::dir::*;
            let mut matches = Vec::new();

            // Order must remain most specific -> least specific
            matches.push(BorderMatch::new(border_matcher(&[W, NW, N, S, SE, E], &[SW, NE]), vec![0, 3]));
            matches.push(BorderMatch::new(border_matcher(&[N, NE, E, W, SW, S], &[NW, SE]), vec![1, 2]));

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
        if val > super::dir::SW { val - 1 } else { val }
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

    pub fn border_matcher_from_matrix(matrix: &[u8], border_val: u8, current_val: u8)
            -> u16 {
        let mut result: u16 = 0;
        for dir in &super::dir::ALL {
            if matrix[*dir] == border_val {
                result = result | (1 << shift_val(*dir));
            } else if matrix[*dir] == current_val {
                result = result | (0x100 << shift_val(*dir));
            }
        }
        result
    }
}

/// Cursor for simplifying the loading of terrain from SCN files.
pub struct TerrainCursor<'a> {
    terrain_block: &'a dat::TerrainBlock,
    tiles: &'a [scn::MapTile],
    width: isize,
    height: isize,
    index: isize,
}

impl<'a> TerrainCursor<'a> {
    pub fn new(terrain_block: &'a dat::TerrainBlock, tiles: &'a [scn::MapTile],
            width: isize, height: isize) -> TerrainCursor<'a> {
        TerrainCursor {
            terrain_block: terrain_block,
            tiles: tiles,
            width: width,
            height: height,
            index: 0,
        }
    }

    /// Returns the current tile under examination.
    pub fn current(&self) -> &'a scn::MapTile {
        &self.tiles[self.index as usize]
    }

    /// Returns the current index under examination.
    pub fn index(&self) -> isize {
        self.index
    }

    /// Returns the current row under examination.
    pub fn row(&self) -> isize {
        self.index / self.width
    }

    /// Returns the current column under examination.
    pub fn col(&self) -> isize {
        self.index % self.width
    }

    /// Returns a tile at the requested row/col while clamping at the edges of the terrain
    /// so that the request cannot go out of bounds. This means that the tiles at the edges
    /// will repeat indefinitely if you query outside of the terrain.
    fn at(&self, row: isize, col: isize) -> &'a scn::MapTile {
        // Clamp the row/col
        let row = cmp::max(0, cmp::min(row, self.height - 1));
        let col = cmp::max(0, cmp::min(col, self.width - 1));
        let index = row * self.width + col;
        &self.tiles[index as usize]
    }

    /// Returns a tile in the requested direction relative to the current position
    /// Has the same clamping as at().
    fn at_relative(&self, direction: usize) -> &'a scn::MapTile {
        let row = self.row() + (direction as isize / 3) - 1;
        let col = self.col() + (direction as isize % 3) - 1;
        self.at(row, col)
    }

    // Returns Option<(border_id, border_index)>
    pub fn border(&self) -> Option<(u16, &'static [u16])> {
        let cur_terrain_id = self.current().terrain_id;
        let cur_terrain = &self.terrain_block.terrains[&TerrainId(cur_terrain_id as isize)];

        let mut matrix = [0; 9];
        for direction in &dir::ALL {
            matrix[*direction] = self.at_relative(*direction).terrain_id;
        }

        // Figure out the terrain ID of the bordering terrain
        let mut visited = [0xFFu8; 9];
        let mut bordering_terrain_id = 0;
        let mut border_id = 0;
        let mut border = None;
        for i in 0..9 {
            if cur_terrain_id != matrix[i] && !visited.contains(&matrix[i]) {
                visited[i] = matrix[i];
                bordering_terrain_id = matrix[i];
                border_id = cur_terrain.terrain_borders[&TerrainId(matrix[i] as isize)].as_isize();
                border = Some(&self.terrain_block.terrain_borders[border_id as usize]);
                if border.unwrap().enabled {
                    break;
                }
            }
        }

        if border.is_some() && border.unwrap().enabled {
            let matcher = border::border_matcher_from_matrix(&matrix,
                bordering_terrain_id, cur_terrain_id);
            let border_style = border.unwrap().border_style;

            match border::BorderMatch::find_match(border_style, matcher) {
                Some(border_match) => {
                    // TODO: Use all indices
                    Some((border_id as u16, &border_match.border_indices))
                }
                None => {
                    println!(concat!("Terrain border failed: ",
                        "bs: {}, r: {}, c: {}, ct: {}, bt: {},",
                        "\n  {:?}\n  {:?}\n  {:?}\nmatcher: {:016b}\n"),
                        border_style, self.row(), self.col(), cur_terrain_id, bordering_terrain_id,
                        &matrix[0..3], &matrix[3..6], &matrix[6..9], matcher);
                    None
                }
            }
        } else {
            None
        }
    }

    /// Returns true if there's another tile available to examine.
    pub fn has_next(&self) -> bool {
        return self.index < self.width * self.height
    }

    /// Advances to the next tile.
    pub fn next(&mut self) {
        self.index += 1;
    }
}
