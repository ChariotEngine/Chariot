//
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
//

use std::fmt;

/// ID for player colors in the empires.dat file
#[derive(Default, Clone, Copy)]
pub struct PlayerColorId(pub isize);

/// ID for an SLP file in a DRS package
#[derive(Default, Clone, Copy)]
pub struct SlpFileId(pub isize);

/// ID for a WAV file in a DRS package
#[derive(Default, Clone, Copy)]
pub struct WavFileId(pub isize);

/// ID for a graphic in the empires.dat file
#[derive(Default, Clone, Copy)]
pub struct GraphicId(pub isize);

/// ID for a sound group in the empires.dat file
#[derive(Default, Clone, Copy)]
pub struct SoundGroupId(pub isize);

/// ID for an age (that defines effects for research) in the empires.dat file
#[derive(Default, Clone, Copy)]
pub struct AgeId(pub isize);

/// ID for research in the empires.dat file
#[derive(Default, Clone, Copy)]
pub struct ResearchId(pub isize);

/// ID for units in the empires.dat file
#[derive(Default, Clone, Copy)]
pub struct UnitId(pub isize);

/// ID for unit classes in the empires.dat file
#[derive(Default, Clone, Copy)]
pub struct UnitClassId(pub isize);

/// ID for unit commands in the empires.dat file
#[derive(Default, Clone, Copy)]
pub struct UnitCommandId(pub isize);

/// ID for finding a localized string in language.dll
#[derive(Default, Clone, Copy)]
pub struct LocalizationId(pub isize);

/// ID for random map script references in the empires.dat file
#[derive(Default, Clone, Copy)]
pub struct RandomMapScriptId(pub isize);

/// ID for terrains defined in empires.dat
#[derive(Default, Clone, Copy)]
pub struct TerrainId(pub isize);

// Implement Debug instead of deriving it so that we can keep it all
// on one line when formatted with {:#?}
macro_rules! impl_id_debug {
    ($id_type:ty) => {
        impl fmt::Debug for $id_type {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, concat!(stringify!($id_type), "({})"), self.0)
            }
        }
    }
}

impl_id_debug!(PlayerColorId);
impl_id_debug!(SlpFileId);
impl_id_debug!(WavFileId);
impl_id_debug!(GraphicId);
impl_id_debug!(SoundGroupId);
impl_id_debug!(AgeId);
impl_id_debug!(ResearchId);
impl_id_debug!(UnitId);
impl_id_debug!(UnitClassId);
impl_id_debug!(UnitCommandId);
impl_id_debug!(LocalizationId);
impl_id_debug!(RandomMapScriptId);
impl_id_debug!(TerrainId);
