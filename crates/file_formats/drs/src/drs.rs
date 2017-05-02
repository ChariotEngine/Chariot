// Chariot: An open source reimplementation of Age of Empires (1997)
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

use error::*;

use chariot_io_tools::ReadExt;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use std::path::Path;

const EXPECTED_COPYRIGHT: &'static str = "Copyright (c) 1997 Ensemble Studios.\u{1A}";
const EXPECTED_VERSION: &'static str = "1.00";
const EXPECTED_TYPE: &'static str = "tribe";

pub struct DrsHeader {
    pub copyright_info: [u8; 40],
    pub file_version: [u8; 4],
    pub file_type: [u8; 12],
    pub table_count: u32,
    pub file_offset: u32,
}

impl DrsHeader {
    pub fn new() -> DrsHeader {
        DrsHeader {
            copyright_info: [0u8; 40],
            file_version: [0u8; 4],
            file_type: [0u8; 12],
            table_count: 0,
            file_offset: 0,
        }
    }

    // TODO: Implement writing

    fn read_from_file(file: &mut File, file_name: &Path) -> Result<DrsHeader> {
        let mut header = DrsHeader::new();
        try!(file.read_exact(&mut header.copyright_info));
        try!(file.read_exact(&mut header.file_version));
        try!(file.read_exact(&mut header.file_type));
        header.table_count = try!(file.read_u32());
        header.file_offset = try!(file.read_u32());

        try!(validate_str(file_name, &header.copyright_info[..], EXPECTED_COPYRIGHT));
        try!(validate_str(file_name, &header.file_version[..], EXPECTED_VERSION));
        try!(validate_str(file_name, &header.file_type[..], EXPECTED_TYPE));
        Ok(header)
    }
}

/// DRS supported file types.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DrsFileType {
    /// "Binary" denotes several different kinds of files used by Age of Empires
    /// that are not graphics or sound (even if they're text files). For example, palettes
    /// are typically in this group.
    Binary,
    /// SLP graphics files (an AOE-specific proprietary format).
    Slp,
    /// SHP graphics files (AOE-specific; hardly used).
    Shp,
    /// Typical WAV audio files.
    Wav,
}

// TODO: Move to using TryFrom when it becomes generally available in Rust
impl From<u32> for DrsFileType {
    fn from(binary_val: u32) -> Self {
        // It looks like the Age of Empires devs decided to store the file types as little endian
        // integers (probably for faster/easier deserialization and type lookup). For binary files,
        // they decided to use "bina", while for all of the other files, they used the file
        // extension with a space (i.e., "wav ").
        match binary_val {
            0x62696E61 => DrsFileType::Binary,
            0x736C7020 => DrsFileType::Slp,
            0x73687020 => DrsFileType::Shp,
            0x77617620 => DrsFileType::Wav,
            _ => {
                panic!("unknown file type encountered in DRS archive: 0x{:X}",
                       binary_val)
            }
        }
    }
}

pub struct DrsTableHeader {
    pub file_type: DrsFileType,
    pub table_offset: u32,
    pub file_count: u32,
}

impl DrsTableHeader {
    pub fn new() -> DrsTableHeader {
        DrsTableHeader {
            file_type: DrsFileType::Binary,
            table_offset: 0u32,
            file_count: 0u32,
        }
    }

    // TODO: Implement writing

    fn read_from_file<R: Read>(file: &mut R) -> Result<DrsTableHeader> {
        let mut header = DrsTableHeader::new();

        header.file_type = DrsFileType::from(try!(file.read_u32()));
        header.table_offset = try!(file.read_u32());
        header.file_count = try!(file.read_u32());
        Ok(header)
    }

    pub fn file_extension(&self) -> &'static str {
        match self.file_type {
            DrsFileType::Binary => "bin",
            DrsFileType::Slp => "slp",
            DrsFileType::Shp => "shp",
            DrsFileType::Wav => "wav",
        }
    }
}

pub struct DrsTableEntry {
    pub file_id: u32,
    pub file_offset: u32,
    pub file_size: u32,
}

impl DrsTableEntry {
    pub fn new() -> DrsTableEntry {
        DrsTableEntry {
            file_id: 0u32,
            file_offset: 0u32,
            file_size: 0u32,
        }
    }

    // TODO: Implement writing

    fn read_from_file<R: Read>(file: &mut R) -> Result<DrsTableEntry> {
        let mut entry = DrsTableEntry::new();
        entry.file_id = try!(file.read_u32());
        entry.file_offset = try!(file.read_u32());
        entry.file_size = try!(file.read_u32());
        Ok(entry)
    }
}

