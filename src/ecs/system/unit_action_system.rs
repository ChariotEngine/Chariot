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

use action::Action;
use ecs::component::*;
use ecs::resource::ActionBatcher;
use specs::{self, Join};
use super::System;
use types::Fixed;

// This is just a temporary batch length value
// It'll be subject to the latencies of networking later
const TURN_LENGTH_SECONDS: Fixed = fixed_const!(0.1);

macro_rules! detach_action_component {
    ($action:expr, $entity:expr, $mtps:expr) => {
        println!("Stopping action: {:?}", $action);
        match $action {
            Action::MoveToPosition(_) => { $mtps.remove($entity); }
            _ => panic!("Failed to detach unknown action: {:?}", $action)
        }
    }
}

macro_rules! attach_action_component {
    ($action:expr, $entity:expr, $mtps:expr) => {
        println!("Starting action: {:?}", $action);
        match $action {
            Action::MoveToPosition(ref params) => {
                $mtps.insert($entity, MoveToPositionActionComponent::new(params.position));
            }
            _ => panic!("Failed to attach unknown action: {:?}", $action)
        }
    }
}

/// This system exists to take the actions batched up in the ActionBatcher
/// synchronize them across the network (in multiplayer), and then add
/// the actions to each individual unit.
pub struct UnitActionSystem {
    turn_accumulator: Fixed,
}

impl UnitActionSystem {
    pub fn new() -> UnitActionSystem {
        UnitActionSystem { turn_accumulator: 0.into() }
    }
}

impl System for UnitActionSystem {
    fn update(&mut self, arg: specs::RunArg, time_step: Fixed) {
        let (mut action_batcher, mut action_queues, entities, mut mtps) = arg.fetch(|w| {
            (w.write_resource::<ActionBatcher>(),
             w.write::<ActionQueueComponent>(),
             w.entities(),
             w.write::<MoveToPositionActionComponent>())
        });

        self.turn_accumulator += time_step;
        if self.turn_accumulator >= TURN_LENGTH_SECONDS {
            self.turn_accumulator -= TURN_LENGTH_SECONDS;

            let action_batch = action_batcher.consume_actions();
            for (entity, action_queue) in (&entities, &mut action_queues).iter() {
                if let Some(actions) = action_batch.get(&entity.get_id()) {
                    for action in actions {
                        match *action {
                            Action::ClearQueue => action_queue.clear(),
                            _ => action_queue.add(action.clone()),
                        }
                    }
                }
            }
        }

        for (entity, action_queue) in (&entities, &mut action_queues).iter() {
            // If action queue has no current action, then take
            // an action from the top of the queue and initialize it
            // by creating a component on that entity for the action type.
            // Handle the actual action via separate systems.
            if action_queue.current_action_done() {
                if let &Some(ref action) = action_queue.current_action() {
                    detach_action_component!(*action, entity, &mut mtps);
                }
                action_queue.next_action();

                if let &Some(ref action) = action_queue.current_action() {
                    attach_action_component!(*action, entity, &mut mtps);
                }
            }
        }
    }
}
