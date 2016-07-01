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

use error::Result;
use texture::{self, Texture, SdlTexture};
use types::Rect;

use sdl2;

// Separate so that it's not exported with the crate
pub trait SdlRenderer {
    fn create_texture_from_surface(&mut self, surface: sdl2::surface::Surface) -> Result<Texture>;
}

pub struct Renderer {
    video: sdl2::VideoSubsystem,
    renderer: sdl2::render::Renderer<'static>,
}

impl Renderer {
    pub fn new(sdl_context: &mut sdl2::Sdl, width: u32, height: u32, title: &str)
            -> Result<Renderer> {
        let video = try!(sdl_context.video());
        let window = try!(video.window(title, width, height).position_centered().opengl().build());
        let renderer = try!(window.renderer().build());

        Ok(Renderer {
            video: video,
            renderer: renderer,
        })
    }

    pub fn present(&mut self) {
        self.renderer.present();
        self.renderer.clear();
    }

    pub fn render_texture(&mut self, texture: &Texture,
            src_rect: Option<Rect>, dst_rect: Option<Rect>) {
        self.renderer.copy(texture.sdl_texture(), src_rect.map(|r| r.into()),
            dst_rect.map(|r| r.into()));
    }
}

impl SdlRenderer for Renderer {
    fn create_texture_from_surface(&mut self, surface: sdl2::surface::Surface) -> Result<Texture> {
        let (width, height) = (surface.width(), surface.height());
        // TODO: Change from unwrap to try! once PR is merged: https://github.com/AngryLawyer/rust-sdl2/pull/519
        let sdl_texture = self.renderer.create_texture_from_surface(surface).unwrap();
        Ok(texture::create_texture(sdl_texture, width, height))
    }
}
