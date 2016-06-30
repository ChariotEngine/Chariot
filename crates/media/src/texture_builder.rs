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

use error::*;
use rect::Rect;
use renderer::{Renderer, SdlRenderer};
use texture::Texture;

use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;

use std::io::{self, Write};
use std::mem;

pub struct TextureBuilder {
    surface: Surface<'static>,
    palette: Vec<u32>,
    error: Option<Error>,
}

fn to_rgba(palette: &[u32], src_pixels: &[u8], width: usize, height: usize) -> Result<Vec<u8>> {
    let mut dst_pixels = io::Cursor::new(vec![0u8; width * height * 4]);

    for y in 0..height {
        for x in 0..width {
            let src_index = y * width + x;
            let color_index = src_pixels[src_index] as usize;
            if color_index > 0 {
                let color = palette[color_index];
                let color_bytes = unsafe { mem::transmute::<u32, [u8; 4]>(color) };
                try!(dst_pixels.write(&color_bytes));
            } else {
                try!(dst_pixels.write(&[0, 0, 0, 0u8]));
            }
        }
    }

    Ok(dst_pixels.into_inner())
}

impl TextureBuilder {
    pub fn new(width: u32, height: u32) -> Result<TextureBuilder> {
        Ok(TextureBuilder {
            surface: try!(Surface::new(width, height, PixelFormatEnum::RGBA8888)),
            palette: Vec::new(),
            error: None,
        })
    }

    pub fn with_palette(mut self, palette: Vec<u32>) -> Self {
        self.palette = palette;
        self
    }

    pub fn blit_shape(mut self, pixel_buffer: &[u8], src_rect: Rect, dst_rect: Rect) -> Self {
        let mut pixels = match to_rgba(&self.palette, pixel_buffer, src_rect.w as usize, src_rect.h as usize) {
            Ok(pixels) => pixels,
            Err(err) => {
                self.error = Some(err);
                return self
            }
        };

        let surf_result = Surface::from_data(&mut pixels, src_rect.w as u32,
            src_rect.h as u32, 4 * (src_rect.w as u32), PixelFormatEnum::RGBA8888);
        if surf_result.is_err() {
            self.error = Some(surf_result.err().unwrap().into());
            return self
        }

        let src_surface = surf_result.ok().unwrap();
        let blit_result = src_surface.blit(Some(src_rect.into()),
            &mut self.surface, Some(dst_rect.into()));
        if blit_result.is_err() {
            self.error = Some(blit_result.err().unwrap().into());
        }

        self
    }

    pub fn build(self, renderer: &mut Renderer) -> Result<Texture> {
        if self.error.is_some() {
            return Err(self.error.unwrap())
        }

        renderer.create_texture_from_surface(self.surface)
    }
}
