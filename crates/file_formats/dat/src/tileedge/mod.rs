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

use std::fs::File;
use std::io::prelude::*;
use std::io::{Cursor, SeekFrom};
use std::borrow::BorrowMut;
use std::path::Path;
use error::*;

use io_tools::*;

#[derive(Default, Debug)]
pub struct OutlineEntry {
    pub index: u8,
    pub left_offset: u8,
    pub right_offset: u8,
}

#[derive(Default, Debug)]
pub struct TileEdgeEntry {
    pub outlines: Vec<OutlineEntry>,
}

#[derive(Default, Debug)]
pub struct TileEdgeDb {
    pub entries: Vec<TileEdgeEntry>,
}

impl TileEdgeDb {
    fn new() -> TileEdgeDb {
        Default::default()
    }

    pub fn read_from_file<P: AsRef<Path>>(file_name: P) -> Result<TileEdgeDb> {
        let mut file = try!(File::open(file_name.as_ref()));
        file.seek(SeekFrom::Start(444));

        let mut bytes = Vec::new();
        try!(file.read_to_end(&mut bytes));

        let mut db = TileEdgeDb::new();

        let mut entry: TileEdgeEntry = TileEdgeEntry { outlines: Vec::new() };
        for i in (0..(&bytes.len() / 3)).map(|x| x * 3) {
            let (idx, left, right) = (bytes[i], bytes[i + 1], bytes[i + 2]);
            if idx == 255 {
                db.entries.push(entry);
                entry = TileEdgeEntry { outlines: Vec::new() };
                continue;
            }

            entry.outlines.push(OutlineEntry {
                index: idx,
                left_offset: left,
                right_offset: right,
            });
        }
        return Ok(db);
    }
}
