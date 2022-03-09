use std::{net::TcpStream, io::{Write, Read}};

use anyhow::Result;

use crate::load::{AppContext, app::App};

pub struct Time {

}

impl App for Time {
    fn init(self: &Self, _ctx: &AppContext) -> Result<()> {
        println!("hello time");
        let mut stream = TcpStream::connect("36.152.44.96:80")?;

        // let err = stream.try_clone();
        // if let Err(err) = err {
        //     println!(
        //         "Duplication of file descriptors does not work (yet) on the ESP-IDF, as expected: {}",
        //         err
        //     );
        // }

        stream.write_all("GET / HTTP/1.0\n\n".as_bytes())?;

        let mut result = Vec::new();

        stream.read_to_end(&mut result)?;

        println!(
            "ntp returned:\n=================\n{}\n=================\nSince it returned something, all is OK",
            std::str::from_utf8(&result)?);
        println!("hello time end");
        Ok(())
    }

    fn run(self: &Self, _ctx: &AppContext) -> Result<()> {
        Ok(())
    }
}