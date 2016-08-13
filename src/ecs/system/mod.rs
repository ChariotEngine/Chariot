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

mod action;
mod system;
mod animation_system;
mod camera_input_system;
mod camera_position_system;
mod velocity_system;
mod grid_system;
mod unit_action_system;
mod unit_selection_system;

pub use self::action::*;
pub use self::animation_system::AnimationSystem;
pub use self::camera_input_system::CameraInputSystem;
pub use self::camera_position_system::CameraPositionSystem;
pub use self::grid_system::GridSystem;
pub use self::system::{System, SystemWrapper};
pub use self::unit_action_system::UnitActionSystem;
pub use self::unit_selection_system::UnitSelectionSystem;
pub use self::velocity_system::VelocitySystem;
