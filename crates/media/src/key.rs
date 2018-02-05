// Chariot: An open source reimplementation of Age of Empires (1997)
// Copyright (c) 2016 Kevin Fuller
// Copyright (c) 2018 Taryn Hill
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

use sdl2;

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Space,
    ShiftLeft,
    CtrlLeft,
    // Add keys as necessary
}

impl Key {
    pub fn from_sdl(scancode: sdl2::keyboard::Scancode) -> Option<Key> {
        sdl2::keyboard::Keycode::from_scancode(scancode).and_then(|keycode| {
            use sdl2::keyboard::Keycode as K;
            Some(match keycode {
                K::Up => Key::Up,
                K::Down => Key::Down,
                K::Left => Key::Left,
                K::Right => Key::Right,
                K::Space => Key::Space,
                K::LShift => Key::ShiftLeft,
                K::LCtrl => Key::CtrlLeft,
                _ => return None,
            })
        })
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum KeyState {
    TransitionDown,
    Down,
    TransitionUp,
    Up,
}

impl KeyState {
    pub fn is_down(&self) -> bool {
        match *self {
            KeyState::TransitionDown | KeyState::Down => true,
            _ => false,
        }
    }

    pub fn is_up(&self) -> bool {
        match *self {
            KeyState::TransitionUp | KeyState::Up => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct KeyStates<K: Eq + Hash>(pub HashMap<K, KeyState>);

impl<K: Eq + Hash> KeyStates<K> {
    pub fn new(states: HashMap<K, KeyState>) -> KeyStates<K> {
        KeyStates(states)
    }

    pub fn key_state(&self, key: K) -> KeyState {
        match self.0.get(&key) {
            Some(key_state) => *key_state,
            None => KeyState::Up,
        }
    }

    pub fn is_down(&self, key: K) -> bool {
        match self.0.get(&key) {
            Some(key_state) => key_state.is_down(),
            None => false,
        }
    }

    pub fn is_up(&self, key: K) -> bool {
        match self.0.get(&key) {
            Some(key_state) => key_state.is_up(),
            None => true,
        }
    }
}
