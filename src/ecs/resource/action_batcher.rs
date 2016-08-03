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
use specs::Index;
use std::collections::HashMap;
use std::mem;

pub struct ActionBatcher {
    actions: HashMap<Index, Vec<Action>>,
}

impl ActionBatcher {
    pub fn new() -> ActionBatcher {
        ActionBatcher { actions: HashMap::new() }
    }

    pub fn queue_for_entity(&mut self, entity_id: Index, action: Action) {
        if !self.actions.contains_key(&entity_id) {
            self.actions.insert(entity_id, Vec::new());
        }
        self.actions.get_mut(&entity_id).unwrap().push(action);
    }

    pub fn consume_actions(&mut self) -> HashMap<Index, Vec<Action>> {
        let mut consumed = HashMap::new();
        mem::swap(&mut consumed, &mut self.actions);
        consumed
    }
}
