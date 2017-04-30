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

use chariot_io_tools::*;
use std::fmt;

use std::io::prelude::*;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum ResourceType {
    Food,
    Wood,
    Stone,
    Gold,
    Unknown(i16),
}

impl ResourceType {
    pub fn from_i16(val: i16) -> ResourceType {
        use self::ResourceType::*;
        match val {
            0 => Food,
            1 => Wood,
            2 => Stone,
            3 => Gold,
            _ => Unknown(val),
        }
    }
}

impl Default for ResourceType {
    fn default() -> ResourceType {
        ResourceType::Unknown(-1)
    }
}

pub trait ReadResourceCost {
    fn read_resource_cost(&mut self, stream: &mut Read) -> Result<()>;
}

#[derive(Default, Clone, Copy)]
pub struct ResourceCost<T: Copy, E: Copy> {
    pub resource_type: ResourceType,
    pub amount: T,
    pub enabled: bool,
    phantom: PhantomData<E>,
}

impl<T: Copy + fmt::Display, E: Copy> fmt::Debug for ResourceCost<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "ResourceCost {{ resource_type: {:?}, amount: {}, enabled: {} }}",
               self.resource_type,
               self.amount,
               self.enabled)
    }
}

impl ReadResourceCost for ResourceCost<i16, u8> {
    fn read_resource_cost(&mut self, mut stream: &mut Read) -> Result<()> {
        self.resource_type = ResourceType::from_i16(try!(stream.read_i16()));
        self.amount = try!(stream.read_i16());
        self.enabled = try!(stream.read_u8()) != 0;
        Ok(())
    }
}

impl ReadResourceCost for ResourceCost<i16, i16> {
    fn read_resource_cost(&mut self, mut stream: &mut Read) -> Result<()> {
        self.resource_type = ResourceType::from_i16(try!(stream.read_i16()));
        self.amount = try!(stream.read_i16());
        self.enabled = try!(stream.read_i16()) != 0;
        Ok(())
    }
}

impl ReadResourceCost for ResourceCost<f32, u8> {
    fn read_resource_cost(&mut self, mut stream: &mut Read) -> Result<()> {
        self.resource_type = ResourceType::from_i16(try!(stream.read_i16()));
        self.amount = try!(stream.read_f32());
        self.enabled = try!(stream.read_u8()) != 0;
        Ok(())
    }
}

#[macro_export]
macro_rules! read_resource_costs {
    ($t:ty, $e:ty, $stream:expr, $num:expr) => {
        {
            let mut result = Vec::new();
            for _ in 0..$num {
                let mut cost: ResourceCost<$t, $e> = Default::default();
                try!(cost.read_resource_cost($stream));
                if cost.enabled {
                    result.push(cost);
                }
            }
            result
        }
    }
}
