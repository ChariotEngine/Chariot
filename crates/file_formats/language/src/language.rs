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

use error::{Result, ErrorKind};

use chariot_io_tools::{ReadExt, ReadArrayExt};

use std::collections::BTreeMap;
use std::fs::File;
use std::io::SeekFrom;
use std::io::prelude::{Read, Seek};
use std::path::Path;

#[derive(Debug)]
pub struct Language {
    pub strings: BTreeMap<usize, String>,
}

impl Language {
    fn new() -> Language {
        Language { strings: BTreeMap::new() }
    }

    pub fn read_from_file<P: AsRef<Path>>(file_name: P) -> Result<Language> {
        let file_name = file_name.as_ref();
        let mut file = try!(File::open(file_name));
        Language::read_from_stream(&mut file)
    }

    pub fn read_from_stream<S: Read + Seek>(stream: &mut S) -> Result<Language> {
        try!(move_to_pe_header(stream));
        let pe_header = try!(read_pe_header(stream));

        let data_directories = try!(read_pe_image_data_directories(stream));
        let resource_data_directory = &data_directories[DATA_DIRECTORY_RESOURCE_INDEX];

        let resource_section = try!(read_pe_image_resource_section_header(pe_header.section_count,
                                                                          resource_data_directory,
                                                                          stream));

        let resource_root_offset = resource_section.raw_data_offset as u64;
        let string_directory = try!(read_pe_string_resource_directory(stream, resource_root_offset));
        let string_entry_map =
            try!(read_pe_string_resource_entries(stream, resource_root_offset, &string_directory));

        let mut language = Language::new();
        for (dir_id, string_entries) in &string_entry_map {
            for string_entry in string_entries {
                let data_entry =
                    try!(read_pe_resource_data_entry(stream, resource_root_offset, string_entry));
                let start_pos = resource_root_offset +
                                (data_entry.data_offset - resource_section.virtual_address) as u64;
                let end_pos = start_pos + data_entry.size as u64;

                try!(stream.seek(SeekFrom::Start(start_pos)));
                let mut string_id = ((dir_id - 1) * 16) as usize;
                loop {
                    let pos = try!(stream.seek(SeekFrom::Current(0)));
                    if pos >= end_pos {
                        break;
                    }

                    let len = try!(stream.read_u16());
                    if len > 0 {
                        let mut words = Vec::new();
                        for _ in 0..len {
                            words.push(try!(stream.read_u16()));
                        }
                        let text = try!(String::from_utf16(&words));
                        language.strings.insert(string_id, text);
                    }
                    string_id += 1;
                }
            }
        }

        Ok(language)
    }
}

const NEW_EXE_HEADER_ADDRESS_OFFSET: u64 = 60;
const DATA_DIRECTORY_OFFSET_FROM_OPTIONAL_HEADER: i64 = 96;

const DATA_DIRECTORY_COUNT: usize = 16;
const DATA_DIRECTORY_RESOURCE_INDEX: usize = 2;

const RESOURCE_TYPE_STRING: u32 = 6;

#[derive(Debug)]
struct PeImageHeader {
    machine: u16,
    section_count: u16,
    timestamp: u32,
    symbol_table_offset: u32,
    symbol_count: u32,
    optional_header_size: u16,
    characteristics: u16,
}

#[derive(Debug)]
struct PeImageDataDirectory {
    virtual_address: u32,
    size: u32,
}

type PeImageDataDirectories = Vec<PeImageDataDirectory>;

#[derive(Debug, Clone)]
struct PeImageSectionHeader {
    name: String,
    physical_address: u32,
    virtual_address: u32,
    raw_data_size: u32,
    raw_data_offset: u32,
    relocations_offset: u32,
    line_numbers_offset: u32,
    relocation_count: u16,
    line_number_count: u16,
    characteristics: u32,
}

type PeImageSectionHeaders = Vec<PeImageSectionHeader>;

#[derive(Debug)]
struct PeImageResourceDirectory {
    characteristics: u32,
    timestamp: u32,
    major_version: u16,
    minor_version: u16,
    named_entry_count: u16,
    id_entry_count: u16,

    entries: Vec<PeImageResourceDirectoryEntry>,
}

#[derive(Debug)]
struct PeImageResourceDirectoryEntry {
    name: u32,
    data_offset: u32,
}

#[derive(Debug)]
struct PeImageResourceDataEntry {
    data_offset: u32,
    size: u32,
    code_page: u32,
    reserved: u32,
}

fn move_to_pe_header<S: Read + Seek>(stream: &mut S) -> Result<()> {
    try!(stream.seek(SeekFrom::Start(0)));
    let mz_magic = try!(stream.read_sized_str(2));
    if mz_magic != "MZ" {
        return Err(ErrorKind::InvalidPeMagic.into());
    }

    try!(stream.seek(SeekFrom::Start(NEW_EXE_HEADER_ADDRESS_OFFSET)));

    let pe_header_offset = try!(stream.read_u32()) as u64;
    try!(stream.seek(SeekFrom::Start(pe_header_offset)));
    Ok(())
}

fn read_pe_header<S: Read + Seek>(stream: &mut S) -> Result<PeImageHeader> {
    let pe_magic = try!(stream.read_sized_str(2));
    if pe_magic != "PE" {
        return Err(ErrorKind::InvalidPeMagic.into());
    }
    try!(stream.seek(SeekFrom::Current(2)));

    Ok(PeImageHeader {
        machine: try!(stream.read_u16()),
        section_count: try!(stream.read_u16()),
        timestamp: try!(stream.read_u32()),
        symbol_table_offset: try!(stream.read_u32()),
        symbol_count: try!(stream.read_u32()),
        optional_header_size: try!(stream.read_u16()),
        characteristics: try!(stream.read_u16()),
    })
}

