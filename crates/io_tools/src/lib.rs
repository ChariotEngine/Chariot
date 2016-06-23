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

extern crate byteorder;

use byteorder::LittleEndian;
use byteorder::ReadBytesExt;

use std::io;

/// All of these methods assume little endian
pub trait ReadExt {
    /// Read and return exactly one byte from the stream
    fn read_byte(&mut self) -> io::Result<u8>;

    fn read_u16(&mut self) -> io::Result<u16>;
    fn read_i16(&mut self) -> io::Result<i16>;
    fn read_u32(&mut self) -> io::Result<u32>;
    fn read_i32(&mut self) -> io::Result<i32>;
    fn read_f32(&mut self) -> io::Result<f32>;

    /// Read in the desired amount of bytes and convert it into a string.
    /// Assumes the slice is either null terminated, or that the string contents
    /// occupy the full width.
    fn read_sized_str(&mut self, len: usize) -> io::Result<String>;
}

impl<T> ReadExt for T where T: io::Read {
    fn read_byte(&mut self) -> io::Result<u8> {
        let mut buffer = [0u8; 1];
        try!(self.read_exact(&mut buffer));
        Ok(buffer[0])
    }

    fn read_u16(&mut self) -> io::Result<u16> {
        ReadBytesExt::read_u16::<LittleEndian>(self)
    }

    fn read_i16(&mut self) -> io::Result<i16> {
        ReadBytesExt::read_i16::<LittleEndian>(self)
    }

    fn read_u32(&mut self) -> io::Result<u32> {
        ReadBytesExt::read_u32::<LittleEndian>(self)
    }

    fn read_i32(&mut self) -> io::Result<i32> {
        ReadBytesExt::read_i32::<LittleEndian>(self)
    }

    fn read_f32(&mut self) -> io::Result<f32> {
        ReadBytesExt::read_f32::<LittleEndian>(self)
    }

    fn read_sized_str(&mut self, len: usize) -> io::Result<String> {
        let mut buffer = vec![0u8; len];
        try!(self.read_exact(&mut buffer));

        // Seems like there should be a better way to do this
        // Find the null terminator since Rust strings are not null-terminated
        let mut null_term = buffer.len();
        for i in 0..buffer.len() {
            if buffer[i] == 0 {
                null_term = i;
                break;
            }
        }
        buffer.resize(null_term, 0u8);
        Ok(try!(String::from_utf8(buffer)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "bad encoding"))))
    }
}

#[test]
fn test_read_byte() {
    let mut cursor = io::Cursor::new("test".as_bytes());
    assert_eq!('t' as u8, cursor.read_byte().unwrap());
    assert_eq!('e' as u8, cursor.read_byte().unwrap());
    assert_eq!('s' as u8, cursor.read_byte().unwrap());
    assert_eq!('t' as u8, cursor.read_byte().unwrap());
}

#[test]
fn test_read_sized_str() {
    let data = "test\0\0\0\0".as_bytes();
    assert_eq!("test".to_string(), io::Cursor::new(data).read_sized_str(8).unwrap());
    assert_eq!("test".to_string(), io::Cursor::new(data).read_sized_str(4).unwrap());
    assert_eq!("te".to_string(), io::Cursor::new(data).read_sized_str(2).unwrap());
}
