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
use renderer::{Renderer, SdlRenderer};

use sdl2;

pub trait Media {
    fn is_open(&self) -> bool;
    fn update(&mut self);

    fn renderer<'a>(&'a mut self) -> &'a mut Renderer;
}

pub fn create_media(width: u32, height: u32, title: &str) -> Result<Box<Media>> {
    SdlMedia::new(width, height, title).map(|m| Box::new(m) as Box<Media>)
}

struct SdlMedia {
    context: sdl2::Sdl,
    renderer: Box<Renderer>,
    open: bool,
}

impl SdlMedia {
    fn new(width: u32, height: u32, title: &str) -> Result<SdlMedia> {
        let mut context = try!(sdl2::init());
        let renderer = try!(SdlRenderer::new(&mut context, width, height, title));

        Ok(SdlMedia {
            context: context,
            renderer: renderer,
            open: true,
        })
    }
}

impl Media for SdlMedia {
    fn is_open(&self) -> bool {
        self.open
    }

    fn update(&mut self) {
        use sdl2::event::Event;

        let mut event_pump = match self.context.event_pump() {
            Ok(pump) => pump,
            Err(err) => {
                println!("Failed to handle window events: {}", err);
                self.open = false;
                return
            }
        };

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    self.open = false;
                },
                _ => { }
            }
        }
    }

    fn renderer<'a>(&'a mut self) -> &'a mut Renderer {
        &mut *self.renderer
    }
}
