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

use identifier::{CivilizationId, PlayerId, PlayerColorId};
use scn::Scenario;
use std::collections::HashMap;

pub struct Player {
    pub name: String,
    pub player_id: PlayerId,
    pub player_color_id: PlayerColorId,
    pub civ_id: CivilizationId,
}

impl Player {
    pub fn new(name: String,
               player_id: PlayerId,
               player_color_id: PlayerColorId,
               civ_id: CivilizationId)
               -> Player {
        Player {
            name: name,
            player_id: player_id,
            player_color_id: player_color_id,
            civ_id: civ_id,
        }
    }
}

pub struct Players {
    local_player_id: PlayerId,
    players: HashMap<PlayerId, Player>,
}

impl Players {
    pub fn new() -> Players {
        Players {
            local_player_id: 0.into(),
            players: HashMap::new(),
        }
    }

    pub fn from_scenario(scenario: &Scenario, local_player_id: PlayerId) -> Players {
        let mut players = Players::new();
        for player_id in scenario.player_ids() {
            let name = scenario.player_data.player_names[*player_id as usize].clone();
            let civ_id = scenario.player_data.player_civs[*player_id as usize].civilization_id;
            let color_id = player_id.into();
            let local = player_id == local_player_id;
            players.add_player(Player::new(name, player_id, color_id, civ_id), local);
        }
        players
    }

    pub fn add_player(&mut self, player: Player, local: bool) {
        if local {
            self.local_player_id = player.player_id;
        }
        self.players.insert(player.player_id, player);
    }

    pub fn local_player<'a>(&'a self) -> &'a Player {
        let local_player_id = self.local_player_id;
        &self.players[&local_player_id]
    }
}
