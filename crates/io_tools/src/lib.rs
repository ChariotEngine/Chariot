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

extern crate byteorder;
extern crate flate2;

use byteorder::LittleEndian;
use byteorder::ReadBytesExt;

use std::io;
use std::io::BufRead;

/// All of these methods assume little endian
pub trait ReadExt {
    /// Read and return exactly one byte from the stream
    fn read_u8(&mut self) -> io::Result<u8>;
    fn read_i8(&mut self) -> io::Result<i8>;

    fn read_u16(&mut self) -> io::Result<u16>;
    fn read_i16(&mut self) -> io::Result<i16>;
    fn read_u32(&mut self) -> io::Result<u32>;
    fn read_i32(&mut self) -> io::Result<i32>;
    fn read_f32(&mut self) -> io::Result<f32>;

    /// Read in the desired amount of bytes and convert it into a string.
    /// Assumes the slice is either null terminated, or that the string contents
    /// occupy the full width.
    fn read_sized_str(&mut self, len: usize) -> io::Result<String>;

    /// Read in zlib compressed data for the remainder of the stream
    fn read_and_decompress(self) -> io::Result<Vec<u8>>;
}

pub trait ReadArrayExt<T: Sized, S: io::Read, E, F: Fn(&mut S) -> Result<T, E>> {
    fn read_array(&mut self, count: usize, read_method: F) -> Result<Vec<T>, E>;
}

impl<T, S, E, F> ReadArrayExt<T, S, E, F> for S
    where T: Sized,
          S: io::Read,
          F: Fn(&mut S) -> Result<T, E>
{
    fn read_array(&mut self, count: usize, read_method: F) -> Result<Vec<T>, E> {
        let mut result: Vec<T> = Vec::new();
        for _ in 0..count {
            result.push(try!(read_method(self)));
        }
        Ok(result)
    }
}

const DECOMPRESSION_CHUNK_SIZE: usize = 16 * 1024; // 16 kibibytes

impl<T> ReadExt for T
    where T: io::Read
{
    fn read_u8(&mut self) -> io::Result<u8> {
        ReadBytesExt::read_u8(self)
    }

    fn read_i8(&mut self) -> io::Result<i8> {
        ReadBytesExt::read_i8(self)
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

    fn read_and_decompress(mut self) -> io::Result<Vec<u8>> {
        use flate2::Status;
        use flate2::Decompress;
        use flate2::Flush;

        let mut compressed: Vec<u8> = Vec::new();
        try!(self.read_to_end(&mut compressed));

        let mut stream = io::Cursor::new(&compressed[..]);

        // At time of implementation, the flate2 library didn't provide an easy way to
        // decompress a stream without a header, so it had to be manually implemented here
        let mut decompressed: Vec<u8> = Vec::new();
        let mut buffer = [0u8; DECOMPRESSION_CHUNK_SIZE];
        let mut decompressor = Decompress::new(false);
        loop {
            let last_out = decompressor.total_out();
            let last_in = decompressor.total_in();

            let (status, end_stream);
            {
                let input = try!(stream.fill_buf());
                end_stream = input.is_empty();

                let flush_type = if end_stream { Flush::Finish } else { Flush::None };
                status = try!(decompressor.decompress(input, &mut buffer, flush_type)
                    .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "failed to decompress")));
            }

            let read = (decompressor.total_in() - last_in) as usize;
            let written = (decompressor.total_out() - last_out) as usize;

            decompressed.extend_from_slice(&buffer[0..written]);
            stream.consume(read);

            match status {
                Status::Ok => {}
                Status::BufError if !end_stream && written == 0 => continue,
                Status::BufError | Status::StreamEnd => break,
            }
        }

        Ok(decompressed)
    }
}

#[test]
fn test_read_byte() {
    let mut cursor = io::Cursor::new("test".as_bytes());
    assert_eq!('t' as u8, ReadExt::read_u8(&mut cursor).unwrap());
    assert_eq!('e' as u8, ReadExt::read_u8(&mut cursor).unwrap());
    assert_eq!('s' as u8, ReadExt::read_u8(&mut cursor).unwrap());
    assert_eq!('t' as u8, ReadExt::read_u8(&mut cursor).unwrap());
}

#[test]
fn test_read_sized_str() {
    let data = "test\0\0\0\0".as_bytes();
    assert_eq!("test".to_string(),
               io::Cursor::new(data).read_sized_str(8).unwrap());
    assert_eq!("test".to_string(),
               io::Cursor::new(data).read_sized_str(4).unwrap());
    assert_eq!("te".to_string(),
               io::Cursor::new(data).read_sized_str(2).unwrap());
    assert_eq!("".to_string(),
               io::Cursor::new(data).read_sized_str(0).unwrap());
    assert!(io::Cursor::new(data).read_sized_str(9).is_err());
}
