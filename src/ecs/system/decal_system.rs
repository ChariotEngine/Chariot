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

use ecs::DecalComponent;
use resource::{ShapeMetadataKey, ShapeMetadataStoreRef};
use specs::{self, Join};
use super::System;
use types::Fixed;

// Hardcoded framerate for now
const SECONDS_PER_FRAME: Fixed = fixed_const!(0.1);

pub struct DecalSystem {
    shape_metadata: ShapeMetadataStoreRef,
}

impl DecalSystem {
    pub fn new(shape_metadata: ShapeMetadataStoreRef) -> DecalSystem {
        DecalSystem { shape_metadata: shape_metadata }
    }
}

impl System for DecalSystem {
    fn update(&mut self, arg: specs::RunArg, time_step: Fixed) {
        fetch_components!(arg, entities, [ mut components(decals: DecalComponent), ]);

        for (entity, decal) in (&entities, &mut decals).iter() {
            let shape_key = ShapeMetadataKey::new(decal.drs_key, decal.slp_file_id);
            if let Some(shape_metadata) = self.shape_metadata.get(&shape_key) {
                decal.frame_time += time_step;
                if decal.frame_time >= SECONDS_PER_FRAME {
                    decal.frame_time -= SECONDS_PER_FRAME;
                    decal.frame += 1;
                    if decal.frame >= shape_metadata.shape_count as u16 {
                        arg.delete(entity);
                    }
                }
            }
        }
    }
}
