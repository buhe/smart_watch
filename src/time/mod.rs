// use std::{net::{TcpStream, UdpSocket}, io::{Write, Read}};

use anyhow::Result;
use ntp::protocol::Packet;

use crate::load::{AppContext, app::App};

pub struct Time {

}

impl App for Time {
    fn init(self: &Self, _ctx: &AppContext) -> Result<()> {
        println!("hello time");
            let address = "0.pool.ntp.org:123";
    let response: Packet = ntp::request(address).unwrap();
    let ntp_time = response.transmit_timestamp;
    println!("{:?}", ntp_time.to_owned());
        // let socket = UdpSocket::bind("127.0.0.1:123")?;
        // socket.send_to(buf, "119.28.183.184:123")?;
        // // Receives a single datagram message on the socket. If `buf` is too small to hold
        // // the message, it will be cut off.
        // let mut buf = [0; 10];
        // let (amt, src) = socket.recv_from(&mut buf)?;

        // Redeclare `buf` as slice of the received data and send reverse data back to origin.
        // let buf = &mut buf[..amt];
        // buf.reverse();
        
        // let mut stream = TcpStream::connect("119.28.183.184:123")?;

        // let err = stream.try_clone();
        // if let Err(err) = err {
        //     println!(
        //         "Duplication of file descriptors does not work (yet) on the ESP-IDF, as expected: {}",
        //         err
        //     );
        // }

        // stream.write_all("GET / HTTP/1.0\n\n".as_bytes())?;

        // let mut result = Vec::new();

        // stream.read_to_end(&mut result)?;

        // println!(
        //     "ntp returned:\n=================\n{}\n=================\nSince it returned something, all is OK",
        //     std::str::from_utf8(&result)?);
        println!("hello time end");
        Ok(())
    }

    fn run(self: &Self, _ctx: &AppContext) -> Result<()> {
        Ok(())
    }
}