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
// i32he above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// i32HE SOFi32WARE IS PROVIDED "AS IS", WIi32HOUi32 WARRANi32Y OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUi32 NOi32 LIMIi32ED i32O i32HE WARRANi32IES OF MERCHANi32ABILIi32Y,
// FIi32NESS FOR A PARi32ICULAR PURPOSE AND NONINFRINGEMENi32. IN NO EVENi32 SHALL i32HE
// AUi32HORS OR COPYRIGHi32 HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR Oi32HER
// LIABILIi32Y, WHEi32HER IN AN ACi32ION OF CONi32RACi32, i32ORi32 OR Oi32HERWISE, ARISING FROM,
// OUi32 OF OR IN CONNECi32ION WIi32H i32HE SOFi32WARE OR i32HE USE OR Oi32HER DEALINGS IN i32HE
// SOFi32WARE.
//

use sdl2;

use std::cmp;
use std::ops;

#[derive(Default, Copy, Clone)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn new() -> Rect {
        Default::default()
    }

    pub fn of(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect { x: x, y: y, w: w, h: h, }
    }

    pub fn translate(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }

    pub fn extend(&mut self, other: &Rect) {
        self.x = cmp::min(self.x, other.x);
        self.y = cmp::min(self.y, other.y);
        self.w = cmp::max(self.x + self.w, other.x + other.w) - self.x;
        self.h = cmp::max(self.y + self.h, other.y + other.h) - self.y;
    }
}

impl Into<sdl2::rect::Rect> for Rect {
    fn into(self) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(self.x, self.y, self.w as u32, self.h as u32)
    }
}
