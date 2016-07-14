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
use identifier::*;

use specs;

#[derive(Clone, Debug)]
pub struct UnitComponent {
    pub player_id: PlayerId,
    pub unit_id: UnitId,
    pub graphic_id: GraphicId,
    pub frame: u32,
    pub frame_time: f32,
}

impl specs::Component for UnitComponent {
    type Storage = specs::VecStorage<UnitComponent>;
}

impl UnitComponent {
    pub fn new(player_id: PlayerId, unit_id: UnitId, graphic_id: GraphicId) -> UnitComponent {
        UnitComponent {
            player_id: player_id,
            unit_id: unit_id,
            graphic_id: graphic_id,
            frame: 0u32,
            frame_time: 0f32,
        }
    }
}

pub struct UnitComponentBuilder<'a> {
    empires_db: &'a dat::EmpiresDb,
    civilization_id: Option<CivilizationId>,
    player_id: Option<PlayerId>,
    unit_id: Option<UnitId>,
}

impl<'a> UnitComponentBuilder<'a> {
    pub fn new(empires_db: &'a dat::EmpiresDb) -> UnitComponentBuilder {
        UnitComponentBuilder {
            empires_db: empires_db,
            civilization_id: None,
            player_id: None,
            unit_id: None,
        }
    }

    pub fn with_civilization_id(mut self, civilization_id: CivilizationId) -> Self {
        self.civilization_id = Some(civilization_id);
        self
    }

    pub fn with_player_id(mut self, player_id: PlayerId) -> Self {
        self.player_id = Some(player_id);
        self
    }

    pub fn with_unit_id(mut self, unit_id: UnitId) -> Self {
        self.unit_id = Some(unit_id);
        self
    }

    pub fn build(self) -> UnitComponent {
        let unit_id = self.unit_id.unwrap();
        let graphic_id = {
            let civ_id = self.civilization_id.unwrap();
            let units = &self.empires_db.civilizations[civ_id.as_usize() - 1].units;
            if !units.contains_key(&unit_id) {
                panic!("Failed to find unit definition for {:?}", unit_id);
            }
            let unit = &units[&unit_id];
            unit.standing_graphic
        };
        UnitComponent::new(self.player_id.unwrap(), unit_id, graphic_id)
    }
}
