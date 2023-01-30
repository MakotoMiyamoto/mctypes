#[allow(unused_imports)]
use std::{net::TcpStream, io::{self, BufWriter, BufReader, Write}};

use types::{varint::VarInt};

mod types;

#[allow(unused)]
fn main() -> Result<(), io::Error> {
    let mut stream = TcpStream::connect("localhost:25565")?;

    let mut writer = BufWriter::new(stream.try_clone()?);
    let mut reader = BufReader::new(&mut stream);

    // let packet_len = VarInt::from(0);
    // let packet_id = VarInt::from(0);
    // writer.write(packet_len.bytes())?;
    // writer.write(packet_id.bytes())?;

    // let proto_vers = VarInt::from(760); // 1.19.2 protocol version
    // let host = "localhost".to_owned();
    // let next_state = VarInt::from(2); // Login state
    
    // writer.write(proto_vers.bytes())?;
    // writer.write(VarInt::from(host.len() as i32).bytes())?;
    // writer.write(host.as_bytes())?;
    // writer.write(25565_u16.to_be_bytes().as_slice())?;
    // writer.write(next_state.bytes())?;

    let host = "localhost".to_owned();

    let mut handshake: Vec<u8> = vec![];
    handshake.push(0x00); // packet id for handshake
    handshake.extend(VarInt::from(760).bytes()); // protocol version for 1.19.2
    handshake.extend(VarInt::from(host.len() as i32).bytes()); // length of host str
    handshake.extend(host.as_bytes()); // host str
    handshake.extend(25565_u16.to_be_bytes()); // port
    handshake.extend(VarInt::from(0x02).bytes()); // next state (login)

    let mut handshake_out: Vec<u8> = vec![];
    handshake_out.extend(VarInt::from(handshake.len() as i32).bytes());
    handshake_out.extend(&handshake);

    println!("{:?}", handshake_out);


    Ok(())
}
