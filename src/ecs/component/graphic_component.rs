// Chariot: An open source reimplementation of Age of Empires (1997)
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

use identifier::{GraphicId, PlayerColorId};
use specs;
use types::Fixed;

#[derive(Clone, Debug)]
pub struct GraphicComponent {
    pub player_color_id: PlayerColorId,
    pub graphic_id: Option<GraphicId>,
    pub frame: u16,
    pub frame_time: Fixed,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
}

impl specs::Component for GraphicComponent {
    type Storage = specs::VecStorage<GraphicComponent>;
}

impl GraphicComponent {
    pub fn new() -> GraphicComponent {
        GraphicComponent {
            player_color_id: 0.into(),
            graphic_id: None,
            frame: 0u16,
            frame_time: 0.into(),
            flip_horizontal: false,
            flip_vertical: false,
        }
    }

    pub fn set_graphic(&mut self, graphic_id: Option<GraphicId>) {
        self.graphic_id = graphic_id;
        self.frame = 0u16;
        self.frame_time = 0.into();
    }
}
