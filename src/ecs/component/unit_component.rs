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
use identifier::*;
use specs;

#[derive(Clone, Debug)]
pub struct UnitComponent {
    pub player_id: PlayerId,
    pub civilization_id: CivilizationId,
    pub unit_id: UnitId,
}

impl specs::Component for UnitComponent {
    type Storage = specs::VecStorage<UnitComponent>;
}

impl UnitComponent {
    pub fn new(player_id: PlayerId, civilization_id: CivilizationId, unit_id: UnitId) -> UnitComponent {
        UnitComponent {
            player_id: player_id,
            civilization_id: civilization_id,
            unit_id: unit_id,
        }
    }

    /// Convenience function to get the unit's information from the empires db
    pub fn db<'a>(&self, empires: &'a dat::EmpiresDbRef) -> &'a dat::Unit {
        empires.unit(self.civilization_id, self.unit_id)
    }
}
