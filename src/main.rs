use std::io::Read;
#[allow(unused_imports)]
use std::{net::TcpStream, io::{self, BufWriter, BufReader, Write}};

use types::{varint::VarInt};

mod types;

#[allow(unused)]
fn main() -> Result<(), io::Error> {
    let mut stream = TcpStream::connect("localhost:25565")?;

    let mut writer = BufWriter::new(stream.try_clone()?);
    let mut reader = BufReader::new(&mut stream);

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

    let mut login_start: Vec<u8> = vec![];
    let username: String = "Makoto".to_owned();
    login_start.push(0x00); // packet id for login start
    login_start.extend(VarInt::from(username.len() as i32).bytes());
    login_start.extend(username.as_bytes());
    login_start.push(0x00); // Do not send UUID.

    let mut login_start_out: Vec<u8> = vec![];
    login_start_out.extend(VarInt::from(login_start.len() as i32).bytes());
    login_start_out.extend(&login_start);

    println!("{:?}", login_start_out);

    writer.write_all(&handshake_out)?;
    writer.write_all(&login_start_out)?;
    writer.flush()?;

    let mut header_buf: [u8; 6] = [0; 6];
    reader.read(&mut header_buf)?;
    let rx_len = VarInt::from_bytes(&header_buf)?;
    let rx_id = VarInt::from_bytes(&header_buf[rx_len.len()..])?;
    println!("{:?} {:?}", rx_len.value(), rx_id.value());

    Ok(())
}
