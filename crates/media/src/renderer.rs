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

use error::Result;

use nalgebra::Vector2;

use sdl2;
use texture::{self, SdlTexture, Texture};
use types::{Color, Rect};

// Separate so that it's not exported with the crate
pub trait SdlRenderer {
    fn create_texture_from_surface(&mut self, surface: sdl2::surface::Surface) -> Result<Texture>;
}

pub struct Renderer {
    camera_pos: Vector2<i32>,
    _video: sdl2::VideoSubsystem,
    renderer: sdl2::render::Renderer<'static>,
}

impl Renderer {
    pub fn new(sdl_context: &mut sdl2::Sdl, width: u32, height: u32, title: &str, fullscreen: bool) -> Result<Renderer> {
        let video = try!(sdl_context.video());

        let mut window = {
            let mut builder = video.window(title, width, height);

            builder
                .position_centered()
                .opengl();

            if fullscreen {
                builder
                    .fullscreen_desktop()
                    .borderless();

                #[cfg(target_os="macos")]
                { builder.position(0, 0); }

                sdl2::hint::set("SDL_VIDEO_MINIMIZE_ON_FOCUS_LOSS", "0");
            } else {
                builder.resizable();
            }

            builder.build()?
        };

        window.set_minimum_size(width, height).expect("set window min size");

        let renderer = try!(window.renderer().present_vsync().build());
        println!("Renderer initialized with {:#?}", renderer.info());

        Ok(Renderer {
            camera_pos: Vector2::new(0, 0),
            _video: video,
            renderer: renderer,
        })
    }

    pub fn present(&mut self) {
        self.set_render_color(Color::rgba(0, 0, 0, 0));
        self.renderer.present();
        self.renderer.clear();
    }

    pub fn viewport_size(&self) -> Vector2<u32> {
        let size = self.renderer.window().unwrap().size();
        Vector2::new(size.0, size.1)
    }

    pub fn set_scale(&mut self, scale_x: f32, scale_y: f32) {
        self.renderer.set_scale(scale_x, scale_y).expect("set render scale");
    }

    pub fn set_camera_position(&mut self, position: &Vector2<i32>) {
        self.camera_pos = *position;
    }

    pub fn render_texture(&mut self,
                          texture: &Texture,
                          src_rect: Option<Rect>,
                          mut dst_rect: Rect,
                          flip_horizontal: bool,
                          flip_vertical: bool) {
        dst_rect.x -= self.camera_pos.x;
        dst_rect.y -= self.camera_pos.y;
        self.renderer
            .copy_ex(texture.sdl_texture(),
                     src_rect.map(|r| r.into()),
                     Some(dst_rect.into()),
                     0.0,
                     None,
                     flip_horizontal,
                     flip_vertical)
            .unwrap_or_else(|err| {
                println!("Failed to render texture: {}", err);
            });
    }

    pub fn set_render_color(&mut self, color: Color) {
        self.renderer.set_draw_color(color.into());
    }

    pub fn render_rect(&mut self, mut rect: Rect) {
        rect.x -= self.camera_pos.x;
        rect.y -= self.camera_pos.y;
        self.renderer.draw_rect(rect.into()).expect("Failed to draw rect");
    }

    pub fn render_line(&mut self, mut first: Vector2<i32>, mut second: Vector2<i32>) {
        first.x -= self.camera_pos.x;
        first.y -= self.camera_pos.y;
        second.x -= self.camera_pos.x;
        second.y -= self.camera_pos.y;
        self.renderer
            .draw_line(sdl2::rect::Point::new(first.x, first.y),
                       sdl2::rect::Point::new(second.x, second.y))
            .expect("Failed to draw line");
    }
}

impl SdlRenderer for Renderer {
    fn create_texture_from_surface(&mut self, surface: sdl2::surface::Surface) -> Result<Texture> {
        let (width, height) = (surface.width(), surface.height());
        let sdl_texture = try!(self.renderer.create_texture_from_surface(surface));
        Ok(texture::create_texture(sdl_texture, width, height))
    }
}
