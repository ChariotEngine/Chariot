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

use specs;

#[derive(Clone, Debug)]
pub struct ActionQueueComponent {
    /// Note: actions should never be directly added to the action queue component,
    /// but should instead flow through the ActionBatcher resource. That way, actions
    /// can be easily synchronized across the network in multiplayer.
    actions: Vec<Action>,
    current_action: Option<Action>,
    current_action_done: bool,
}

impl specs::Component for ActionQueueComponent {
    type Storage = specs::VecStorage<ActionQueueComponent>;
}

impl ActionQueueComponent {
    pub fn new() -> ActionQueueComponent {
        ActionQueueComponent {
            actions: Vec::new(),
            current_action: None,
            current_action_done: true,
        }
    }

    /// This should be called from a system that actually performs and action
    /// after the action is completed. Marking it as done will tell the UnitActionSystem
    /// to move on to the next action in the queue.
    pub fn mark_current_done(&mut self) {
        self.current_action_done = true;
    }

    /// This should only ever be called by ActionBatcher
    pub fn add(&mut self, action: Action) {
        self.actions.push(action);
    }

    /// This should only ever be called by UnitActionSystem
    pub fn clear(&mut self) {
        self.actions.clear();
        // Don't overwrite the current_action; it's needed to remove components
        // in the UnitActionSystem. Instead, set done = true.
        self.current_action_done = true;
    }

    /// This should only ever be called by UnitActionSystem
    pub fn current_action_done(&self) -> bool {
        self.current_action_done
    }

    /// This should only ever be called by UnitActionSystem
    pub fn current_action(&self) -> &Option<Action> {
        &self.current_action
    }

    /// This should only ever be called by UnitActionSystem
    pub fn next_action(&mut self) {
        if !self.actions.is_empty() {
            self.current_action = Some(self.actions.remove(0));
            self.current_action_done = false;
        } else {
            self.current_action = None
        }
    }
}
