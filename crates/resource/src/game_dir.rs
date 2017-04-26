// Chariot: An open source reimplementation of Age of Empires (1997)
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

use error::*;

use std::ascii::AsciiExt;
use std::fs;
use std::path::{Component, Path, PathBuf};

#[derive(Clone)]
pub struct GameDir {
    dir: PathBuf,
}

impl GameDir {
    pub fn new<P: AsRef<Path>>(dir: P) -> Result<GameDir> {
        let dir = dir.as_ref();
        if !dir.exists() {
            return Err(error(dir, "Given game data directory doesn't exist"));
        }

        let dir_metadata = try!(fs::metadata(dir)
            .chain_err(|| "Failed to get metadata on the game data directory"));
        if !dir_metadata.is_dir() {
            return Err(error(dir, "Given game data directory isn't a directory"));
        }

        let game_dir = GameDir { dir: dir.to_path_buf() };
        for file_name in &["language.dll",
                           "data/border.drs",
                           "data/empires.dat",
                           "data/graphics.drs",
                           "data/interfac.drs",
                           "data/sounds.drs",
                           "data/terrain.drs",
                           "data/tileedge.dat"] {
            if let Ok(actual_file_name) = game_dir.find_file(file_name) {
                println!("Found {:?} at {:?}", file_name, actual_file_name);
            } else {
                return Err(error(dir, &format!("Failed to find {}", file_name)));
            }
        }
        Ok(game_dir)
    }

    /// Find a file in the game data directory even if the requested case doesn't match
    /// the case of the file name on the file system (for case-sensitive file systems)
    pub fn find_file<P: AsRef<Path>>(&self, file_name: P) -> Result<PathBuf> {
        let mut full_path = self.dir.clone();
        for component in file_name.as_ref().components() {
            if let Component::Normal(component_name) = component {
                let component_name = component_name.to_string_lossy();
                let mut found = false;
                for dir_entry in try!(fs::read_dir(&full_path)
                    .chain_err(|| "Failed to traverse game data directory")) {
                    let dir_entry =
                        try!(dir_entry.chain_err(|| "Failed to read directory entry in game data directory"));
                    if component_name.eq_ignore_ascii_case(&dir_entry.path()
                        .file_name()
                        .unwrap()
                        .to_string_lossy()) {
                        full_path = full_path.join(&*dir_entry.path().file_name().unwrap().to_string_lossy());
                        found = true;
                        break;
                    }
                }
                if !found {
                    return Err(ErrorKind::GameDataFileNotFound(file_name.as_ref().to_path_buf()).into());
                }
            }
        }
        Ok(full_path)
    }
}

fn error(dir: &Path, msg: &str) -> Error {
    let msg = format!("{}\nGame data directory: {:?}\n\n The game data directory can be found on \
                       the original Age of Empires CD.\n If you're on Windows, it might be in \
                       D:\\game. Or, if you're on Linux,\n it may be in /media/AOE/game (or in \
                       the game/ directory relative to where\n the game CD is mounted).\n\n This \
                       directory should have empires.exe, language.dll, and several \
                       directories\n such as avi, campaign, and data in it.\n",
                      msg,
                      dir);
    ErrorKind::GameDirInvalid(msg.into()).into()
}
