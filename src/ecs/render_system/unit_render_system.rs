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

use ecs::{TransformComponent, UnitComponent, VisibleUnitComponent};
use ecs::resource::ViewProjector;

use dat;
use media::MediaRef;
use resource::{DrsKey, ShapeKey, ShapeManagerRef};
use identifier::PlayerColorId;

use specs::{self, Join};

pub struct UnitRenderSystem<'a> {
    media: MediaRef,
    shape_manager: ShapeManagerRef,
    empires_db: &'a dat::EmpiresDb,
}

impl<'a> UnitRenderSystem<'a> {
    pub fn new(media: MediaRef,
               shape_manager: ShapeManagerRef,
               empires_db: &'a dat::EmpiresDb)
               -> UnitRenderSystem<'a> {
        UnitRenderSystem {
            media: media,
            shape_manager: shape_manager,
            empires_db: empires_db,
        }
    }

    pub fn render(&self, world: &mut specs::World) {
        let (transforms, units, visible_units, projector) = (world.read::<TransformComponent>(),
                                                             world.read::<UnitComponent>(),
                                                             world.read::<VisibleUnitComponent>(),
                                                             world.read_resource::<ViewProjector>());
        let mut media = self.media.borrow_mut();
        let renderer = media.renderer();

        for (transform, unit, _visible_units) in (&transforms, &units, &visible_units).iter() {
            let position = projector.project(&transform.position);
            let graphic = &self.empires_db.graphics[&unit.graphic_id];
            let shape_key = ShapeKey::new(DrsKey::Graphics,
                                          graphic.slp_id,
                                          PlayerColorId(unit.player_id.as_isize()));

            // TODO: Render the unit's rotation as well
            self.shape_manager
                .borrow_mut()
                .get(&shape_key, renderer)
                .expect("failed to get shape for unit rendering")
                .render_frame(renderer, unit.frame as usize, &position);
        }
    }
}
