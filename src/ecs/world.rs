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

use ecs::resource::{MouseState, PressedKeys, ViewProjector, Viewport};
use ecs::system::{CameraInputSystem, CameraPositionSystem, GridSystem, VelocitySystem};
use partition::GridPartition;

use dat::EmpiresDbRef;
use media::MediaRef;

use specs;

use std::collections::HashSet;

const NUM_THREADS: usize = 4;
const GRID_CELL_SIZE: i32 = 10; // in tiles

pub fn create_world_planner(media: MediaRef, empires: EmpiresDbRef) -> specs::Planner<()> {
    let viewport_size = media.borrow().viewport_size();
    let (tile_half_width, tile_half_height) = (empires.terrain_block.tile_half_width as i32,
                                               empires.terrain_block.tile_half_height as i32);

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
        // Temporary hardcoded camera offset
        .with(TransformComponent::new(126f32 * tile_half_width as f32,
                                      -145f32 * tile_half_height as f32, 0., 0.))
        .with(VelocityComponent::new())
        .with(CameraComponent)
        .build();

    let mut planner = specs::Planner::<()>::new(world, NUM_THREADS);
    planner.add_system(VelocitySystem::new(), "VelocitySystem", 100);
    planner.add_system(CameraInputSystem::new(), "CameraInputSystem", 1000);
    planner.add_system(CameraPositionSystem::new(), "CameraPositionSystem", 1001);
    planner.add_system(GridSystem::new(), "GridSystem", 2000);
    planner
}
