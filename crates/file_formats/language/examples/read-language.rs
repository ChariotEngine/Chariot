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
extern crate open_aoe_language as language;

use clap::{Arg, App};

fn main() {
    let matches = App::new("read-language")
        .version("1.0")
        .author("Kevin Fuller <angered.ghandi@gmail.com>")
        .about("Reads strings out of language.dll files from Age of Empires (1997)")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input dll to use")
            .required(true)
            .index(1))
        .get_matches();

    let file_name = matches.value_of("INPUT").unwrap();
    match language::Language::read_from_file(file_name) {
        Ok(lang) => {
            println!("{:#?}", lang.strings);
        },
        Err(err) => {
            println!("Failed to read the language file: {}", err);
        }
    }
}
