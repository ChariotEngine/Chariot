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

use media::Renderer;
use nalgebra::Vector2;
use std::cmp::{Ordering, PartialOrd};
use super::{ShapeKey, ShapeManager};
use types::{Color, Rect};

#[derive(Copy, Clone, Debug)]
pub enum RenderCommand {
    RenderShape(RenderOrder, RenderShapeParams),
    RenderRect(RenderOrder, RenderRectParams),
    RenderLine(RenderOrder, RenderLineParams),
}

impl RenderCommand {
    pub fn render_all(renderer: &mut Renderer,
                      shape_manager: &mut ShapeManager,
                      commands: &mut Vec<RenderCommand>) {
        use RenderCommand::*;
        commands.sort_by(|a, b| a.order().cmp(b.order()));
        for command in commands {
            match *command {
                RenderShape(_, params) => {
                    shape_manager.get(&params.shape_key, renderer)
                        .unwrap()
                        .render_frame(renderer,
                                      params.frame_num as usize,
                                      &params.position,
                                      params.flip_horizontal,
                                      params.flip_vertical);
                }
                RenderRect(_, params) => {
                    renderer.render_rect(params.rect);
                }
                RenderLine(_, params) => {
                    renderer.set_render_color(params.color);
                    renderer.render_line(params.points[0], params.points[1]);
                }
            }
        }
    }

    pub fn new_shape(layer: u16,
                     depth: i32,
                     shape_key: ShapeKey,
                     frame_num: u16,
                     position: Vector2<i32>,
                     flip_horizontal: bool,
                     flip_vertical: bool)
                     -> RenderCommand {
        let order = RenderOrder::new(layer, depth, false);
        let params = RenderShapeParams::new(shape_key,
                                            frame_num,
                                            position,
                                            flip_horizontal,
                                            flip_vertical);
        RenderCommand::RenderShape(order, params)
    }

    pub fn new_line(layer: u16,
                    depth: i32,
                    color: Color,
                    point1: Vector2<i32>,
                    point2: Vector2<i32>)
                    -> RenderCommand {
        let order = RenderOrder::new(layer, depth, false);
        let params = RenderLineParams::new(color, point1, point2);
        RenderCommand::RenderLine(order, params)
    }

    pub fn new_debug_rect(layer: u16, depth: i32, rect: Rect) -> RenderCommand {
        let order = RenderOrder::new(layer, depth, true);
        let params = RenderRectParams::new(rect);
        RenderCommand::RenderRect(order, params)
    }

    pub fn new_debug_line(layer: u16,
                          depth: i32,
                          color: Color,
                          point1: Vector2<i32>,
                          point2: Vector2<i32>)
                          -> RenderCommand {
        let order = RenderOrder::new(layer, depth, true);
        let params = RenderLineParams::new(color, point1, point2);
        RenderCommand::RenderLine(order, params)
    }

    pub fn order(&self) -> &RenderOrder {
        use RenderCommand::*;
        match *self {
            RenderShape(ref order, _) => order,
            RenderRect(ref order, _) => order,
            RenderLine(ref order, _) => order,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RenderOrder {
    pub layer: u16,
    pub depth: i32,
    pub debug: bool,
}

impl RenderOrder {
    pub fn new(layer: u16, depth: i32, debug: bool) -> RenderOrder {
        RenderOrder {
            layer: layer,
            depth: depth,
            debug: debug,
        }
    }
}

impl PartialOrd for RenderOrder {
    fn partial_cmp(&self, other: &RenderOrder) -> Option<Ordering> {
        match self.layer.cmp(&other.layer) {
            Ordering::Equal => self.depth.partial_cmp(&other.depth),
            v @ _ => Some(v),
        }
    }
}

impl Ord for RenderOrder {
    fn cmp(&self, other: &RenderOrder) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RenderShapeParams {
    pub shape_key: ShapeKey,
    pub frame_num: u16,
    pub position: Vector2<i32>,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
}

impl RenderShapeParams {
    pub fn new(shape_key: ShapeKey,
               frame_num: u16,
               position: Vector2<i32>,
               flip_horizontal: bool,
               flip_vertical: bool)
               -> RenderShapeParams {
        RenderShapeParams {
            shape_key: shape_key,
            frame_num: frame_num,
            position: position,
            flip_horizontal: flip_horizontal,
            flip_vertical: flip_vertical,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RenderRectParams {
    pub rect: Rect,
}

impl RenderRectParams {
    pub fn new(rect: Rect) -> RenderRectParams {
        RenderRectParams { rect: rect }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RenderLineParams {
    pub color: Color,
    pub points: [Vector2<i32>; 2],
}

impl RenderLineParams {
    pub fn new(color: Color, point1: Vector2<i32>, point2: Vector2<i32>) -> RenderLineParams {
        RenderLineParams {
            color: color,
            points: [point1, point2],
        }
    }
}
