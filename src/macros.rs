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

// The messaging is a little more user-friendly than panic
macro_rules! unrecoverable {
    ( $fmt:expr, $($args:expr),* ) => {
        use std::process;
        println!($fmt, $($args),*);
        process::exit(1);
    }
}

macro_rules! fetch_components {
    (
        $arg:expr,
        $entities:ident,
        [
            $( components($name:ident: $typ:path), )*
            $( mut components($mut_name:ident: $mut_typ:path), )*
            $( resource($res_name:ident: $res_typ:path), )*
            $( mut resource($mut_res_name:ident: $mut_res_typ:path), )*
        ]
    ) => {
        let (
            $entities,
            $( $name, )*
            $( $res_name, )*
            $( mut $mut_name, )*
            $( mut $mut_res_name, )*
        ) = $arg.fetch(|w| {
            (
                w.entities(),
                $( w.read::<$typ>(), )*
                $( w.read_resource::<$res_typ>(), )*
                $( w.write::<$mut_typ>(), )*
                $( w.write_resource::<$mut_res_typ>(), )*
            )
        });
    };
}
