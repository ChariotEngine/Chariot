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

mod ecs;
mod partition;

use ecs::render_system::{TerrainRenderSystem, TileDebugRenderSystem, UnitRenderSystem};
use ecs::resource::{MouseState, PressedKeys, Viewport};

use nalgebra::Vector2;

use std::process;

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 768;

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

    let game_dir = match resource::GameDir::new(game_data_dir) {
        Ok(game_dir) => game_dir,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };

    let drs_manager = resource::DrsManager::new(&game_dir);
    if let Err(err) = drs_manager.borrow_mut().preload() {
        println!("Failed to preload DRS archives: {}", err);
        process::exit(1);
    }

    let shape_manager = match resource::ShapeManager::new(drs_manager.clone()) {
        Ok(shape_manager) => shape_manager,
        Err(err) => {
            println!("Failed to initialize the shape manager: {}", err);
            process::exit(1);
        }
    };

    println!("Loading \"data/empires.dat\"...");
    let empires_dat_location = game_dir.find_file("data/empires.dat").unwrap();
    let empires = dat::EmpiresDbRef::new(dat::EmpiresDb::read_from_file(empires_dat_location)
        .expect("data/empires.dat"));

    println!("Loading \"{}\"...", scenario_file_name);
    let test_scn = scn::Scenario::read_from_file(scenario_file_name).expect(scenario_file_name);

    let media = match media::create_media(WINDOW_WIDTH, WINDOW_HEIGHT, "OpenAOE") {
        Ok(media) => media,
        Err(err) => {
            println!("Failed to create media window: {}", err);
            process::exit(1);
        }
    };

    let mut planner = ecs::create_world_planner(media.clone(), empires.clone(), &test_scn);

    let mut terrain_render_system =
        TerrainRenderSystem::new(media.clone(), shape_manager.clone(), empires.clone());
    let unit_render_system =
        UnitRenderSystem::new(media.clone(), shape_manager.clone(), empires.clone());
    let tile_debug_render_system = TileDebugRenderSystem::new(media.clone(), shape_manager.clone());

    while media.borrow().is_open() {
        media.borrow_mut().update();
        media.borrow_mut().renderer().present();

        update_input_resources(planner.mut_world(), &**media.borrow());

        planner.dispatch(());
        planner.wait();

        update_viewport(planner.mut_world(), &mut **media.borrow_mut());

        // Need to render in the main thread, and
        // don't want to write communication between threads to do it
        terrain_render_system.render(planner.mut_world());
        unit_render_system.render(planner.mut_world());
        tile_debug_render_system.render(planner.mut_world());
    }
}

fn update_viewport(world: &mut specs::World, media: &mut media::Media) {
    let viewport = world.read_resource::<Viewport>();
    let camera_pos = Vector2::new(viewport.top_left.x as i32, viewport.top_left.y as i32);
    media.renderer().set_camera_position(&camera_pos);
}

fn update_input_resources(world: &mut specs::World, media: &media::Media) {
    let (mut keys, mut mouse_state) = {
        (world.write_resource::<PressedKeys>(), world.write_resource::<MouseState>())
    };

    (*keys).0 = media.pressed_keys().clone();
    mouse_state.position = media.mouse_position().clone();
}
