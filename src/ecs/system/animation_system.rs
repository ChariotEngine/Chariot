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

use specs::{self, Join};

pub struct AnimationSystem {
    empires: dat::EmpiresDbRef,
}

impl AnimationSystem {
    pub fn new(empires: dat::EmpiresDbRef) -> AnimationSystem {
        AnimationSystem { empires: empires }
    }

    fn update_unit(&self,
                   unit: &mut UnitComponent,
                   rotation: f32,
                   graphic: &dat::Graphic,
                   time_step: f32) {
        unit.frame_time += time_step;

        let start_frame = start_frame(rotation,
                                      graphic.frame_count,
                                      graphic.angle_count,
                                      graphic.mirror_mode);
        let current_frame = start_frame +
                            frame_at_time(unit.frame_time,
                                          graphic.frame_rate,
                                          graphic.frame_count,
                                          graphic.replay_delay);
        if current_frame < unit.frame {
            unit.frame_time = 0.0;
        }
        unit.frame = current_frame;
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

fn start_frame(rotation: f32, frame_count: u16, angle_count: u16, mirror_mode: u8) -> u16 {
    // TODO: Implement angles and mirror modes (#27)
    0
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
    use super::frame_at_time;

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