fn read_pe_image_data_directories<S: Read + Seek>(stream: &mut S) -> Result<PeImageDataDirectories> {
    try!(stream.seek(SeekFrom::Current(DATA_DIRECTORY_OFFSET_FROM_OPTIONAL_HEADER)));

    let mut directories = Vec::new();
    for _ in 0..DATA_DIRECTORY_COUNT {
        directories.push(PeImageDataDirectory {
            virtual_address: try!(stream.read_u32()),
            size: try!(stream.read_u32()),
        });
    }

    Ok(directories)
}

fn read_pe_image_section_headers<S: Read + Seek>(section_count: u16,
                                                 stream: &mut S)
                                                 -> Result<PeImageSectionHeaders> {
    stream.read_array(section_count as usize, |c| read_pe_image_section_header(c))
}

fn read_pe_image_section_header<S: Read + Seek>(stream: &mut S) -> Result<PeImageSectionHeader> {
    Ok(PeImageSectionHeader {
        name: try!(stream.read_sized_str(8)),
        physical_address: try!(stream.read_u32()),
        virtual_address: try!(stream.read_u32()),
        raw_data_size: try!(stream.read_u32()),
        raw_data_offset: try!(stream.read_u32()),
        relocations_offset: try!(stream.read_u32()),
        line_numbers_offset: try!(stream.read_u32()),
        relocation_count: try!(stream.read_u16()),
        line_number_count: try!(stream.read_u16()),
        characteristics: try!(stream.read_u32()),
    })
}

fn read_pe_image_resource_section_header<S: Read + Seek>(section_count: u16,
                                                         resource_data_directory: &PeImageDataDirectory,
                                                         stream: &mut S)
                                                         -> Result<PeImageSectionHeader> {
    let sections = try!(read_pe_image_section_headers(section_count, stream));
    let resource_section = sections.into_iter().find(&|section: &PeImageSectionHeader| {
        section.virtual_address == resource_data_directory.virtual_address
    });
    match resource_section {
        Some(section) => Ok(section),
        None => Err(ErrorKind::ResourceSectionNotFound.into()),
    }
}

fn read_pe_resource_directories<S: Read + Seek>(stream: &mut S) -> Result<PeImageResourceDirectory> {
    let mut directory = PeImageResourceDirectory {
        characteristics: try!(stream.read_u32()),
        timestamp: try!(stream.read_u32()),
        major_version: try!(stream.read_u16()),
        minor_version: try!(stream.read_u16()),
        named_entry_count: try!(stream.read_u16()),
        id_entry_count: try!(stream.read_u16()),
        entries: Vec::new(),
    };

    // We're not considering named entries here which could potentially be a problem
    // if someone modifies the language.dll (the original uses id entries)

    for _ in 0..(directory.id_entry_count as usize) {
        directory.entries.push(PeImageResourceDirectoryEntry {
            name: try!(stream.read_u32()),
            // Clear the high bit that indicates the next level is a sub directory
            // We'll just assume it is
            data_offset: try!(stream.read_u32()) & 0x7FFFFFFF,
        });
    }

    Ok(directory)
}

fn read_pe_string_resource_directory<S: Read + Seek>(stream: &mut S,
                                                     resource_root_offset: u64)
                                                     -> Result<PeImageResourceDirectoryEntry> {
    try!(stream.seek(SeekFrom::Start(resource_root_offset)));

    let resource_directories = try!(read_pe_resource_directories(stream));
    let string_directory = resource_directories.entries
        .into_iter()
        .find(&|dir: &PeImageResourceDirectoryEntry| dir.name == RESOURCE_TYPE_STRING);
    match string_directory {
        Some(dir) => Ok(dir),
        None => Err(ErrorKind::StringResourcesNotFound.into()),
    }
}

fn read_pe_string_resource_entries<S: Read + Seek>
    (stream: &mut S,
     resource_root_offset: u64,
     string_directory: &PeImageResourceDirectoryEntry)
     -> Result<BTreeMap<u32, Vec<PeImageResourceDirectoryEntry>>> {
    // Seek to the string sub directory
    let string_subdir_rel_offset = string_directory.data_offset as u64;
    try!(stream.seek(SeekFrom::Start(resource_root_offset + string_subdir_rel_offset)));

    let mut string_entry_map: BTreeMap<u32, Vec<PeImageResourceDirectoryEntry>> = BTreeMap::new();
    let sub_directories = try!(read_pe_resource_directories(stream));
    for sub_dir in &sub_directories.entries {
        // Seek to the sub directory's data
        try!(stream.seek(SeekFrom::Start(resource_root_offset + sub_dir.data_offset as u64)));
        let language_dir_entries = try!(read_pe_resource_directories(stream));
        if string_entry_map.get(&sub_dir.name).is_some() {
            string_entry_map.get_mut(&sub_dir.name).unwrap().extend(language_dir_entries.entries);
        } else {
            let mut entries = Vec::new();
            entries.extend(language_dir_entries.entries);
            string_entry_map.insert(sub_dir.name, entries);
        }
    }
    Ok(string_entry_map)
}

fn read_pe_resource_data_entry<S: Read + Seek>(stream: &mut S,
                                               resource_root_offset: u64,
                                               string_entry: &PeImageResourceDirectoryEntry)
                                               -> Result<PeImageResourceDataEntry> {
    try!(stream.seek(SeekFrom::Start(resource_root_offset + string_entry.data_offset as u64)));
    Ok(PeImageResourceDataEntry {
        data_offset: try!(stream.read_u32()),
        size: try!(stream.read_u32()),
        code_page: try!(stream.read_u32()),
        reserved: try!(stream.read_u32()),
    })
}
