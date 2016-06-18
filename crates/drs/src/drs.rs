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

use std::path::Path;
use std::path::PathBuf;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::string::FromUtf8Error;

use byteorder::LittleEndian;
use byteorder::ReadBytesExt;

use quick_error::ResultExt;

quick_error! {
    #[derive(Debug)]
    pub enum DrsError {
        ReadError(err: io::Error, path: PathBuf) {
            display("failed to read DRS {:?}: {}", path, err)
            context(path: &'a Path, err: io::Error)
                -> (err, path.to_path_buf())
        }
        InvalidDrs(reason: &'static str, path: PathBuf) {
            display("invalid DRS {:?}: {}", path, reason)
        }
        EncodingError(err: FromUtf8Error, path: PathBuf) {
            display("invalid UTF-8 encoding in DRS {:?}: {}", path, err)
            context(path: &'a Path, err: FromUtf8Error)
                -> (err, path.to_path_buf())
        }
    }
}

pub type DrsResult<T> = Result<T, DrsError>;

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

    fn read_from_file(file: &mut File, file_name: &Path) -> DrsResult<DrsHeader> {
        let mut header = DrsHeader::new();
        try!(file.read_exact(&mut header.copyright_info).context(file_name));
        try!(file.read_exact(&mut header.file_version).context(file_name));
        try!(file.read_exact(&mut header.file_type).context(file_name));
        header.table_count = try!(file.read_u32::<LittleEndian>().context(file_name));
        header.file_offset = try!(file.read_u32::<LittleEndian>().context(file_name));

        try!(validate_ascii(file_name, &header.copyright_info[..],
            "Copyright (c) 1997 Ensemble Studios.\u{1A}"));
        try!(validate_ascii(file_name, &header.file_version[..], "1.00"));
        try!(validate_ascii(file_name, &header.file_type[..], "tribe"));
        Ok(header)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DrsFileType {
    Binary,
    Slp,
    Shp,
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
            _ => panic!("unknown file type encountered in DRS archive: 0x{:X}", binary_val),
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

    fn read_from_file(file: &mut File, file_name: &Path) -> DrsResult<DrsTableHeader> {
        let mut header = DrsTableHeader::new();

        header.file_type = DrsFileType::from(try!(file.read_u32::<LittleEndian>().context(file_name)));
        header.table_offset = try!(file.read_u32::<LittleEndian>().context(file_name));
        header.file_count = try!(file.read_u32::<LittleEndian>().context(file_name));
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

    fn read_from_file(file: &mut File, file_name: &Path) -> DrsResult<DrsTableEntry> {
        let mut entry = DrsTableEntry::new();
        entry.file_id = try!(file.read_u32::<LittleEndian>().context(file_name));
        entry.file_offset = try!(file.read_u32::<LittleEndian>().context(file_name));
        entry.file_size = try!(file.read_u32::<LittleEndian>().context(file_name));
        Ok(entry)
    }
}

pub type DrsFileContents = Vec<u8>;

pub struct DrsLogicalTable {
    pub header: DrsTableHeader,
    pub entries: Vec<DrsTableEntry>,
    pub contents: Vec<DrsFileContents>,
}

impl DrsLogicalTable {
    pub fn new() -> DrsLogicalTable {
        DrsLogicalTable {
            header: DrsTableHeader::new(),
            entries: Vec::new(),
            contents: Vec::new(),
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

    pub fn read_from_file<P: AsRef<Path>>(file_name: P) -> DrsResult<DrsFile> {
        let file_name = file_name.as_ref();
        let mut file = try!(File::open(file_name).context(file_name));

        let mut drs_file = DrsFile::new();
        drs_file.header = try!(DrsHeader::read_from_file(&mut file, file_name));
        for table_index in 0..drs_file.header.table_count {
            drs_file.tables.push(DrsLogicalTable::new());
            drs_file.tables[table_index as usize].header =
                try!(DrsTableHeader::read_from_file(&mut file, file_name));
        }
        for table_index in 0..drs_file.header.table_count {
            for _file_index in 0..drs_file.tables[table_index as usize].header.file_count {
                let table_entry = try!(DrsTableEntry::read_from_file(&mut file, file_name));
                drs_file.tables[table_index as usize].entries.push(table_entry);
            }
        }

        for table_index in 0..drs_file.header.table_count {
            let file_sizes: Vec<u32> = drs_file.tables[table_index as usize].entries.iter()
                .map(|e| e.file_size).collect();
            for file_size in file_sizes {
                let mut buffer = DrsFileContents::new();
                buffer.resize(file_size as usize, 0u8);
                try!(file.read_exact(&mut buffer[..]).context(file_name));
                drs_file.tables[table_index as usize].contents.push(buffer);
            }
        }

        Ok(drs_file)
    }
}

fn validate_ascii(file_name: &Path, bytes: &[u8], expected: &'static str) -> DrsResult<()> {
    let value: String = try!(String::from_utf8(Vec::from(bytes)).context(file_name));
    if !value.starts_with(expected) {
        return Err(DrsError::InvalidDrs("invalid value in header", file_name.to_path_buf()));
    }
    Ok(())
}
