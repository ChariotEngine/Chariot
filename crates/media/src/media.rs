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
use key::{Key, KeyState, KeyStates, MouseButton};

use nalgebra::Vector2;
use renderer::Renderer;

use sdl2;
use std::cell::RefCell;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::rc::Rc;

pub trait Media {
    fn is_open(&self) -> bool;
    fn update(&mut self);

    fn key_states(&self) -> &KeyStates<Key>;

    fn mouse_position(&self) -> Vector2<i32>;
    fn mouse_button_states<'a>(&'a self) -> &'a KeyStates<MouseButton>;

    fn renderer<'a>(&'a mut self) -> &'a mut Renderer;
    fn viewport_size(&self) -> Vector2<u32>;
}

pub type MediaRef = Rc<RefCell<Box<Media>>>;

pub fn create_media(width: u32, height: u32, title: &str) -> Result<MediaRef> {
    SdlMedia::new(width, height, title).map(|m| Rc::new(RefCell::new(Box::new(m) as Box<Media>)))
}

struct SdlMedia {
    context: sdl2::Sdl,
    renderer: Renderer,
    open: bool,
    keys_pressed: HashSet<Key>,
    key_states: KeyStates<Key>,
    mouse_position: Vector2<i32>,
    mouse_button_states: KeyStates<MouseButton>,
    initial_width: u32,
    initial_height: u32,
    scale_x: f32,
    scale_y: f32,
}

impl SdlMedia {
    fn new(width: u32, height: u32, title: &str) -> Result<SdlMedia> {
        let mut context = try!(sdl2::init());
        let renderer = try!(Renderer::new(&mut context, width, height, title));

        Ok(SdlMedia {
            context: context,
            renderer: renderer,
            open: true,
            keys_pressed: HashSet::new(),
            key_states: KeyStates::new(HashMap::new()),
            mouse_position: Vector2::new(0, 0),
            mouse_button_states: KeyStates::new(HashMap::new()),
            initial_width: width,
            initial_height: height,
            scale_x: 1f32,
            scale_y: 1f32,
        })
    }
}

impl Media for SdlMedia {
    fn is_open(&self) -> bool {
        self.open
    }

    fn update(&mut self) {
        use sdl2::event::{Event, WindowEvent};

        let mut event_pump = match self.context.event_pump() {
            Ok(pump) => pump,
            Err(err) => {
                println!("Failed to handle window events: {}", err);
                self.open = false;
                return;
            }
        };

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    self.open = false;
                }
                Event::Window { win_event: WindowEvent::Resized(data1, data2), .. } => {
                    self.scale_x = data1 as f32 / self.initial_width as f32;
                    self.scale_y = data2 as f32 / self.initial_height as f32;
                    self.renderer.set_scale(self.scale_x, self.scale_y);
                }
                _ => {}
            }
        }

        self.keys_pressed =
            event_pump.keyboard_state().pressed_scancodes().filter_map(Key::from_sdl).collect();

        let new_key_states = update_key_states(&self.key_states, &self.keys_pressed);
        self.key_states = new_key_states;

        let mouse_state = event_pump.mouse_state();
        self.mouse_position = Vector2::new(mouse_state.x(), mouse_state.y());

        let new_mouse_states = update_key_states(&self.mouse_button_states, &determine_pressed_mouse_buttons(&mouse_state));
        self.mouse_button_states = new_mouse_states;
    }

    fn key_states(&self) -> &KeyStates<Key> {
        &self.key_states
    }

    fn mouse_position(&self) -> Vector2<i32> {
        Vector2::new((self.mouse_position.x as f32 / self.scale_x) as i32,
                     (self.mouse_position.y as f32 / self.scale_y) as i32)
    }

    fn mouse_button_states<'a>(&'a self) -> &'a KeyStates<MouseButton> {
        &self.mouse_button_states
    }

    fn renderer<'a>(&'a mut self) -> &'a mut Renderer {
        &mut self.renderer
    }

    fn viewport_size(&self) -> Vector2<u32> {
        self.renderer.viewport_size()
    }
}

fn update_key_states<K: Eq + Hash + Copy>(key_states: &KeyStates<K>,
                                          pressed_keys: &HashSet<K>)
                                          -> KeyStates<K> {
    use KeyState::*;

    let mut new_states: HashMap<K, KeyState> = HashMap::new();
    for key in pressed_keys {
        new_states.insert(*key, TransitionDown);
    }
    for (key, state) in &key_states.0 {
        let pressed = pressed_keys.contains(&key);
        new_states.insert(*key,
                          match *state {
                              TransitionDown | Down => if pressed { Down } else { TransitionUp },
                              TransitionUp | Up => if pressed { TransitionDown } else { Up },
                          });
    }
    KeyStates::new(new_states)
}

fn determine_pressed_mouse_buttons(mouse_state: &sdl2::mouse::MouseState) -> HashSet<MouseButton> {
    let mut buttons = HashSet::new();
    if mouse_state.left() {
        buttons.insert(MouseButton::Left);
    }
    if mouse_state.middle() {
        buttons.insert(MouseButton::Middle);
    }
    if mouse_state.right() {
        buttons.insert(MouseButton::Right);
    }
    buttons
}
