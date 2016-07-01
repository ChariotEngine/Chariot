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

use drs_manager::{DrsKey, DrsManagerRef};
use error::*;

use drs::DrsFileType;
use slp::SlpFile;
use palette::{self, PaletteColor};
use media::{Renderer, Texture, TextureBuilder};
use types::{Point, Rect};

use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
use std::rc::Rc;

const SHAPE_PADDING: i32 = 4;
const PALETTE_FILE_ID: u32 = 50500;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct ShapeKey {
    pub drs_key: DrsKey,
    pub slp_id: u32,
    pub player_color: u8,
}

impl ShapeKey {
    pub fn new(drs_key: DrsKey, slp_id: u32, player_color: u8) -> ShapeKey {
        ShapeKey {
            drs_key: drs_key,
            slp_id: slp_id,
            player_color: player_color,
        }
    }
}

pub struct Shape {
    texture: Texture,
    frames: Vec<Rect>,
    centers: Vec<Point>,
}

impl Shape {
    fn load_from(slp: &SlpFile, palette: &[u32], renderer: &mut Renderer) -> Result<Shape> {
        let mut total_rect = Rect::new();
        let mut dst_rects = Vec::new();
        let mut centers = Vec::new();
        let mut next_x = 0i32;
        for shape in &slp.shapes {
            let dst_rect = Rect::of(next_x, 0, shape.header.width as i32, shape.header.height as i32);
            dst_rects.push(dst_rect);
            centers.push(Point::new(shape.header.center_x, shape.header.center_y));

            total_rect.extend(&dst_rect);
            next_x += shape.header.width as i32 + SHAPE_PADDING;
        }

        let mut texture_builder = try!(TextureBuilder::new(total_rect.w as u32,
            total_rect.h as u32, &palette));
        for (index, shape) in slp.shapes.iter().enumerate() {
            texture_builder = texture_builder.blit_shape(&shape.pixels,
                Rect::of(0, 0, shape.header.width as i32, shape.header.height as i32),
                dst_rects[index]);
        }

        Ok(Shape {
            texture: try!(texture_builder.build(renderer)),
            frames: dst_rects,
            centers: centers,
        })
    }

    pub fn render_frame(&self, renderer: &mut Renderer, frame: usize, position: &Point) {
        let src_rect = self.frames[frame];
        let center = &self.centers[frame];

        let mut dst_rect = src_rect;
        dst_rect.translate(position.x, position.y);
        dst_rect.translate(-center.x, -center.y);

        renderer.render_texture(&self.texture, Some(src_rect.into()), Some(dst_rect.into()));
    }
}

enum ShapeCache {
    Cached(Shape),
    Failed,
}

pub struct ShapeManager {
    drs_manager: DrsManagerRef,
    shapes: HashMap<ShapeKey, ShapeCache>,
    palette: Vec<u32>,
}

pub type ShapeManagerRef = Rc<RefCell<ShapeManager>>;

impl ShapeManager {
    pub fn new(drs_manager: DrsManagerRef) -> Result<ShapeManagerRef> {
        let palette = {
            let borrowed_drs = drs_manager.borrow();
            let interfac = borrowed_drs.get(DrsKey::Interfac);
            let bin_table = try!(interfac.find_table(DrsFileType::Binary)
                .ok_or(ErrorKind::InterfacBinaryTableMissing));
            let palette_contents = &try!(bin_table.find_file_contents(PALETTE_FILE_ID)
                .ok_or(ErrorKind::InterfacMissingPalette));
            try!(palette::read_from(&mut io::Cursor::new(palette_contents)))
        }.iter().map(|c: &PaletteColor| -> u32 { (*c).into() }).collect();

        Ok(Rc::new(RefCell::new(ShapeManager {
            drs_manager: drs_manager,
            shapes: HashMap::new(),
            palette: palette,
        })))
    }

    pub fn get<'a>(&'a mut self, shape_key: &ShapeKey, renderer: &mut Renderer) -> Option<&'a Shape> {
        use self::ShapeCache::*;

        let cached = self.shapes.get(&shape_key).is_some();
        if !cached {
            match self.load_shape(shape_key, renderer) {
                Ok(shape) => {
                    self.shapes.insert(*shape_key, Cached(shape));
                },
                Err(err) => {
                    self.shapes.insert(*shape_key, Failed);
                    println!("Failed to load shape {:?}: {}", shape_key, err);
                    return None
                }
            };
        }

        match *self.shapes.get(&shape_key).unwrap() {
            Cached(ref shape) => return Some(shape),
            Failed => return None,
        }
    }

    fn load_shape(&self, shape_key: &ShapeKey, renderer: &mut Renderer) -> Result<Shape> {
        let borrowed_drs = self.drs_manager.borrow();
        let drs_file = borrowed_drs.get(shape_key.drs_key);

        let slp_table = try!(drs_file.find_table(DrsFileType::Slp)
            .ok_or(ErrorKind::NoSlpTableInDrs(shape_key.drs_key)));
        let slp_contents = try!(slp_table.find_file_contents(shape_key.slp_id)
            .ok_or(ErrorKind::SlpNotFound(shape_key.drs_key, shape_key.slp_id)));
        let slp = try!(SlpFile::read_from(&mut io::Cursor::new(slp_contents)));

        Shape::load_from(&slp, &self.palette, renderer)
    }
}
