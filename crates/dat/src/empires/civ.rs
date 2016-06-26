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

use empires::unit::{Unit, read_unit};
use error::*;

use io_tools::*;

use std::io::prelude::*;

#[derive(Default, Debug)]
pub struct Civilization {
    enabled: bool,
    name: String,
    resources: Vec<f32>,
    tech_tree_id: i16,
    icon_set: i8,
    units: Vec<Unit>,
}

pub fn read_civs<R: Read + Seek>(stream: &mut R) -> EmpiresDbResult<Vec<Civilization>> {
    let civ_count = try!(stream.read_u16()) as usize;
    stream.read_array(civ_count, |c| read_civ(c))
}

fn read_civ<R: Read + Seek>(stream: &mut R) -> EmpiresDbResult<Civilization> {
    let mut civ: Civilization = Default::default();
    civ.enabled = try!(stream.read_u8()) != 0;
    civ.name = try!(stream.read_sized_str(20));

    let resource_count = try!(stream.read_u16()) as usize;
    civ.tech_tree_id = try!(stream.read_i16());
    civ.resources = try!(stream.read_array(resource_count, |c| c.read_f32()));
    civ.icon_set = try!(stream.read_i8());

    let unit_count = try!(stream.read_u16()) as usize;
    let unit_pointers = try!(stream.read_array(unit_count, |c| c.read_i32()));
    for i in 0..unit_count {
        // Similarly with graphics, units have an array of pointers that are meaningless
        // except that if one of them is zero, that unit has to be skipped
        if unit_pointers[i] != 0 {
            civ.units.push(try!(read_unit(stream)));
        }
    }
    Ok(civ)
}
