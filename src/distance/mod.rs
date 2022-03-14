use std::time::Instant;

use anyhow::Result;
use embedded_svc::io::Bytes;
use embedded_svc::http::{client::*, SendHeaders};
use esp_idf_hal::gpio::{Gpio22, Unknown, Gpio21};
// use embedded_hal::digital::v2::OutputPin;
use esp_idf_hal::prelude::*;
use esp_idf_hal::i2c::{self, I2C0, Master};
use vl53l0x::VL53L0x;

use crate::load::{AppContext, app::App};

const URL: &str = "https://jcxivsbsjuqmeafnwuwk.supabase.co/rest/v1/onoff?id=eq.2&select=en";

pub struct Distance {
    pub count: Option<Instant>,
    pub cond: u64,
    pub tof: Option<VL53L0x<Master<I2C0, Gpio22<Unknown>, Gpio21<Unknown>>>>,
}

impl App for Distance {
    fn init(self: &mut Self, ctx: &mut AppContext) -> Result<()> {
        self.count = Some(Instant::now());
        println!("About to initialize a generic SSD1306 I2C LED driver");

    let config = <i2c::config::MasterConfig as Default>::default().baudrate(1.MHz().into());
    self.tof = Some(vl53l0x::VL53L0x::new(i2c::Master::<i2c::I2C0, _, _>::new(
        ctx.i2c0.take().unwrap(),
        i2c::MasterPins { sda: ctx.gpio22.take().unwrap(), scl: ctx.gpio21.take().unwrap() },
        config,
    )?).unwrap());

     println!("ready to set meas budget");
    self.tof.as_mut().unwrap().set_measurement_timing_budget(200000).expect("timbudg");
        Ok(())
    }

    fn run(self: &mut Self, ctx: &mut AppContext) -> Result<()> {
        let e = self.count.unwrap().elapsed().as_secs();
         if e % 4 == 0 && e != self.cond {
            println!("at 4s {:?}", e);
            self.cond = e;
            let response = ctx.http.get(URL)?
            .header("apikey", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImpjeGl2c2JzanVxbWVhZm53dXdrIiwicm9sZSI6ImFub24iLCJpYXQiOjE2NDcwNjYwOTEsImV4cCI6MTk2MjY0MjA5MX0.YP7o3MKM7sxsNioyuVuVqTIgdgJbKz638njLOnT9DRA")
            .header("Authorization", "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImpjeGl2c2JzanVxbWVhZm53dXdrIiwicm9sZSI6ImFub24iLCJpYXQiOjE2NDcwNjYwOTEsImV4cCI6MTk2MjY0MjA5MX0.YP7o3MKM7sxsNioyuVuVqTIgdgJbKz638njLOnT9DRA").submit()?;

            let body: Result<Vec<u8>, _> = Bytes::<_, 64>::new(response.reader()).collect();
            let body = body?;
            let str = String::from_utf8_lossy(&body).into_owned();
            if str == "[{\"en\":0}]" {
                // let gpio26 = &ctx.peripherals.pins.gpio26;
            } else {
                self.tof.as_mut().unwrap().start_continuous(0).expect("start cont");
                // ctx.peripherals.pins.gpio26.into_output().unwrap().set_high()?;
            }
        }
        Ok(())
    }
}