pub type DrsFileContents = Vec<u8>;

/// Tables aren't actually stored in the DRS files in this layout, but instead, this
/// struct exists like this to make it more convenient to pull data out of the tables
/// after the DRS file has been read.
pub struct DrsLogicalTable {
    pub header: DrsTableHeader,
    pub entries: Vec<DrsTableEntry>,
    pub contents: Vec<DrsFileContents>,
    index_map: HashMap<u32, usize>,
}

impl DrsLogicalTable {
    pub fn new() -> DrsLogicalTable {
        DrsLogicalTable {
            header: DrsTableHeader::new(),
            entries: Vec::new(),
            contents: Vec::new(),
            index_map: HashMap::new(),
        }
    }

    /// All files present inside of a DRS archive are labeled with a 32-bit integer file ID.
    /// This method attempts to find a file by ID in the given table.
    pub fn find_file_contents(&self, file_id: u32) -> Option<&DrsFileContents> {
        match self.index_map.get(&file_id) {
            Some(index) => Some(&self.contents[*index]),
            None => None,
        }
    }

    fn populate_index_map(&mut self) {
        for i in 0..self.entries.len() {
            self.index_map.insert(self.entries[i].file_id, i);
        }
    }
}

pub struct DrsFile {
    pub header: DrsHeader,
    pub tables: Vec<DrsLogicalTable>,
}

impl DrsFile {
    pub fn new() -> DrsFile {
        DrsFile {
            header: DrsHeader::new(),
            tables: Vec::new(),
        }
    }

    /// DRS archives are partitioned into tables by file type. This method will
    /// attempt to find a table of the requested type, and return None if it doesn't exist.
    pub fn find_table(&self, file_type: DrsFileType) -> Option<&DrsLogicalTable> {
        for table in &self.tables {
            if table.header.file_type == file_type {
                return Some(table);
            }
        }
        return None;
    }

    /// Loads a DRS archive from the file system.
    pub fn read_from_file<P: AsRef<Path>>(file_name: P) -> Result<DrsFile> {
        let file_name = file_name.as_ref();
        let mut file = try!(File::open(file_name));

        let mut drs_file = DrsFile::new();
        drs_file.header = try!(DrsHeader::read_from_file(&mut file, file_name));
        try!(DrsFile::read_table_headers(&mut file, &mut drs_file));
        try!(DrsFile::read_file_entry_headers(&mut file, &mut drs_file));
        try!(DrsFile::read_file_contents(&mut file, &mut drs_file));

        for table in &mut drs_file.tables {
            table.populate_index_map();
        }

        Ok(drs_file)
    }

    fn read_table_headers<R: Read>(file: &mut R, drs_file: &mut DrsFile) -> Result<()> {
        for table_index in 0..drs_file.header.table_count {
            drs_file.tables.push(DrsLogicalTable::new());
            drs_file.tables[table_index as usize].header = try!(DrsTableHeader::read_from_file(file));
        }
        Ok(())
    }

    fn read_file_entry_headers<R: Read>(file: &mut R, drs_file: &mut DrsFile) -> Result<()> {
        for table_index in 0..drs_file.header.table_count {
            for _file_index in 0..drs_file.tables[table_index as usize].header.file_count {
                let table_entry = try!(DrsTableEntry::read_from_file(file));
                drs_file.tables[table_index as usize].entries.push(table_entry);
            }
        }
        Ok(())
    }

    fn read_file_contents<R: Read>(file: &mut R, drs_file: &mut DrsFile) -> Result<()> {
        for table_index in 0..drs_file.header.table_count {
            let file_sizes: Vec<u32> = drs_file.tables[table_index as usize]
                .entries
                .iter()
                .map(|e| e.file_size)
                .collect();
            for file_size in file_sizes {
                let mut buffer = vec![0u8; file_size as usize];
                try!(file.read_exact(&mut buffer[..]));
                drs_file.tables[table_index as usize].contents.push(buffer);
            }
        }
        Ok(())
    }
}

fn validate_str(file_name: &Path, bytes: &[u8], expected: &'static str) -> Result<()> {
    if bytes.len() < expected.len() || &bytes[0..expected.len()] != expected.as_bytes() {
        return Err(ErrorKind::InvalidDrs(file_name.into()).into());
    }
    Ok(())
}
