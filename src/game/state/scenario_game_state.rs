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

use ecs::resource::{KeyboardKeyStates, MouseState, RenderCommands, Viewport};
use ecs;
use game::{Game, GameState};

use media::MediaRef;
use resource::ShapeManagerRef;
use scn;

use nalgebra::{Cast, Vector2};
use specs;

pub struct ScenarioGameState {
    media: MediaRef,
    shape_manager: ShapeManagerRef,
    planner: specs::Planner<(ecs::SystemGroup, f32)>,
}

impl ScenarioGameState {
    pub fn new(g: &Game, scenario: scn::Scenario) -> ScenarioGameState {
        ScenarioGameState {
            media: g.media(),
            shape_manager: g.shape_manager(),
            planner: ecs::create_world_planner(g.media(), g.empires_db(), g.shape_metadata(), &scenario),
        }
    }

    fn update_viewport(&mut self, lerp: f32) {
        let viewport = self.planner.mut_world().read_resource::<Viewport>();
        let top_left: Vector2<i32> = Cast::from(viewport.lerped_top_left(lerp));
        self.media.borrow_mut().renderer().set_camera_position(&top_left);
    }

    fn update_input_resources(&mut self) {
        let world = self.planner.mut_world();
        let (mut keys, mut mouse_state) = {
            (world.write_resource::<KeyboardKeyStates>(), world.write_resource::<MouseState>())
        };

        let media = self.media.borrow();
        *keys = media.key_states().clone();
        (*mouse_state).position = media.mouse_position().clone();
        (*mouse_state).key_states = media.mouse_button_states().clone();
    }
}

impl GameState for ScenarioGameState {
    fn start(&mut self) {}

    fn stop(&mut self) {}

    fn update(&mut self, time_step: f32) -> bool {
        self.update_input_resources();

        {
            let world = self.planner.mut_world();
            let mut render_commands = world.write_resource::<RenderCommands>();
            render_commands.clear_debug();
        }

        self.planner.dispatch((ecs::SystemGroup::Normal, time_step));
        self.planner.wait();

        true
    }

    fn render(&mut self, lerp: f32) {
        self.update_viewport(lerp);

        self.planner.dispatch((ecs::SystemGroup::Render, lerp));
        self.planner.wait();

        let world = self.planner.mut_world();
        let mut render_commands = world.write_resource::<RenderCommands>();
        render_commands.execute(self.media.borrow_mut().renderer(),
                                &mut *self.shape_manager.borrow_mut());
        render_commands.clear_rendered();
    }
}
