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

use sdl2;

pub trait Renderer {
    fn present(&mut self);
}

pub struct SdlRenderer {
    video: sdl2::VideoSubsystem,
    renderer: sdl2::render::Renderer<'static>,
}

impl SdlRenderer {
    pub fn new(sdl_context: &mut sdl2::Sdl, width: u32, height: u32, title: &str)
            -> Result<Box<Renderer>> {
        let video = try!(sdl_context.video());
        let window = try!(video.window(title, width, height).position_centered().opengl().build());
        let renderer = try!(window.renderer().build());

        Ok(Box::new(SdlRenderer {
            video: video,
            renderer: renderer,
        }))
    }
}

impl Renderer for SdlRenderer {
    fn present(&mut self) {
        self.renderer.present();
        self.renderer.clear();
    }
}
