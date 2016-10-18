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

use sdl2::{self, surface};
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
    pub fn new(sdl_context: &mut sdl2::Sdl, width: u32, height: u32, title: &str) -> Result<Renderer> {
        let video = try!(sdl_context.video());
        let mut window = try!(video.window(title, width, height)
            .position_centered()
            .resizable()
            .opengl()
            .build());
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

    /// Renders `text` to the screen at `screen_coords` in the
    /// font that exists at path-to-disc/SYSTEM/FONT/ARIAL.TTF
    pub fn render_text(&mut self, text: &str, screen_coords: (i32, i32)) {
        use rusttype::{FontCollection, PositionedGlyph, Scale, point};
        let font_data = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/arial.ttf"));
        let collection = FontCollection::from_bytes(font_data as &[u8]);
        let font = collection.into_font().unwrap();
        let height: f32 = 12.4*2.0;
        let pixel_height = (height.ceil() * 2.0) as usize;
        let scale = Scale {
            x: height * 2.0,
            y: height * 2.0,
        };
        let v_metrics = font.v_metrics(scale);
        let offset = point(0.0, v_metrics.ascent);
        let glyphs: Vec<PositionedGlyph> = font.layout(text, scale, offset).collect();
        let width = glyphs.iter()
            .rev()
            .filter_map(|g| {
                g.pixel_bounding_box()
                    .map(|b| b.min.x as f32 + g.unpositioned().h_metrics().advance_width)
            })
            .next()
            .unwrap_or(0.0)
            .ceil() as usize;

        let bytes_per_pixel = 4;
        let pitch = width * bytes_per_pixel;
        let mut pixel_data = vec![0; pixel_height * pitch];

        // Now we actually render the glyphs to a bitmap...
        for g in glyphs {
            if let Some(bb) = g.pixel_bounding_box() {
                // v is the amount of the pixel covered
                // by the glyph, in the range 0.0 to 1.0
                g.draw(|x, y, v| {
                    let c = (v * 255.0) as u8;
                    let x = x as i32 + bb.min.x;
                    let y = y as i32 + bb.min.y;
                    // There's still a possibility that the glyph clips the boundaries of the bitmap
                    if x >= 0 && x < width as i32 && y >= 0 && y < pixel_height as i32 {
                        let x = x as usize * bytes_per_pixel;
                        let y = y as usize;
                        pixel_data[(x + y * pitch + 0)] = c;
                        pixel_data[(x + y * pitch + 1)] = c;
                        pixel_data[(x + y * pitch + 2)] = c;
                        pixel_data[(x + y * pitch + 3)] = c;
                    }
                });
            }
        }

        // Copy the bitmap onto a surface, and we're basically done!
        let format = sdl2::pixels::PixelFormatEnum::RGBA8888;
        let surface = match surface::Surface::from_data(&mut pixel_data,
                                                        width as u32,
                                                        pixel_height as u32,
                                                        pitch as u32,
                                                        format) {
            Ok(s) => s,
            Err(_) => panic!("Failed to create surface"),
        };

        let texture = match self.create_texture_from_surface(surface) {
            Ok(t) => t,
            Err(_) => panic!("Failed to create texture"),
        };

        self.render_texture(&texture,
                            None,
                            Rect::of(screen_coords.0,
                                     screen_coords.1,
                                     width as i32,
                                     height as i32),
                            false,
                            false);
    }
}

impl SdlRenderer for Renderer {
    fn create_texture_from_surface(&mut self, surface: sdl2::surface::Surface) -> Result<Texture> {
        let (width, height) = (surface.width(), surface.height());
        let sdl_texture = try!(self.renderer.create_texture_from_surface(surface));
        Ok(texture::create_texture(sdl_texture, width, height))
    }
}
