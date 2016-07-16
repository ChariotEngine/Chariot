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

extern crate sdl2_ttf;

use error::Result;
use texture::{self, Texture, SdlTexture};
use types::Rect;

use nalgebra::Vector2;

use sdl2;
use sdl2_ttf::Font;
use std::path::Path;


// Separate so that it's not exported with the crate
pub trait SdlRenderer {
    fn create_texture_from_surface(&mut self, surface: sdl2::surface::Surface) -> Result<Texture>;
}

pub struct Renderer {
    camera_pos: Vector2<i32>,
    _video: sdl2::VideoSubsystem,
    renderer: sdl2::render::Renderer<'static>,
    font: sdl2_ttf::Font,
}

impl Renderer {
    pub fn new(sdl_context: &mut sdl2::Sdl,
               ttf_context: &mut sdl2_ttf::Sdl2TtfContext,
               font_path: &Path,
               width: u32,
               height: u32,
               title: &str)
            -> Result<Renderer> {
        let video = try!(sdl_context.video());
        let window = try!(video.window(title, width, height).position_centered().opengl().build());
        let renderer = try!(window.renderer().build());

        let mut font = try!(ttf_context.load_font(font_path, 16));

        Ok(Renderer {
            camera_pos: Vector2::new(0, 0),
            _video: video,
            renderer: renderer,
            font: font,
        })
    }

    pub fn present(&mut self) {
        self.renderer.present();
        self.renderer.clear();
    }

    pub fn set_camera_position(&mut self, position: &Vector2<i32>) {
        self.camera_pos = *position;
    }

    pub fn render_texture(&mut self, texture: &Texture,
            src_rect: Option<Rect>, mut dst_rect: Rect) {
        dst_rect.x -= self.camera_pos.x;
        dst_rect.y -= self.camera_pos.y;
        self.renderer.copy(texture.sdl_texture(), src_rect.map(|r| r.into()),
            Some(dst_rect.into()));
    }

    pub fn render_text(&mut self, text: &str, screen_position: &Vector2<i32>) {
        use sdl2::pixels::Color;

        let ref font = self.font;
        let surface = font.render(text)
            .blended(Color::RGBA(255, 0, 0, 255)).unwrap();

        let src_rect = Rect::of(screen_position.x, screen_position.y, 100, 100);
        let dst_rect = Rect::of(0, 0, src_rect.w, src_rect.h);

        let mut texture = self.create_texture_from_surface(surface).unwrap();
        self.render_texture(&texture, Some(src_rect.into()), dst_rect.into());
    }
}

impl SdlRenderer for Renderer {
    fn create_texture_from_surface(&mut self, surface: sdl2::surface::Surface) -> Result<Texture> {
        let (width, height) = (surface.width(), surface.height());
        let sdl_texture = try!(self.renderer.create_texture_from_surface(surface));
        Ok(texture::create_texture(sdl_texture, width, height))
    }
}
