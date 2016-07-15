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

mod terrain;
mod ecs;

use terrain::TerrainBlender;
use terrain::TerrainRenderer;
use types::{Point, Rect};
use ecs::resource::{CameraPosition, PressedKeys};
use ecs::render_system::UnitRenderSystem;

use std::process;

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
    let empires = dat::EmpiresDb::read_from_file(game_dir.find_file("data/empires.dat").unwrap())
        .expect("data/empires.dat");

    println!("Loading \"{}\"...", scenario_file_name);
    let test_scn = scn::Scenario::read_from_file(scenario_file_name).expect(scenario_file_name);

    let media = match media::create_media(1024, 768, "OpenAOE") {
        Ok(media) => media,
        Err(err) => {
            println!("Failed to create media window: {}", err);
            process::exit(1);
        }
    };

    let tile_half_width = empires.terrain_block.tile_half_width as i32;
    let tile_half_height = empires.terrain_block.tile_half_height as i32;

    let mut terrain_renderer = TerrainRenderer::new(&empires.terrain_block);
    let terrain_blender = TerrainBlender::new(&empires.terrain_block,
                                              &test_scn.map.tiles,
                                              test_scn.map.width as isize,
                                              test_scn.map.height as isize);

    let mut world_planner = ecs::create_world_planner();

    // Setup the camera
    world_planner.mut_world().add_resource(CameraPosition::new(0., 0.));
    world_planner.mut_world().create_now()
        // Temporary hardcoded camera offset
        .with(ecs::TransformComponent::new(126f32 * tile_half_width as f32,
                                           -145f32 * tile_half_height as f32, 0., 0.))
        .with(ecs::VelocityComponent::new())
        .with(ecs::CameraComponent)
        .build();

    // Create entities for each unit in the SCN
    for (player_id, units) in &test_scn.player_units {
        let civ_id = test_scn.player_data.player_civs[player_id.as_usize()].civilization_id;
        for unit in units {
            let transform_component = ecs::TransformComponent::new(unit.position_x,
                                                                   unit.position_y,
                                                                   unit.position_z,
                                                                   unit.rotation);
            let unit_component = ecs::UnitComponentBuilder::new(&empires)
                .with_player_id(*player_id)
                .with_unit_id(unit.unit_id)
                .with_civilization_id(civ_id)
                .build();

            world_planner.mut_world()
                .create_now()
                .with(transform_component)
                .with(ecs::VelocityComponent::new())
                .with(unit_component)
                .build();
        }
    }

    world_planner.add_system(ecs::system::VelocitySystem::new(), "VelocitySystem", 100);
    world_planner.add_system(ecs::system::CameraInputSystem::new(),
                             "CameraInputSystem",
                             1000);
    world_planner.add_system(ecs::system::CameraPositionSystem::new(),
                             "CameraPositionSystem",
                             1001);

    let unit_render_system = UnitRenderSystem::new(media.clone(), shape_manager.clone(), &empires);

    while media.borrow().is_open() {
        media.borrow_mut().update();

        media.borrow_mut().renderer().present();

        world_planner.mut_world()
            .add_resource(PressedKeys(media.borrow().pressed_keys().clone()));
        world_planner.dispatch(());
        world_planner.wait();

        {
            let camera_pos = world_planner.mut_world().read_resource::<CameraPosition>();
            media.borrow_mut()
                .renderer()
                .set_camera_position(&Point::new(camera_pos.x as i32, camera_pos.y as i32));
        }

        // TODO: Render only the visible portion of the map
        let map_rect = Rect::of(0,
                                0,
                                terrain_blender.width() as i32,
                                terrain_blender.height() as i32);
        terrain_renderer.render(media.borrow_mut().renderer(),
                                &mut *shape_manager.borrow_mut(),
                                &terrain_blender,
                                map_rect);

        // Need to render in the main thread
        // Not sure how to do this with a specs system yet
        unit_render_system.render(world_planner.mut_world());
    }
}
