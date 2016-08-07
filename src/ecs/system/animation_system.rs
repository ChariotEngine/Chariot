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

use dat;
use ecs::{TransformComponent, UnitComponent};
use resource::{DrsKey, ShapeMetadataKey, ShapeMetadataStoreRef};
use specs::{self, Join};
use std::ops::Rem;
use super::System;
use types::Fixed;

pub struct AnimationSystem {
    empires: dat::EmpiresDbRef,
    shape_metadata: ShapeMetadataStoreRef,
}

impl AnimationSystem {
    pub fn new(empires: dat::EmpiresDbRef, shape_metadata: ShapeMetadataStoreRef) -> AnimationSystem {
        AnimationSystem {
            empires: empires,
            shape_metadata: shape_metadata,
        }
    }

    fn update_unit(&self,
                   unit: &mut UnitComponent,
                   rotation: Fixed,
                   graphic: &dat::Graphic,
                   time_step: Fixed) {
        unit.frame_time += time_step;

        if let Some(slp_id) = graphic.slp_id {
            let shape_key = ShapeMetadataKey::new(DrsKey::Graphics, slp_id);
            if let Some(shape_metadata) = self.shape_metadata.get(&shape_key) {
                let (start_frame, flip_horizontal) = start_frame_and_mirroring(rotation,
                                                                               shape_metadata.shape_count,
                                                                               graphic.frame_count,
                                                                               graphic.angle_count);
                let current_frame = start_frame +
                                    frame_at_time(unit.frame_time,
                                                  graphic.frame_rate.into(),
                                                  graphic.frame_count,
                                                  graphic.replay_delay.into());
                if current_frame < unit.frame {
                    unit.frame_time = 0.into();
                }
                unit.frame = current_frame;
                unit.flip_horizontal = flip_horizontal;
            }
        }
    }
}

impl System for AnimationSystem {
    fn update(&mut self, arg: specs::RunArg, time_step: Fixed) {
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
fn wrap_angle(angle: Fixed) -> Fixed {
    let modded = angle.rem(Fixed::two_pi());
    if modded < 0.into() { Fixed::two_pi() - modded.abs() } else { modded }
}

/// Returns the start frame for the given rotation, and whether mirroring should occur
fn start_frame_and_mirroring(rotation: Fixed,
                             shape_count: u32,
                             frame_count: u16,
                             angle_count: u16)
                             -> (u16, bool) {
    let rotation = wrap_angle(rotation);
    let angles_in_slp = shape_count as u16 / frame_count;
    let mirror_count = angle_count - angles_in_slp;

    let mut angle_index = u16::from((rotation * angle_count.into() / Fixed::two_pi())) % angle_count;
    let mut mirror = false;
    if angle_index < mirror_count {
        angle_index = angle_count - angle_index - (angle_count / 4);
        mirror = true;
    }

    (((angle_index - mirror_count) * frame_count) % (shape_count as u16), mirror)
}

fn frame_at_time(time: Fixed, frame_rate: Fixed, frame_count: u16, replay_delay: Fixed) -> u16 {
    if frame_rate > 0.into() {
        // Did a screen recording of the original game and analyzed it frame by frame to
        // determine how exactly the frame rate works. It looks like it's double seconds per frame.
        let seconds_per_frame = frame_rate / 2.into();
        let frame_num = u16::from(time / seconds_per_frame);

        let delay_frames = u16::from(replay_delay / seconds_per_frame);
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
    use super::{frame_at_time, start_frame_and_mirroring, wrap_angle};
    use types::Fixed;

    #[test]
    fn test_start_frame_and_mirroring() {
        let rad = |deg: u32| Fixed::from(deg) * Fixed::pi() / Fixed::from(180);

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
        assert_eq!(Fixed::from(0), wrap_angle(0.into()));
        assert_eq!(Fixed::from(1), wrap_angle(1.into()));
        assert_eq!(Fixed::two_pi() - 1.into(), wrap_angle((-1).into()));
        assert_eq!(Fixed::from(8) - Fixed::two_pi(), wrap_angle(8.into()));
    }

    #[test]
    fn test_frame_at_time() {
        // Test playing through an animation with 4 frames
        // and 2 delay frames at 0.5 seconds per frame (1.0 in empires.dat frame rate terms)
        assert_eq!(0u16, frame_at_time(0.0.into(), 1.into(), 4, 1.into()));
        assert_eq!(1u16, frame_at_time(0.6.into(), 1.into(), 4, 1.into()));
        assert_eq!(3u16, frame_at_time(1.6.into(), 1.into(), 4, 1.into()));
        assert_eq!(3u16, frame_at_time(2.0.into(), 1.into(), 4, 1.into()));
        assert_eq!(0u16, frame_at_time(3.0.into(), 1.into(), 4, 1.into()));
    }
}
