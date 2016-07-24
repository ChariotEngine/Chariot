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

use drs_manager::{DrsKey, DrsManager};

use drs::DrsFileType;
use identifier::SlpFileId;
use slp::SlpHeader;

use std::collections::HashMap;
use std::sync::Arc;
use std::io;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct ShapeMetadataKey {
    pub drs_key: DrsKey,
    pub slp_id: SlpFileId,
}

impl ShapeMetadataKey {
    pub fn new(drs_key: DrsKey, slp_id: SlpFileId) -> ShapeMetadataKey {
        ShapeMetadataKey {
            drs_key: drs_key,
            slp_id: slp_id,
        }
    }
}

pub struct ShapeMetadata {
    pub shape_count: u32,
}

pub struct ShapeMetadataStore {
    metadata: HashMap<ShapeMetadataKey, ShapeMetadata>,
}

pub type ShapeMetadataStoreRef = Arc<ShapeMetadataStore>;

impl ShapeMetadataStore {
    pub fn load(drs_manager: &DrsManager) -> ShapeMetadataStore {
        let drs = drs_manager.get(DrsKey::Graphics);
        let mut metadata = HashMap::new();
        if let Some(table) = drs.find_table(DrsFileType::Slp) {
            let keys: Vec<ShapeMetadataKey> = table.entries
                .iter()
                .map(|entry| ShapeMetadataKey::new(DrsKey::Graphics, (entry.file_id as usize).into()))
                .collect();

            for key in keys {
                // TODO: Should probably return a result instead of unwrapping
                let contents = table.find_file_contents(*key.slp_id).unwrap();
                let slp_header = SlpHeader::read_from(&mut io::Cursor::new(contents)).unwrap();
                metadata.insert(key, ShapeMetadata { shape_count: slp_header.shape_count });
            }
        }
        ShapeMetadataStore { metadata: metadata }
    }

    pub fn get<'a>(&'a self, key: &ShapeMetadataKey) -> Option<&'a ShapeMetadata> {
        self.metadata.get(key)
    }
}
