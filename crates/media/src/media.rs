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

use error::*;
use key::Key;
use renderer::Renderer;

use sdl2;

use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;

pub trait Media {
    fn is_open(&self) -> bool;
    fn update(&mut self);

    fn is_key_down(&self, key: Key) -> bool;
    fn pressed_keys(&self) -> &HashSet<Key>;

    fn renderer<'a>(&'a mut self) -> &'a mut Renderer;
}

pub type MediaRef = Rc<RefCell<Box<Media>>>;

pub fn create_media(width: u32, height: u32, title: &str) -> Result<MediaRef> {
    SdlMedia::new(width, height, title).map(|m| Rc::new(RefCell::new(Box::new(m) as Box<Media>)))
}

struct SdlMedia {
    context: sdl2::Sdl,
    renderer: Renderer,
    open: bool,
    pressed_keys: HashSet<Key>,
}

impl SdlMedia {
    fn new(width: u32, height: u32, title: &str) -> Result<SdlMedia> {
        let mut context = try!(sdl2::init());
        let renderer = try!(Renderer::new(&mut context, width, height, title));

        Ok(SdlMedia {
            context: context,
            renderer: renderer,
            open: true,
            pressed_keys: HashSet::new(),
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

        self.pressed_keys = event_pump.keyboard_state()
            .pressed_scancodes()
            .filter_map(Key::from_sdl)
            .collect();
    }

    fn is_key_down(&self, key: Key) -> bool {
        self.pressed_keys.contains(&key)
    }

    fn pressed_keys(&self) -> &HashSet<Key> {
        &self.pressed_keys
    }

    fn renderer<'a>(&'a mut self) -> &'a mut Renderer {
        &mut self.renderer
    }
}
