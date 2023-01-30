use std::io;

use super::{Serializeable, Deserializeable};

#[allow(unused)]
pub struct VarInt {
    value: i32,
    bytes: Vec<u8>
}

impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        VarInt { value, bytes: to_varint(value) }
    }
}

#[allow(unused)]
impl VarInt {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, io::Error> {
        let (value, bytes) = from_varint_bytes(bytes)?;

        Ok(VarInt{ value, bytes: bytes.to_vec() })
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn bytes(&self) -> &[u8] {
        return &self.bytes
    }

    pub fn len(&self) -> usize {
        return self.bytes.len()
    }
}

impl Serializeable for VarInt {
    fn mc_serialize(&self) -> Result<Vec<u8>, io::Error> {
        Ok(self.bytes.to_vec())
    }
}

impl Deserializeable for VarInt {
    fn mc_deserialize(bytes: &[u8]) -> Result<Self, io::Error> {
        VarInt::from_bytes(bytes)
    }
}

fn from_varint_bytes(bytes: &[u8]) -> Result<(i32, &[u8]), io::Error> {
    let mut value = 0;
    let mut pos = 0;
    let mut end_idx = 0;

    const SEGMENT_BITS: i32 = 0x7F;
    const CONTINUE_BIT: i32 = 0x80;

    for b in bytes.iter() {
        value |= ((*b as i32) & SEGMENT_BITS) << pos;
        end_idx += 1;

        if (*b as i32) & CONTINUE_BIT == 0 {
            break;
        }

        pos += 7;

        if pos >= 32 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "VarInt descriptor exceeds >5 bytes"
            ));
        }
    }

    Ok((value, &bytes[..end_idx]))
}

fn to_varint(mut value: i32) -> Vec<u8> {
    let mut bytes = Vec::<u8>::new();

    const SEGMENT_BITS: i32 = 0x7F;
    const CONTINUE_BIT: i32 = 0x80;

    loop {
        if (value & !SEGMENT_BITS) == 0 {
            bytes.push(value as u8);
            break;
        }

        bytes.push(((value & SEGMENT_BITS) | CONTINUE_BIT) as u8);

        // https://stackoverflow.com/a/70212287
        value = ((value as u32) >> 7) as i32;
    }

    bytes
}
