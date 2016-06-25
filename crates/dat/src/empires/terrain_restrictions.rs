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

use empires::EmpiresDb;
use error::*;

use io_tools::*;

use std::io::prelude::*;

impl EmpiresDb {
    pub fn read_terrain_restrictions<R: Read + Seek>(&mut self, cursor: &mut R)
            -> EmpiresDbResult<()> {
        self.terrain_restriction_count = try!(cursor.read_u16());
        self.terrain_count = try!(cursor.read_u16());

        let mut terrain_restriction_pointers = Vec::new();
        for _ in 0..self.terrain_restriction_count {
            terrain_restriction_pointers.push(cursor.read_u32());
        }

        // Don't know what any of the terrain restriction data is for yet, so read/skip for now
        for _ in 0..self.terrain_restriction_count {
            for _ in 0..self.terrain_count {
                try!(cursor.read_f32()); // passable/buildable/dmg multiplier?
            }
        }

        Ok(())
    }
}
