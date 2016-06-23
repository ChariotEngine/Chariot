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

extern crate clap;
extern crate open_aoe_slp as slp;

use clap::{Arg, App};

fn main() {
    let matches = App::new("read-slp")
        .version("1.0")
        .author("Kevin Fuller <angered.ghandi@gmail.com>")
        .about("Reads SLP shape/graphics files from Age of Empires (1997)")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();

    let file_name = matches.value_of("INPUT").unwrap();
    match slp::SlpFile::read_from_file(file_name) {
        Ok(slp_file) => {
            println!("Shape count: {}", slp_file.header.shape_count);
            for shape in &slp_file.shapes {
                println!("{:?}", shape.header);

                let width = shape.header.width;
                let height = shape.header.height;
                for y in 0..height {
                    for x in 0..width {
                        let val = (shape.pixels[(y * width + x) as usize] as f32 / 25.6f32) as usize;
                        if val != 0 {
                            print!("{}", val);
                        } else {
                            print!(" ");
                        }
                    }
                    println!("");
                }
            }
        },
        Err(err) => {
            println!("Failed to read the SLP file: {}", err);
        }
    }
}
