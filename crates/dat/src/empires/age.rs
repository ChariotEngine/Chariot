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

use std::io;
use std::io::prelude::*;

#[derive(Default, Debug)]
pub struct AgeEffect {
    type_id: i8,
    effect_a: i16,
    effect_b: i16,
    effect_c: i16,
    effect_d: f32,
}

impl AgeEffect {
    pub fn new() -> AgeEffect {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct Age {
    name: String,
    effects: Vec<AgeEffect>,
}

impl Age {
    pub fn new() -> Age {
        Default::default()
    }
}

impl EmpiresDb {
    pub fn read_ages<R: Read + Seek>(&mut self, cursor: &mut R) -> EmpiresDbResult<()> {
        let age_count = try!(cursor.read_u32()) as usize;
        self.ages = try!(cursor.read_array(age_count, |c| EmpiresDb::read_age(c)));
        Ok(())
    }

    pub fn read_age<R: Read + Seek>(cursor: &mut R) -> io::Result<Age> {
        let mut age = Age::new();
        age.name = try!(cursor.read_sized_str(31));

        let effect_count = try!(cursor.read_u16()) as usize;
        age.effects = try!(cursor.read_array(effect_count, |c| EmpiresDb::read_age_effect(c)));
        Ok(age)
    }

    fn read_age_effect<R: Read + Seek>(cursor: &mut R) -> io::Result<AgeEffect> {
        let mut effect = AgeEffect::new();
        effect.type_id = try!(cursor.read_i8());
        effect.effect_a = try!(cursor.read_i16());
        effect.effect_b = try!(cursor.read_i16());
        effect.effect_c = try!(cursor.read_i16());
        effect.effect_d = try!(cursor.read_f32());
        Ok(effect)
    }
}
