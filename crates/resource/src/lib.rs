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

#![recursion_limit = "1024"] // for the error_chain crate

#[macro_use]
extern crate error_chain;

extern crate nalgebra;

extern crate chariot_drs as drs;
extern crate chariot_slp as slp;
extern crate chariot_palette as palette;
extern crate chariot_media as media;
extern crate chariot_identifier as identifier;

#[macro_use]
extern crate chariot_types as types;

mod error;
mod game_dir;
mod drs_manager;
mod shape_manager;
mod shape_metadata;
mod render_command;

pub use drs_manager::{DrsKey, DrsManager, DrsManagerRef};
pub use game_dir::GameDir;
pub use render_command::*;
pub use shape_manager::{Shape, ShapeKey, ShapeManager, ShapeManagerRef};
pub use shape_metadata::{ShapeMetadata, ShapeMetadataKey, ShapeMetadataStore, ShapeMetadataStoreRef};
