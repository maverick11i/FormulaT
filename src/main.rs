use serde::Deserialize;
use std::io::Cursor;
use std::net::UdpSocket;

fn main() -> Result<(), Box<bincode::ErrorKind>> {
    let socket = UdpSocket::bind("0.0.0.0:20777")?;
    println!("Listening on {}", socket.local_addr()?);

    let mut buf = [0u8; 2048];
    loop {
        match socket.recv(&mut buf) {
            Ok(size) => {
                let mut packet = Cursor::new(&buf);
                let header: Header = bincode::deserialize_from(&mut packet)?;

                println!("buf-size: {:?}", size);
                println!("buf-packet{:?}", header);
            }
            Err(e) => {
                println!("couldn't receive request: {:?}", e);
            }
        }
    }
}

#[derive(Deserialize, Debug)]
struct Header {
    packet_format: u16,
    game_major_version: u8,
    game_minor_version: u8,
    packet_version: u8,
    packet_id: u8,
    session_uid: u64,
    session_time: f32,
    frame_identifier: u32,
    player_car_index: u8,
}
