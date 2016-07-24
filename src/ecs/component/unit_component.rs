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
    pub civilization_id: CivilizationId,
    pub unit_id: UnitId,
    pub graphic_id: Option<GraphicId>,
    pub frame: u16,
    pub frame_time: f32,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
}

impl specs::Component for UnitComponent {
    type Storage = specs::VecStorage<UnitComponent>;
}

impl UnitComponent {
    pub fn new(player_id: PlayerId,
               civilization_id: CivilizationId,
               unit_id: UnitId,
               graphic_id: Option<GraphicId>)
               -> UnitComponent {
        UnitComponent {
            player_id: player_id,
            civilization_id: civilization_id,
            unit_id: unit_id,
            graphic_id: graphic_id,
            frame: 0u16,
            frame_time: 0f32,
            flip_horizontal: false,
            flip_vertical: false,
        }
    }
}

pub struct UnitComponentBuilder<'a> {
    empires: &'a dat::EmpiresDb,
    civilization_id: Option<CivilizationId>,
    player_id: Option<PlayerId>,
    unit_id: Option<UnitId>,
}

impl<'a> UnitComponentBuilder<'a> {
    pub fn new(empires: &'a dat::EmpiresDb) -> UnitComponentBuilder {
        UnitComponentBuilder {
            empires: empires,
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
            self.empires.unit(civ_id, unit_id).standing_graphic
        };
        UnitComponent::new(self.player_id.unwrap(),
                           self.civilization_id.unwrap(),
                           unit_id,
                           graphic_id)
    }
}
