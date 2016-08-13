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


use drs;
use drs_manager::DrsKey;
use media;
use palette;
use slp;

use std::path::PathBuf;

error_chain! {
    types {
        Error, ErrorKind, ChainErr, Result;
    }

    links {
        drs::Error, drs::ErrorKind, Drs;
        media::Error, media::ErrorKind, Media;
        palette::Error, palette::ErrorKind, Palette;
        slp::Error, slp::ErrorKind, Slp;
    }

    foreign_links {
    }

    errors {
        InterfacBinaryTableMissing {
            description("interfac.drs is missing its binary table")
            display("interfac.drs is missing its binary table")
        }
        InterfacMissingPalette {
            description("interfac.drs is missing the 50500 palette file")
            display("interfac.drs is missing the 50500 palette file")
        }
        NoSlpTableInDrs(drs_key: DrsKey) {
            description("no SLPs in DRS")
            display("no SLPs found in \"{}\"", drs_key.path())
        }
        SlpNotFound(drs_key: DrsKey, slp_id: u32) {
            description("SLP not found")
            display("{}.slp not found in \"{}\"", slp_id, drs_key.path())
        }
        GameDirInvalid(message: String) {
            description("Game directory is invalid")
            display("{}", message)
        }
        GameDataFileNotFound(file_name: PathBuf) {
            description("Game data file not found")
            display("Game data file not found: {:?}", file_name)
        }
    }
}
