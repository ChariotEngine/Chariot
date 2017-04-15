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

use dat::{EmpiresDb, EmpiresDbRef};
use media::{self, MediaRef};
use resource::{DrsManager, DrsManagerRef, GameDir, ShapeManager, ShapeManagerRef, ShapeMetadataStore,
               ShapeMetadataStoreRef};
use super::state::GameState;
use time;
use types::Fixed;

const WINDOW_TITLE: &'static str = "OpenAOE";
const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 768;

pub struct Game {
    game_dir: GameDir,
    drs_manager: DrsManagerRef,
    shape_manager: ShapeManagerRef,
    shape_metadata: ShapeMetadataStoreRef,
    empires: EmpiresDbRef,
    media: MediaRef,
    states: Vec<Box<GameState>>,
}

impl Game {
    pub fn new(game_data_dir: &str) -> Game {
        let game_dir = GameDir::new(game_data_dir).unwrap_or_else(|err| {
            unrecoverable!("{}", err);
        });

        let drs_manager = DrsManager::new(&game_dir);
        if let Err(err) = drs_manager.borrow_mut().preload() {
            unrecoverable!("Failed to preload DRS archives: {}", err);
        }

        let shape_manager = ShapeManager::new(drs_manager.clone()).unwrap_or_else(|err| {
            unrecoverable!("Failed to initialize the shape manager: {}", err);
        });

        let shape_metadata = ShapeMetadataStoreRef::new(ShapeMetadataStore::load(&*drs_manager.borrow()));

        let empires_dat_location = game_dir.find_file("data/empires.dat").unwrap();
        let empires = EmpiresDbRef::new(EmpiresDb::read_from_file(empires_dat_location)
            .unwrap_or_else(|err| {
                unrecoverable!("Failed to load empires.dat: {}", err);
            }));

        let media = media::create_media(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE).unwrap_or_else(|err| {
            unrecoverable!("Failed to create media window: {}", err);
        });

        Game {
            game_dir: game_dir,
            drs_manager: drs_manager,
            shape_manager: shape_manager,
            shape_metadata: shape_metadata,
            empires: empires,
            media: media,
            states: Vec::new(),
        }
    }

    pub fn push_state(&mut self, mut state: Box<GameState>) {
        if let Some(mut prev_state) = self.current_state() {
            prev_state.stop();
        }
        state.start();
        self.states.push(state);
    }

    pub fn game_loop(&mut self) {
        let time_step_nanos = 1000000000 / 60u64;
        let time_step_seconds = Fixed::from(1) / Fixed::from(60);

        let mut accumulator: u64 = 0;
        let mut last_time = time::precise_time_ns();

        let mut has_saved_bmp = true;

        while self.media.borrow().is_open() {
            self.media.borrow_mut().update();
            self.media.borrow_mut().renderer().present();

            if !has_saved_bmp {
                let mut media_borrow = self.media.borrow_mut();
                let renderer = media_borrow.renderer();
                has_saved_bmp = renderer.save_bmp("/Users/thill/Desktop/1.bmp");
            }

            let new_time = time::precise_time_ns();
            accumulator += new_time - last_time;
            last_time = new_time;

            while accumulator >= time_step_nanos {
                self.media.borrow_mut().update_input();
                self.update(time_step_seconds);
                accumulator -= time_step_nanos;
            }

            let lerp = Fixed::from(accumulator as f64 / time_step_nanos as f64);
            if let Some(state) = self.current_state() {
                state.render(lerp);
            }
        }
    }

    fn pop_state(&mut self) {
        if let Some(state) = self.current_state() {
            state.stop();
        }
        if !self.states.is_empty() {
            self.states.pop();
        }
    }

    fn update(&mut self, time_step: Fixed) -> bool {
        let mut pop_required = false;
        let result = if let Some(state) = self.current_state() {
            if !state.update(time_step) {
                pop_required = true;
                false
            } else {
                true
            }
        } else {
            false
        };
        if pop_required {
            self.pop_state();
        }
        result
    }

    fn current_state<'a>(&'a mut self) -> Option<&'a mut GameState> {
        if !self.states.is_empty() {
            let index = self.states.len() - 1; // satisfy the borrow checker
            Some(&mut *self.states[index])
        } else {
            None
        }
    }

    pub fn game_dir<'a>(&'a self) -> &'a GameDir {
        &self.game_dir
    }

    pub fn drs_manager(&self) -> DrsManagerRef {
        self.drs_manager.clone()
    }

    pub fn shape_manager(&self) -> ShapeManagerRef {
        self.shape_manager.clone()
    }

    pub fn shape_metadata(&self) -> ShapeMetadataStoreRef {
        self.shape_metadata.clone()
    }

    pub fn empires_db(&self) -> EmpiresDbRef {
        self.empires.clone()
    }

    pub fn media(&self) -> MediaRef {
        self.media.clone()
    }
}
