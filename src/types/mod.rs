use std::io;

use self::varint::VarInt;

pub mod varint;

pub trait Serializeable {
    fn mc_serialize(&self) -> Result<Vec<u8>, io::Error>;
}

pub trait Deserializeable where Self: Sized {
    fn mc_deserialize(bytes: &[u8]) -> Result<Self, io::Error>;
}

impl Serializeable for String {
    fn mc_serialize(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes: Vec<u8> = vec![];

        let len: i32 = match self.len().try_into() {
            Ok(i) => Ok(i),
            Err(_) => Err(
                io::Error::new(
                    io::ErrorKind::InvalidData, 
                    format!("String size larger than {}.", i32::MAX)))
        }?;

        bytes.extend(VarInt::from(len).bytes());
        bytes.extend(self.as_bytes());
        
        Ok(bytes)
    }
}

impl Deserializeable for String {
    fn mc_deserialize(bytes: &[u8]) -> Result<Self, io::Error> {
        let len = VarInt::from_bytes(bytes)?;

        let s: String = match String::from_utf8(bytes[len.len()..len.value() as usize].to_vec()) {
            Ok(s) => Ok(s),
            Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
        }?;
        
        Ok(s)
    }
}
