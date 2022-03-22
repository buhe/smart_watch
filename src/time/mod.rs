// use std::{net::{TcpStream, UdpSocket}, io::{Write, Read}};



use std::time::Instant;

use anyhow::Result;
use ntp::protocol::Packet;
// use time::{Date, Month};
use time::macros::{date, time};
use time::{ Duration, PrimitiveDateTime};
use time::ext::{NumericalDuration};

use crate::load::{AppContext, app::App};
use crate::target::Target;

pub struct Time {
    pub r: Option<PrimitiveDateTime>,
    pub count: Option<Instant>,
}

impl App for Time {
    fn init(self: &mut Self, _ctx: &mut AppContext) -> Result<()> {
        println!("hello time");
        let address = "0.cn.pool.ntp.org:123";
        let response: Packet = ntp::request(address).unwrap();
        let ntp_time = response.receive_timestamp;
        println!("{:?}", ntp_time.to_owned());
        let d = date!(1900-01-01).with_time(time!(0:00));
        let s = Duration::seconds(ntp_time.seconds.into());
        self.r = Some(d + s + 8.hours());
        self.count = Some(Instant::now());
        println!("hello time {} {:?}", self.r.unwrap(), self.count.unwrap());
        Ok(())
    }

    fn run(self: &mut Self, _ctx: &mut AppContext, ts: &mut Vec<Target>) -> Result<()> {
        
        // println!("time is {}", self.r.unwrap() + self.count.unwrap().elapsed());
        Ok(())
    }
}