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

use ecs::render_system::{TerrainRenderSystem, TileDebugRenderSystem, UnitRenderSystem};
use ecs::resource::{MouseState, PressedKeys, Viewport};
use ecs;
use game::{Game, GameState};

use media::MediaRef;
use scn;

use nalgebra::{Cast, Vector2};
use specs;

pub struct ScenarioGameState {
    media: MediaRef,
    planner: specs::Planner<f32>,
    terrain_render_system: TerrainRenderSystem,
    unit_render_system: UnitRenderSystem,
    tile_debug_render_system: TileDebugRenderSystem,
}

impl ScenarioGameState {
    pub fn new(g: &Game, scenario: scn::Scenario) -> ScenarioGameState {
        ScenarioGameState {
            media: g.media(),
            planner: ecs::create_world_planner(g.media(), g.empires_db(), &scenario),
            terrain_render_system: TerrainRenderSystem::new(g.media(),
                                                            g.shape_manager(),
                                                            g.empires_db()),
            unit_render_system: UnitRenderSystem::new(g.media(), g.shape_manager(), g.empires_db()),
            tile_debug_render_system: TileDebugRenderSystem::new(g.media(), g.shape_manager()),
        }
    }

    fn update_viewport(&mut self) {
        let viewport = self.planner.mut_world().read_resource::<Viewport>();
        let top_left: Vector2<i32> = Cast::from(*viewport.top_left());
        self.media.borrow_mut().renderer().set_camera_position(&top_left);
    }

    fn update_input_resources(&mut self) {
        let world = self.planner.mut_world();
        let (mut keys, mut mouse_state) = {
            (world.write_resource::<PressedKeys>(), world.write_resource::<MouseState>())
        };

        (*keys).0 = self.media.borrow().pressed_keys().clone();
        mouse_state.position = self.media.borrow().mouse_position().clone();
    }
}

impl GameState for ScenarioGameState {
    fn start(&mut self) {}

    fn stop(&mut self) {}

    fn update(&mut self, time_step: f32) -> bool {
        self.update_input_resources();

        self.planner.dispatch(time_step);
        self.planner.wait();

        self.update_viewport();

        true
    }

    fn render(&mut self, lerp: f32) {
        let mut world = self.planner.mut_world();
        self.terrain_render_system.render(world);
        self.unit_render_system.render(world, lerp);
        self.tile_debug_render_system.render(world);
    }
}
