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

extern crate open_aoe_drs as drs;
extern crate open_aoe_slp as slp;
extern crate open_aoe_palette as palette;
extern crate open_aoe_dat as dat;
extern crate open_aoe_language as language;
extern crate open_aoe_scn as scn;
extern crate open_aoe_media as media;
extern crate open_aoe_resource as resource;
extern crate open_aoe_types as types;
extern crate open_aoe_identifier as identifier;

#[macro_use]
extern crate lazy_static;

extern crate clap;
extern crate specs;
extern crate nalgebra;
extern crate time;

#[macro_use]
mod macros;

mod ecs;
mod game;
mod partition;

use game::{Game, GameState, ScenarioGameState};

fn main() {
    let arg_matches = clap::App::new("OpenAOE")
        .about("An open source reimplementation of Age of Empires (1997)")
        .arg(clap::Arg::with_name("game_data_dir")
            .short("d")
            .long("game-data-dir")
            .value_name("GAME_DATA_DIR")
            .help("Sets the directory to look in for game data. Defaults to \"game\".")
            .takes_value(true))
        .arg(clap::Arg::with_name("SCENARIO")
            .required(true)
            .help("Scenario file to load (temporary while there's no menu)"))
        .get_matches();

    let game_data_dir = arg_matches.value_of("game_data_dir").unwrap_or("game");
    let scenario_file_name = arg_matches.value_of("SCENARIO").unwrap();

    let scenario = scn::Scenario::read_from_file(scenario_file_name).unwrap_or_else(|err| {
        unrecoverable!("Failed to load scenario \"{}\": {}",
                       scenario_file_name,
                       err);
    });

    let mut game = Game::new(game_data_dir);
    let initial_state = Box::new(ScenarioGameState::new(&game, scenario));
    game.push_state(initial_state as Box<GameState>);

    game.game_loop();
}
