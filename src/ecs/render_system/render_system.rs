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

use super::super::world::SystemGroup;
use specs;

pub trait RenderSystem: Send {
    fn render(&mut self, arg: specs::RunArg, lerp: f32);
}

pub struct RenderSystemWrapper(Box<RenderSystem>);

impl RenderSystemWrapper {
    pub fn new(render_system: Box<RenderSystem>) -> RenderSystemWrapper {
        RenderSystemWrapper(render_system)
    }
}

impl specs::System<(SystemGroup, f32)> for RenderSystemWrapper {
    fn run(&mut self, arg: specs::RunArg, params: (SystemGroup, f32)) {
        match params.0 {
            SystemGroup::Render => self.0.render(arg, params.1),
            _ => arg.fetch(|_| {}),
        }
    }
}
