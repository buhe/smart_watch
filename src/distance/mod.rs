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
use crate::target::Target;

const URL: &str = "https://jcxivsbsjuqmeafnwuwk.supabase.co/rest/v1/onoff?id=eq.2&select=en";
pub enum State {
    Running,
    Stopped,
}
pub struct Distance {
    pub state: State,
    pub count: Option<Instant>,
    pub cond: u64,
    pub tof: Option<VL53L0x<Master<I2C0, Gpio21<Unknown>, Gpio22<Unknown>>>>,
}

impl App for Distance {
    fn init(self: &mut Self, ctx: &mut AppContext) -> Result<()> {
        self.count = Some(Instant::now());
        println!("About to initialize a generic I2C driver");

        let config = <i2c::config::MasterConfig as Default>::default().baudrate(400.kHz().into());
        self.tof = Some(vl53l0x::VL53L0x::new(i2c::Master::<i2c::I2C0, _, _>::new(
            ctx.i2c0.take().unwrap(),
            i2c::MasterPins { sda: ctx.gpio21.take().unwrap(), scl: ctx.gpio22.take().unwrap() },
            config,
        )?).unwrap());

        println!("ready to set meas budget");
        self.tof.as_mut().unwrap().set_measurement_timing_budget(200000).expect("timbudg");
        Ok(())
    }

    fn run(self: &mut Self, ctx: &mut AppContext, ts: &mut Vec<Target>) -> Result<()> {
        let e = self.count.unwrap().elapsed().as_secs();
         if e % 4 == 0 && e != self.cond {
            self.cond = e;
            let response = ctx.http.get(URL)?
            .header("apikey", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImpjeGl2c2JzanVxbWVhZm53dXdrIiwicm9sZSI6ImFub24iLCJpYXQiOjE2NDcwNjYwOTEsImV4cCI6MTk2MjY0MjA5MX0.YP7o3MKM7sxsNioyuVuVqTIgdgJbKz638njLOnT9DRA")
            .header("Authorization", "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImpjeGl2c2JzanVxbWVhZm53dXdrIiwicm9sZSI6ImFub24iLCJpYXQiOjE2NDcwNjYwOTEsImV4cCI6MTk2MjY0MjA5MX0.YP7o3MKM7sxsNioyuVuVqTIgdgJbKz638njLOnT9DRA").submit()?;

            let body: Result<Vec<u8>, _> = Bytes::<_, 64>::new(response.reader()).collect();
            let body = body?;
            let str = String::from_utf8_lossy(&body).into_owned();
            if str == "[{\"en\":0}]" {
                println!("stop distance");
                match self.state {
                    State::Running => {
                        let l = self.tof.as_mut().unwrap();
                        l.stop_continuous().unwrap();   
                        self.state = State::Stopped;
                    },
                    State::Stopped => {},
                }   
            } else {
                println!("start distance");
                match self.state {
                    State::Running => {
                        match self.tof.as_mut().unwrap().read_range_continuous_millimeters_blocking() {
                            Ok(meas) => {
                                println!("vl: millis {}", meas);
                            }
                            Err(e) => {
                                println!("Err meas: {:?}\r\n", e);
                            }
                        };
                    },
                    State::Stopped => {
                        self.tof.as_mut().unwrap().start_continuous(0).expect("start cont");
                        self.state = State::Running;
                    },
                } 
                
            }
        }
        Ok(())
    }
}