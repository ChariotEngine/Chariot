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

use media::Renderer;
use resource::{RenderCommand, ShapeManager};

pub struct RenderCommands {
    commands: Vec<RenderCommand>,
}

impl RenderCommands {
    pub fn new() -> RenderCommands {
        RenderCommands { commands: Vec::new() }
    }

    pub fn push(&mut self, render_command: RenderCommand) {
        self.commands.push(render_command);
    }

    pub fn execute(&mut self, renderer: &mut Renderer, shape_manager: &mut ShapeManager) {
        RenderCommand::render_all(renderer, shape_manager, &mut self.commands);
    }

    pub fn clear_rendered(&mut self) {
        self.commands.retain(|c| c.order().debug);
    }

    pub fn clear_debug(&mut self) {
        self.commands.clear();
    }
}
