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

use sdl2;

pub struct Texture {
    pub width: u32,
    pub height: u32,
    texture: sdl2::render::Texture,
}

// TODO: Haven't quite figured out how to make a new method on Texture that is only exposed
// to other members of the crate (but not outside of the crate)
pub fn create_texture(sdl_texture: sdl2::render::Texture, width: u32, height: u32) -> Texture {
    Texture {
        width: width,
        height: height,
        texture: sdl_texture,
    }
}

// Separate so that it's not exported with the crate
pub trait SdlTexture {
    fn sdl_texture<'a>(&'a self) -> &'a sdl2::render::Texture;
}

impl SdlTexture for Texture {
    fn sdl_texture<'a>(&'a self) -> &'a sdl2::render::Texture {
        &self.texture
    }
}
