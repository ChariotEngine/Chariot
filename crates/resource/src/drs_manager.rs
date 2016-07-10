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

use error::*;
use game_dir::GameDir;

use drs::DrsFile;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum DrsKey {
    Border,
    Graphics,
    Interfac,
    Sounds,
    Terrain,
}

impl DrsKey {
    pub fn path(&self) -> &'static str {
        use self::DrsKey::*;
        match *self {
            Border => "data/border.drs",
            Graphics => "data/graphics.drs",
            Interfac => "data/interfac.drs",
            Sounds => "data/sounds.drs",
            Terrain => "data/terrain.drs",
        }
    }
}

pub struct DrsManager {
    game_dir: GameDir,
    resources: HashMap<DrsKey, DrsFile>,
}

pub type DrsManagerRef = Rc<RefCell<DrsManager>>;

impl DrsManager {
    pub fn new(game_dir: &GameDir) -> DrsManagerRef {
        Rc::new(RefCell::new(DrsManager {
            game_dir: game_dir.clone(),
            resources: HashMap::new(),
        }))
    }

    pub fn get<'a>(&'a self, drs_key: DrsKey) -> &'a DrsFile {
        self.resources.get(&drs_key).unwrap()
    }

    pub fn preload(&mut self) -> Result<()> {
        try!(self.preload_drs(DrsKey::Border));
        try!(self.preload_drs(DrsKey::Graphics));
        try!(self.preload_drs(DrsKey::Interfac));
        try!(self.preload_drs(DrsKey::Sounds));
        try!(self.preload_drs(DrsKey::Terrain));
        Ok(())
    }

    fn preload_drs(&mut self, drs_key: DrsKey) -> Result<()> {
        let file_name = try!(self.game_dir.find_file(drs_key.path()));
        println!("Loading {:?}...", file_name);
        let drs = try!(DrsFile::read_from_file(file_name));
        self.resources.insert(drs_key, drs);
        Ok(())
    }
}
