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


use dat;
use ecs::TransformComponent;

use nalgebra::Vector3;
use types::AABox;

pub fn selection_box(unit_info: &dat::Unit, transform: &TransformComponent) -> AABox {
    let position = transform.position();
    AABox::new(Vector3::new(position.x - unit_info.selection_shape_size_x.into(),
                            position.y - unit_info.selection_shape_size_y.into(),
                            position.z + unit_info.selection_shape_size_z.into()),
               Vector3::new(position.x + unit_info.selection_shape_size_x.into(),
                            position.y + unit_info.selection_shape_size_y.into(),
                            position.z))
}

pub fn collision_box(unit_info: &dat::Unit, transform: &TransformComponent) -> AABox {
    let position = transform.position();
    AABox::new(Vector3::new(position.x - unit_info.collision_size_x.into(),
                            position.y - unit_info.collision_size_y.into(),
                            position.z + unit_info.collision_size_z.into()),
               Vector3::new(position.x + unit_info.collision_size_x.into(),
                            position.y + unit_info.collision_size_y.into(),
                            position.z))
}
