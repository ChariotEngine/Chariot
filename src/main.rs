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

extern crate open_aoe_drs as drs;
extern crate open_aoe_slp as slp;
extern crate open_aoe_palette as palette;
extern crate open_aoe_dat as dat;
extern crate open_aoe_language as language;
extern crate open_aoe_scn as scn;
extern crate open_aoe_media as media;

use media::Rect;
use palette::PaletteColor;

use std::io;
use std::process;
use std::path;

fn main() {
    let terrain_drs = drs::DrsFile::read_from_file("data/terrain.drs").expect("terrain.drs");
    println!("Loaded terrain.drs");

    let interfac_drs = drs::DrsFile::read_from_file("data/interfac.drs").expect("interfac.drs");
    println!("Loaded interfac.drs");

    let empires = dat::EmpiresDb::read_from_file("data/empires.dat").expect("empires.dat");
    println!("Loaded empires.dat");

    let test_scn = scn::Scenario::read_from_file("data/test.scn").expect("test.scn");
    println!("Loaded test.scn");

    let mut media = match media::create_media(800, 600, "OpenAOE") {
        Ok(media) => media,
        Err(err) => {
            println!("Failed to create media window: {}", err);
            process::exit(1);
        }
    };

    // Temp testing code
    let bin_table = &interfac_drs.tables[0];
    let palette_contents = &bin_table.contents[26];
    let palette = palette::read_from(&mut io::Cursor::new(palette_contents),
            path::PathBuf::from("50500.bin").as_path()).expect("palette");
    let slp_contents = terrain_drs.find_table(drs::DrsFileType::Slp).expect("slp table")
        .find_file_contents(15001).expect("15001");
    let slp = slp::SlpFile::read_from(&mut io::Cursor::new(slp_contents)).expect("slp");
    let shape = &slp.shapes[0];

    let texture = media::TextureBuilder::new(shape.header.width, shape.header.height).expect("texture builder")
        .with_palette(palette.iter().map(|c: &PaletteColor| -> u32 {
            (*c).into()
        }).collect())
        .blit_shape(&shape.pixels,
            Rect::of(0, 0, shape.header.width as i32, shape.header.height as i32),
            Rect::of(0, 0, shape.header.width as i32, shape.header.height as i32))
        .build(media.renderer()).expect("texture");

    while media.is_open() {
        media.update();

        media.renderer().present();

        media.renderer().render_texture(&texture, None,
            Some(Rect::of(0, 0, texture.width as i32, texture.height as i32)));
    }
}
