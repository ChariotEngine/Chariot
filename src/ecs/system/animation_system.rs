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

use ecs::{TransformComponent, UnitComponent};

use dat;
use resource::{DrsKey, ShapeMetadataKey, ShapeMetadataStoreRef};

use specs::{self, Join};

use std::f32::consts::PI;
use std::ops::Rem;

const TWO_PI: f32 = 2.0 * PI;

pub struct AnimationSystem {
    empires: dat::EmpiresDbRef,
    shape_metadata: ShapeMetadataStoreRef,
}

impl AnimationSystem {
    pub fn new(empires: dat::EmpiresDbRef,
               shape_metadata: ShapeMetadataStoreRef)
               -> AnimationSystem {
        AnimationSystem {
            empires: empires,
            shape_metadata: shape_metadata,
        }
    }

    fn update_unit(&self,
                   unit: &mut UnitComponent,
                   rotation: f32,
                   graphic: &dat::Graphic,
                   time_step: f32) {
        unit.frame_time += time_step;

        if let Some(slp_id) = graphic.slp_id {
            let shape_key = ShapeMetadataKey::new(DrsKey::Graphics, slp_id);
            if let Some(shape_metadata) = self.shape_metadata.get(&shape_key) {
                let (start_frame, flip_horizontal) =
                    start_frame_and_mirroring(rotation,
                                              shape_metadata.shape_count,
                                              graphic.frame_count,
                                              graphic.angle_count);
                let current_frame = start_frame +
                                    frame_at_time(unit.frame_time,
                                                  graphic.frame_rate,
                                                  graphic.frame_count,
                                                  graphic.replay_delay);
                if current_frame < unit.frame {
                    unit.frame_time = 0.0;
                }
                unit.frame = current_frame;
                unit.flip_horizontal = flip_horizontal;
            }
        }
    }
}

impl specs::System<f32> for AnimationSystem {
    fn run(&mut self, arg: specs::RunArg, time_step: f32) {
        let (transforms, mut units) =
            arg.fetch(|w| (w.read::<TransformComponent>(), w.write::<UnitComponent>()));

        for (transform, unit) in (&transforms, &mut units).iter() {
            if let Some(graphic_id) = unit.graphic_id {
                let graphic = self.empires.graphic(graphic_id);
                if graphic.frame_count > 1 {
                    self.update_unit(unit, transform.rotation, graphic, time_step);
                }
            }
        }
    }
}

/// Constrains angle to be between [0, 2*PI] radians
fn wrap_angle(angle: f32) -> f32 {
    let modded = angle.rem(TWO_PI);
    if modded < 0.0 { TWO_PI - modded.abs() } else { modded }
}

/// Returns the start frame for the given rotation, and whether mirroring should occur
fn start_frame_and_mirroring(rotation: f32,
                             shape_count: u32,
                             frame_count: u16,
                             angle_count: u16)
                             -> (u16, bool) {
    let rotation = wrap_angle(rotation);
    let angles_in_slp = shape_count as u16 / frame_count;
    let mirror_count = angle_count - angles_in_slp;

    let mut angle_index = ((angle_count as f32 * rotation / TWO_PI) as u16) % angle_count;
    let mut mirror = false;
    if angle_index < mirror_count {
        angle_index = angle_count - angle_index - (angle_count / 4);
        mirror = true;
    }

    (((angle_index - mirror_count) * frame_count) % (shape_count as u16), mirror)
}

fn frame_at_time(time: f32, frame_rate: f32, frame_count: u16, replay_delay: f32) -> u16 {
    if frame_rate > 0f32 {
        // Did a screen recording of the original game and analyzed it frame by frame to
        // determine how exactly the frame rate works. It looks like it's double seconds per frame.
        let seconds_per_frame = frame_rate / 2.0;
        let frame_num = (time / seconds_per_frame) as u16;

        let delay_frames = (replay_delay / seconds_per_frame) as u16;
        if frame_num >= frame_count {
            if frame_num >= frame_count + delay_frames { 0 } else { frame_count - 1 }
        } else {
            frame_num
        }
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;
    use super::{TWO_PI, frame_at_time, start_frame_and_mirroring, wrap_angle};

    #[test]
    fn test_start_frame_and_mirroring() {
        let rad = |deg: u32| deg as f32 * PI / 180.0;

        assert_eq!((0u16, true), start_frame_and_mirroring(rad(1), 16, 16, 2));
        assert_eq!((0u16, false),
                   start_frame_and_mirroring(rad(181), 16, 16, 2));

        assert_eq!((3u16 * 6, true),
                   start_frame_and_mirroring(rad(1), 30, 6, 8));
        assert_eq!((2u16 * 6, true),
                   start_frame_and_mirroring(rad(46), 30, 6, 8));
        assert_eq!((1u16 * 6, true),
                   start_frame_and_mirroring(rad(91), 30, 6, 8));
        assert_eq!((0u16 * 6, false),
                   start_frame_and_mirroring(rad(136), 30, 6, 8));
        assert_eq!((1u16 * 6, false),
                   start_frame_and_mirroring(rad(181), 30, 6, 8));
        assert_eq!((2u16 * 6, false),
                   start_frame_and_mirroring(rad(226), 30, 6, 8));
        assert_eq!((3u16 * 6, false),
                   start_frame_and_mirroring(rad(271), 30, 6, 8));
        assert_eq!((4u16 * 6, false),
                   start_frame_and_mirroring(rad(316), 30, 6, 8));
    }

    #[test]
    fn test_wrap_angle() {
        assert_eq!(0.0, wrap_angle(0.0));
        assert_eq!(1.0, wrap_angle(1.0));
        assert_eq!(TWO_PI - 1.0, wrap_angle(-1.0));
        assert_eq!(8.0 - TWO_PI, wrap_angle(8.0));
    }

    #[test]
    fn test_frame_at_time() {
        // Test playing through an animation with 4 frames
        // and 2 delay frames at 0.5 seconds per frame (1.0 in empires.dat frame rate terms)
        assert_eq!(0u16, frame_at_time(0.0, 1.0, 4, 1.0));
        assert_eq!(1u16, frame_at_time(0.6, 1.0, 4, 1.0));
        assert_eq!(3u16, frame_at_time(1.6, 1.0, 4, 1.0));
        assert_eq!(3u16, frame_at_time(2.0, 1.0, 4, 1.0));
        assert_eq!(0u16, frame_at_time(3.0, 1.0, 4, 1.0));
    }
}
