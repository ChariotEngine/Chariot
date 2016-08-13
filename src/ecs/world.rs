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

use dat::EmpiresDbRef;
use ecs::render_system::*;
use ecs::resource::*;
use ecs::system::*;
use media::MediaRef;
use partition::GridPartition;
use resource::ShapeMetadataStoreRef;
use scn;
use specs;
use std::collections::HashMap;
use super::component::*;
use types::{Fixed, Vector3};

const NUM_THREADS: usize = 4;
const GRID_CELL_SIZE: i32 = 10; // in tiles

pub type WorldPlanner = specs::Planner<(SystemGroup, Fixed)>;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SystemGroup {
    Normal,
    Render,
}

pub fn create_world_planner(media: MediaRef,
                            empires: EmpiresDbRef,
                            shape_metadata: ShapeMetadataStoreRef,
                            scenario: &scn::Scenario)
                            -> WorldPlanner {
    let mut world = specs::World::new();
    register_components(&mut world);
    add_resources(&mut world, &media, &empires, scenario);

    // Create entities for each unit in the SCN
    for player_id in scenario.player_ids() {
        let units = scenario.player_units(player_id);
        let civ_id = scenario.player_civilization_id(player_id);
        for unit in units {
            let transform_component = TransformComponent::new(Vector3::new(unit.position_x.into(),
                                                                           unit.position_y.into(),
                                                                           unit.position_z.into()),
                                                              unit.rotation.into());

            let unit_info = empires.unit(civ_id, unit.unit_id);

            let mut graphic_component = GraphicComponent::new();
            graphic_component.player_color_id = player_id.into();
            graphic_component.graphic_id = unit_info.standing_graphic;

            // TODO: Use the bulk creation iterator for better performance
            world.create_now()
                .with(ActionQueueComponent::new())
                .with(transform_component)
                .with(graphic_component)
                .with(UnitComponent::new(player_id, civ_id, unit.unit_id))
                .with(VelocityComponent::new())
                .build();
        }
    }

    let mut planner = WorldPlanner::new(world, NUM_THREADS);
    attach_systems(&mut planner, &empires, &shape_metadata);
    attach_render_systems(&mut planner, &empires);
    planner
}

fn register_components(world: &mut specs::World) {
    world.register::<ActionQueueComponent>();
    world.register::<CameraComponent>();
    world.register::<GraphicComponent>();
    world.register::<MoveToPositionActionComponent>();
    world.register::<SelectedUnitComponent>();
    world.register::<TransformComponent>();
    world.register::<UnitComponent>();
    world.register::<VelocityComponent>();
    world.register::<VisibleUnitComponent>();
}

fn add_resources(world: &mut specs::World,
                 media: &MediaRef,
                 empires: &EmpiresDbRef,
                 scenario: &scn::Scenario) {
    let viewport_size = media.borrow().viewport_size();
    let (tile_half_width, tile_half_height) = empires.tile_half_sizes();

    // Input resources
    world.add_resource(KeyboardKeyStates::new(HashMap::new()));
    world.add_resource(MouseState::new());

    // Render resources
    world.add_resource(RenderCommands::new());
    world.add_resource(ViewProjector::new(tile_half_width, tile_half_height));
    world.add_resource(GridPartition::new(GRID_CELL_SIZE, GRID_CELL_SIZE));

    // Camera resources and entity
    world.add_resource(Viewport::new(viewport_size.x as i32, viewport_size.y as i32));
    world.create_now()
        .with(TransformComponent::new(Vector3::new(0.into(), 0.into(), 0.into()), 0.into()))
        .with(VelocityComponent::new())
        .with(CameraComponent)
        .build();

    // Unit resources
    world.add_resource(ActionBatcher::new());

    // Terrain resource
    world.add_resource(Terrain::from(&scenario.map, empires.clone()));
}

macro_rules! system {
    ($planner:expr, $typ:ident, $priority:expr) => {
        $planner.add_system(SystemWrapper::new(Box::new($typ::new())), stringify!($typ), $priority);
    };
    ($planner:expr, $typ:ident, $inst:expr, $priority:expr) => {
        $planner.add_system(SystemWrapper::new(Box::new($inst)), stringify!($typ), $priority);
    };
}

fn attach_systems(planner: &mut WorldPlanner,
                  empires: &EmpiresDbRef,
                  shape_metadata: &ShapeMetadataStoreRef) {
    system!(planner, VelocitySystem, 1000);
    system!(planner, CameraInputSystem, 1000);
    system!(planner, CameraPositionSystem, 1000);
    system!(planner, CameraPositionSystem, 1000);
    system!(planner, GridSystem, 1000);
    system!(planner,
            AnimationSystem,
            AnimationSystem::new(empires.clone(), shape_metadata.clone()),
            1000);
    system!(planner, UnitActionSystem, UnitActionSystem::new(), 1000);
    system!(planner,
            UnitSelectionSystem,
            UnitSelectionSystem::new(empires.clone()),
            1000);
    system!(planner,
            MoveToPositionActionSystem,
            MoveToPositionActionSystem::new(empires.clone()),
            1000);
}

macro_rules! render_system {
    ($planner:expr, $typ:ident, $priority:expr) => {
        $planner.add_system(RenderSystemWrapper::new(Box::new($typ::new())), stringify!($typ), $priority);
    };
    ($planner:expr, $typ:ident, $inst:expr, $priority:expr) => {
        $planner.add_system(RenderSystemWrapper::new(Box::new($inst)), stringify!($typ), $priority);
    };
}

fn attach_render_systems(planner: &mut WorldPlanner, empires: &EmpiresDbRef) {
    render_system!(planner,
                   TerrainRenderSystem,
                   TerrainRenderSystem::new(empires.clone()),
                   1000);
    render_system!(planner,
                   GraphicRenderSystem,
                   GraphicRenderSystem::new(empires.clone()),
                   1000);
    render_system!(planner,
                   UnitSelectionRenderSystem,
                   UnitSelectionRenderSystem::new(empires.clone()),
                   1000);
    render_system!(planner, TileDebugRenderSystem, 1000);
}
