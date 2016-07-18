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

use super::component::*;

use ecs::resource::{MouseState, PressedKeys, Terrain, ViewProjector, Viewport};
use ecs::system::{AnimationSystem, CameraInputSystem, CameraPositionSystem, GridSystem,
                  VelocitySystem};
use partition::GridPartition;

use dat::EmpiresDbRef;
use resource::ShapeMetadataStoreRef;
use media::MediaRef;
use scn;

use nalgebra::Vector3;
use specs;

use std::collections::HashSet;

const NUM_THREADS: usize = 4;
const GRID_CELL_SIZE: i32 = 10; // in tiles

pub fn create_world_planner(media: MediaRef,
                            empires: EmpiresDbRef,
                            shape_metadata: ShapeMetadataStoreRef,
                            scenario: &scn::Scenario)
                            -> specs::Planner<f32> {
    let viewport_size = media.borrow().viewport_size();
    let (tile_half_width, tile_half_height) = empires.tile_half_sizes();

    let mut world = specs::World::new();
    world.register::<TransformComponent>();
    world.register::<CameraComponent>();
    world.register::<UnitComponent>();
    world.register::<VelocityComponent>();
    world.register::<VisibleUnitComponent>();

    // Input resources
    world.add_resource(PressedKeys(HashSet::new()));
    world.add_resource(MouseState::new());

    // Render resources
    world.add_resource(ViewProjector::new(tile_half_width, tile_half_height));
    world.add_resource(GridPartition::new(GRID_CELL_SIZE, GRID_CELL_SIZE));

    // Camera resources and entity
    world.add_resource(Viewport::new(viewport_size.x as f32, viewport_size.y as f32));
    world.create_now()
        .with(TransformComponent::new(Vector3::new(0., 0., 0.), 0.))
        .with(VelocityComponent::new())
        .with(CameraComponent)
        .build();

    // Terrain resource
    world.add_resource(Terrain::from(&scenario.map, empires.clone()));

    // Create entities for each unit in the SCN
    for player_id in scenario.player_ids() {
        let units = scenario.player_units(player_id);
        let civ_id = scenario.player_civilization_id(player_id);
        for unit in units {
            let transform_component = TransformComponent::new(Vector3::new(unit.position_x,
                                                                           unit.position_y,
                                                                           unit.position_z),
                                                              unit.rotation);
            let unit_component = UnitComponentBuilder::new(&empires)
                .with_player_id(player_id)
                .with_unit_id(unit.unit_id)
                .with_civilization_id(civ_id)
                .build();

            // TODO: Use the bulk creation iterator for better performance
            world.create_now()
                .with(transform_component)
                .with(VelocityComponent::new())
                .with(unit_component)
                .build();
        }
    }

    let mut planner = specs::Planner::<f32>::new(world, NUM_THREADS);
    planner.add_system(VelocitySystem::new(), "VelocitySystem", 100);
    planner.add_system(CameraInputSystem::new(), "CameraInputSystem", 1000);
    planner.add_system(CameraPositionSystem::new(), "CameraPositionSystem", 1001);
    planner.add_system(GridSystem::new(), "GridSystem", 2000);
    planner.add_system(AnimationSystem::new(empires.clone(), shape_metadata.clone()),
                       "AnimationSystem",
                       3000);
    planner
}
